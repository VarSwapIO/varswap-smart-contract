use gstd::exec;
use sails_rs::calls::{Call, Query};
use sails_rs::gstd::calls::GStdRemoting;
use sails_rs::{
    prelude::*,
    gstd::msg
};
use crate::clients::extended_new_vft::traits::Vft;
use crate::clients::extended_new_vft::Vft as VftClient;
use crate::clients::factory_vara_dex_client::traits::FactoryService;
use crate::clients::factory_vara_dex_client::FactoryService as FactoryServiceClient;
use crate::states::lp_state::{LPError, LPEvent, StateLp, LP, MINIMUM_LIQUIDITY};
use vft_service::{Service as VftService, Storage};

pub struct LPService{
    pub vft_client:VftClient<GStdRemoting>,
    pub vft_service:VftService,
    pub factory_client:FactoryServiceClient<GStdRemoting>,

}

impl LPService {
    pub fn seed(factory:ActorId, token_a:ActorId, token_b:ActorId, name:String, symbol:String, decimals:u8) -> Self {
        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };
        unsafe {
            LP = Some(StateLp {
                name:name.clone(),
                symbol:symbol.clone(),
                decimals,
                factory,
                token: token_pair,
                ..Default::default()
            });
        }
        LPService {
            vft_service: <VftService>::seed(name, symbol, decimals),
            vft_client: VftClient::new(GStdRemoting),
            factory_client: FactoryServiceClient::new(GStdRemoting),
        }
    }
}

#[service(extends = VftService,events = LPEvent)]
impl LPService {

    pub fn new(vft_client:VftClient<GStdRemoting>, factory_client:FactoryServiceClient<GStdRemoting>) -> Self {
        Self {
            vft_client,
            vft_service:VftService::new(),
            factory_client,
        }
    }

    //private function
    fn _update(&mut self, balance:(U256, U256), reverse:(U256, U256)) -> Result<(), LPError> {
        let state_lp = StateLp::get_mut();
        if balance.0 > U256::MAX || balance.1 > U256::MAX {
            return Err(LPError::Overflow);
        }
        let block_timestamp = exec::block_timestamp() % 2u64.pow(32);
        let time_elapsed = block_timestamp.saturating_sub(state_lp.last_block_ts);
        if time_elapsed > 0 && reverse.0 != U256::zero() && reverse.1 != U256::zero() {
            state_lp.cumulative_price.0 += (reverse.1 / reverse.0) * U256::from(time_elapsed);
            state_lp.cumulative_price.1 += (reverse.0 / reverse.1) * U256::from(time_elapsed);
        }
        state_lp.reserve.0 = balance.0;
        state_lp.reserve.1 = balance.1;
        state_lp.last_block_ts = block_timestamp;
        Ok(())
    }

    fn _mint(&mut self, to:ActorId, liquidity:U256) {
        let old_balance = self.vft_service.balance_of(to);
        let new_balance = old_balance.checked_add(liquidity).unwrap();
        let storage_balance = Storage::balances();
        storage_balance.insert(to, new_balance);
        //update total supply
        let total_supply = Storage::total_supply();
        *total_supply =  total_supply.checked_add(liquidity).unwrap();
    }

    fn _burn(&mut self, from:ActorId, liquidity:U256) {
        let old_balance = self.vft_service.balance_of(from);
        let new_balance = old_balance.checked_sub(liquidity).unwrap();
        let storage_balance = Storage::balances();
        if !new_balance.is_zero() {
            storage_balance.insert(from, new_balance);
        }else {
            storage_balance.remove(&from);
        };
        let total_supply = Storage::total_supply();
        *total_supply =  total_supply.checked_sub(liquidity).unwrap();
    }

    async fn _mint_fee(&mut self, reserve_0:U256, reserve_1:U256) -> Result<bool, LPError>{
        let state_lp = StateLp::get_mut();
        let fee_to_res = self.factory_client.get_fee_to().recv(state_lp.factory).await;
        let Ok(fee_to) = fee_to_res else {
            return Err(LPError::CanNotConnectToFactory);
        };
        let _k_last = state_lp.k_last;
        let fee_on = if fee_to != ActorId::zero() {
            true
        } else {
            false
        };
        if fee_on {
            if _k_last != U256::zero() {
               let root_k = (reserve_0 * reserve_1).integer_sqrt();
               let root_klast = _k_last.integer_sqrt();
               if root_k > root_klast {
                let numerator = Storage::total_supply().checked_mul(root_k.checked_sub(root_klast).unwrap()).unwrap();
                let denominator = root_k.checked_mul(U256::from(5)).unwrap();
                let liquidity = numerator.checked_div(denominator).unwrap();
                if liquidity > U256::zero() {
                    self._mint(fee_to, liquidity);
                }
               }
            }
        }else if _k_last == U256::zero() {
            state_lp.k_last = U256::zero();
        }
        Ok(fee_on)
    }

    async fn _safe_transfer(&mut self, token: ActorId, to: ActorId, value: U256) -> Result<(), LPError> {
        let transfer_res = self.vft_client.transfer(to, value).send_recv(token).await;
        let Ok(transfer_status) = transfer_res else {
            return Err(LPError::TransferFailed);
        };
        if !transfer_status {
            return Err(LPError::TransferFailed);
        }
        Ok(())
    }

    pub async fn mint(&mut self, to:ActorId) ->  Result<U256, LPError>{
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get_mut();
        let token_pair = state_lp.token.clone();
        let balance_0_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.0.clone()).await;
        let Ok(balance_0) = balance_0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance_1_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.1.clone()).await;
        let Ok(balance_1) = balance_1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let amount_0 = balance_0.checked_sub(reserve0).unwrap();
        let amount_1 = balance_1.checked_sub(reserve1).unwrap();
        
         let fee_on = self._mint_fee(reserve0, reserve1).await?;
         let total_supply = Storage::total_supply().clone();
         let liquidity = if total_supply == U256::zero() {
            let mint_amount = (amount_0 * amount_1).integer_sqrt() - U256::from(MINIMUM_LIQUIDITY);
            self._mint(ActorId::zero(), U256::from(MINIMUM_LIQUIDITY));
            mint_amount
         }else {
            gstd::cmp::min(
                amount_0 * total_supply/reserve0, 
                amount_1 * total_supply/reserve1)
         };
         if liquidity <= U256::zero() {
            return Err(LPError::InsufficientLiquidityMinted);
         };
         self._mint(to, liquidity.clone());
         let _ = self._update((balance_0,balance_1), (reserve0, reserve1));
         if fee_on {
            state_lp.k_last = state_lp.reserve.0.checked_mul(state_lp.reserve.1).unwrap();
         }
         self.notify_on(LPEvent::Mint { sender: msg::source(), amount: (amount_0,amount_1) }).unwrap();
         Ok(liquidity)
            
    }

    pub async fn burn(&mut self, to: ActorId) -> Result<(U256, U256), LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get_mut();
        let token_pair = state_lp.token.clone();
        let balance0_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.0.clone()).await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.1.clone()).await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let liquidity = self.vft_service.balance_of(exec::program_id());

        let fee_on = self._mint_fee(reserve0, reserve1).await?;
        let total_supply = self.vft_service.total_supply().clone();

        let amount0 = liquidity * balance0 / total_supply;
        let amount1 = liquidity * balance1 / total_supply;

        if amount0 == U256::zero() || amount1 == U256::zero() {
            return Err(LPError::InsufficientLiquidityBurned);
        }
        self._burn(exec::program_id(), liquidity);
        self._safe_transfer(token_pair.0.clone(), to, amount0).await?;
        self._safe_transfer(token_pair.1.clone(), to, amount1).await?;
        let balance0_after_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.0.clone()).await;
        let Ok(balance0_after) = balance0_after_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_after_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.1.clone()).await;
        let Ok(balance1_after) = balance1_after_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let _ = self._update((balance0_after, balance1_after), (reserve0, reserve1));
        if fee_on {
            state_lp.k_last = state_lp.reserve.0.checked_mul(state_lp.reserve.1).unwrap();
        }
        self.notify_on(LPEvent::Burn {
            sender: msg::source(),
            amount: (amount0, amount1),
            to,
        }).unwrap();

        Ok((amount0, amount1))
    }

    pub async fn swap(&mut self, amount0_out: U256, amount1_out: U256, to: ActorId) -> Result<(), LPError> {
        if amount0_out == U256::zero() && amount1_out == U256::zero() {
            return Err(LPError::InsufficientOutputAmount);
        }

        let (reserve0, reserve1, _) = self.get_reserves();
        if amount0_out >= reserve0 || amount1_out >= reserve1 {
            return Err(LPError::InsufficientLiquidity);
        }
        let state_lp = StateLp::get();
        let token_pair = state_lp.token.clone();

        if to == token_pair.0 || to == token_pair.1 {
            return Err(LPError::InvalidTo);
        }
        if amount0_out > U256::zero() {
            self._safe_transfer(token_pair.0, to, amount0_out).await?;
        }
        if amount1_out > U256::zero() {
            self._safe_transfer(token_pair.1, to, amount1_out).await?;
        }
        let balance0_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.0.clone()).await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.1.clone()).await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let amount0_in = if balance0 > reserve0 - amount0_out { balance0 - (reserve0 - amount0_out) } else { U256::zero() };
        let amount1_in = if balance1 > reserve1 - amount1_out { balance1 - (reserve1 - amount1_out) } else { U256::zero() };
        if amount0_in == U256::zero() && amount1_in == U256::zero() {
            return Err(LPError::InsufficientInputAmount);
        }
        let balance0_adjusted = balance0 * U256::from(1000) - amount0_in * U256::from(3);
        let balance1_adjusted = balance1 * U256::from(1000) - amount1_in * U256::from(3);
        if balance0_adjusted * balance1_adjusted < reserve0 * reserve1 * U256::from(1000 * 1000) {
            return Err(LPError::KConstant);
        }
        let _ = self._update((balance0, balance1), (reserve0, reserve1));
        // Emit Swap event
        self.notify_on(LPEvent::Swap {
            sender: msg::source(),
            amount_in: (amount0_in, amount1_in),
            amount_out: (amount0_out, amount1_out),
            to,
        }).unwrap();

        Ok(())
    }

    pub async fn skim(&mut self, to:ActorId) -> Result<(), LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get();
        let token_pair = state_lp.token.clone();
        let balance0_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.0.clone()).await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.1.clone()).await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        self._safe_transfer(token_pair.0.clone(), to, balance0 - reserve0).await?;
        self._safe_transfer(token_pair.1.clone(), to, balance1 - reserve1).await?;

        self.notify_on(LPEvent::Skim { amount_a: (balance0 - reserve0), amount_b: (balance1 - reserve1), to }).unwrap();
        Ok(())
    }

    pub async fn sync(&mut self) -> Result<(), LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get();
        let token_pair = state_lp.token.clone();
        let balance0_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.0.clone()).await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self.vft_client.balance_of(exec::program_id()).recv(token_pair.1.clone()).await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let _ = self._update((balance0, balance1), (reserve0, reserve1));
        self.notify_on(LPEvent::Sync { reserve_a: state_lp.reserve.0, reserve_b: state_lp.reserve.1 }).unwrap();
        Ok(())
    }

    pub fn get_reserves(&self) ->(U256, U256, u64) {
        let state_lp = StateLp::get();
        (state_lp.reserve.0, state_lp.reserve.1, exec::block_timestamp())
    }   
}

impl AsRef<VftService> for LPService {
    fn as_ref(&self) -> &VftService {
        &self.vft_service
    }
}