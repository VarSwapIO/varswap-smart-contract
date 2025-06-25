use crate::clients::extended_new_vft::traits::Vft;
use crate::clients::extended_new_vft::Vft as VftClient;
use crate::clients::factory_vara_dex_client::traits::FactoryService;
use crate::clients::factory_vara_dex_client::FactoryService as FactoryServiceClient;
use crate::states::lp_state::{LPError, LPEvent, StateLp, LP, MINIMUM_LIQUIDITY};
use gstd::exec;
use sails_rs::calls::{Call, Query};
use sails_rs::gstd::calls::GStdRemoting;
use sails_rs::{gstd::msg, prelude::*};
use vft_service::{Service as VftService, Storage};

pub struct LPService {
    pub vft_client: VftClient<GStdRemoting>,
    pub vft_service: VftService,
    pub factory_client: FactoryServiceClient<GStdRemoting>,
}

impl LPService {
    pub fn seed(
        factory: ActorId,
        token_a: ActorId,
        token_b: ActorId,
        name: String,
        symbol: String,
        decimals: u8,
        admin: ActorId,
        router: ActorId,
    ) -> Self {
        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };
        unsafe {
            LP = Some(StateLp {
                name: name.clone(),
                symbol: symbol.clone(),
                admin,
                router,
                decimals,
                factory,
                token: token_pair,
                // lock: false,
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
    pub fn new(
        vft_client: VftClient<GStdRemoting>,
        factory_client: FactoryServiceClient<GStdRemoting>,
    ) -> Self {
        Self {
            vft_client,
            vft_service: VftService::new(),
            factory_client,
        }
    }

    pub fn get_reserves(&self) -> (U256, U256, u64) {
        let state_lp = StateLp::get();
        (
            state_lp.reserve.0,
            state_lp.reserve.1,
            exec::block_timestamp(),
        )
    }

    pub fn get_admin(&self) -> ActorId {
        let state_lp = StateLp::get();
        state_lp.admin
    }

    pub fn get_router(&self) -> ActorId {
        let state_lp = StateLp::get();
        state_lp.router
    }

    pub fn get_factory(&self) -> ActorId {
        let state_lp = StateLp::get();
        state_lp.factory
    }

    pub fn set_admin(&mut self, new_admin: ActorId) -> Result<(), LPError> {
        let state_lp = StateLp::get_mut();
        if msg::source() != state_lp.admin {
            return Err(LPError::Unauthorized);
        }
        if new_admin == ActorId::zero() {
            return Err(LPError::InvalidAdmin);
        }
        state_lp.admin = new_admin;
        self.emit_event(LPEvent::AdminSet(new_admin)).ok();
        Ok(())
    }

    pub fn set_router(&mut self, new_router: ActorId) -> Result<(), LPError> {
        let state_lp = StateLp::get_mut();
        if msg::source() != state_lp.admin {
            return Err(LPError::Unauthorized);
        }
        if new_router == ActorId::zero() {
            return Err(LPError::InvalidRouter);
        }
        state_lp.router = new_router;
        self.emit_event(LPEvent::RouterSet(new_router)).ok();
        Ok(())
    }

    pub async fn mint(&mut self, to: ActorId) -> Result<U256, LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get_mut();

        let token_pair = state_lp.token.clone();
        let balance_0_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.0.clone())
            .await;
        let Ok(balance_0) = balance_0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance_1_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.1.clone())
            .await;
        let Ok(balance_1) = balance_1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let amount_0 = balance_0.checked_sub(reserve0).ok_or(LPError::Overflow)?;
        let amount_1 = balance_1.checked_sub(reserve1).ok_or(LPError::Overflow)?;

        let fee_on = self._mint_fee(reserve0, reserve1).await?;
        let total_supply = Storage::total_supply().clone();
        let liquidity = if total_supply == U256::zero() {
          
            let mint_amount = amount_0
                .checked_mul(amount_1)
                .map(|v| v.integer_sqrt())
                .and_then(|v| v.checked_sub(U256::from(MINIMUM_LIQUIDITY)))
                .ok_or(LPError::Overflow)?;
            self._mint(ActorId::zero(), U256::from(MINIMUM_LIQUIDITY))?;
            mint_amount
        } else {
          
            let amount_0_min = amount_0
                .checked_mul(total_supply)
                .and_then(|v| v.checked_div(reserve0))
                .ok_or(LPError::Overflow)?;
            let amount_1_min = amount_1
                .checked_mul(total_supply)
                .and_then(|v| v.checked_div(reserve1))
                .ok_or(LPError::Overflow)?;
            gstd::cmp::min(amount_0_min, amount_1_min)
        };
        if liquidity <= U256::zero() {
            return Err(LPError::InsufficientLiquidityMinted);
        };
        self._mint(to, liquidity.clone())?;
        self._update((balance_0, balance_1), (reserve0, reserve1))?;
        if fee_on {
            state_lp.k_last = state_lp
                .reserve
                .0
                .checked_mul(state_lp.reserve.1)
                .ok_or(LPError::Overflow)?;
        }

        self.emit_event(LPEvent::Mint {
            sender: msg::source(),
            amount: (amount_0, amount_1),
        })
        .ok();
        Ok(liquidity)
    }

    pub async fn burn(&mut self, to: ActorId) -> Result<(U256, U256), LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get_mut();

        let token_pair = state_lp.token.clone();
        let balance0_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.0.clone())
            .await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.1.clone())
            .await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let liquidity = self.vft_service.balance_of(exec::program_id());

        let fee_on = self._mint_fee(reserve0, reserve1).await?;
        let total_supply = self.vft_service.total_supply().clone();

     
        let amount0 = liquidity
            .checked_mul(balance0)
            .and_then(|v| v.checked_div(total_supply))
            .ok_or(LPError::Overflow)?;
       
        let amount1 = liquidity
            .checked_mul(balance1)
            .and_then(|v| v.checked_div(total_supply))
            .ok_or(LPError::Overflow)?;

        if amount0 == U256::zero() || amount1 == U256::zero() {
            return Err(LPError::InsufficientLiquidityBurned);
        }
        self._burn(exec::program_id(), liquidity)?;
        let transfer_a_res = self._safe_transfer(token_pair.0.clone(), to, amount0).await;
        if transfer_a_res.is_err() {
            return Err(LPError::TransferFailed);
        }
        let transfer_b_res = self._safe_transfer(token_pair.1.clone(), to, amount1).await;
        if transfer_b_res.is_err() {
            return Err(LPError::TransferFailed);
        }
        let balance0_after_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.0.clone())
            .await;
        let Ok(balance0_after) = balance0_after_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_after_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.1.clone())
            .await;
        let Ok(balance1_after) = balance1_after_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        self._update((balance0_after, balance1_after), (reserve0, reserve1))?;
        if fee_on {
            state_lp.k_last = state_lp
                .reserve
                .0
                .checked_mul(state_lp.reserve.1)
                .ok_or(LPError::Overflow)?;
        }

        self.emit_event(LPEvent::Burn {
            sender: msg::source(),
            amount: (amount0, amount1),
            to,
        })
        .ok();

        Ok((amount0, amount1))
    }

    pub async fn swap(
        &mut self,
        amount0_out: U256,
        amount1_out: U256,
        to: ActorId,
    ) -> Result<(), LPError> {
        if amount0_out == U256::zero() && amount1_out == U256::zero() {
            return Err(LPError::InsufficientOutputAmount);
        }

        let (reserve0, reserve1, _) = self.get_reserves();
        if amount0_out >= reserve0 || amount1_out >= reserve1 {
            return Err(LPError::InsufficientLiquidity);
        }
        let state_lp = StateLp::get_mut();

        let token_pair = state_lp.token.clone();

        if to == token_pair.0 || to == token_pair.1 {
            return Err(LPError::InvalidTo);
        }
        if amount0_out > U256::zero() {
            let transfer_res = self._safe_transfer(token_pair.0, to, amount0_out).await;
            if transfer_res.is_err() {
                return Err(LPError::TransferFailed);
            }
        }
        if amount1_out > U256::zero() {
            let transfer_res = self._safe_transfer(token_pair.1, to, amount1_out).await;
            if transfer_res.is_err() {
                return Err(LPError::TransferFailed);
            }
        }
        let balance0_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.0.clone())
            .await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.1.clone())
            .await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let remain_0 = reserve0.checked_sub(amount0_out).ok_or(LPError::Overflow)?;
        let amount0_in = if balance0 > remain_0 {
            balance0.checked_sub(remain_0).ok_or(LPError::Overflow)?
        } else {
            U256::zero()
        };
        let remain_1 = reserve1.checked_sub(amount1_out).ok_or(LPError::Overflow)?;
        let amount1_in = if balance1 > remain_1 {
            balance1.checked_sub(remain_1).ok_or(LPError::Overflow)?
        } else {
            U256::zero()
        };
        if amount0_in == U256::zero() && amount1_in == U256::zero() {
            return Err(LPError::InsufficientInputAmount);
        }
    

        let x_adjusted = amount0_in
            .checked_mul(U256::from(3))
            .ok_or(LPError::Overflow)?;

        let balance0_adjusted = balance0
            .checked_mul(U256::from(1_000))
            .and_then(|v| v.checked_sub(x_adjusted))
            .ok_or(LPError::Overflow)?;
        
        let y_adjusted = amount1_in
            .checked_mul(U256::from(3))
            .ok_or(LPError::Overflow)?;
        let balance1_adjusted = balance1
            .checked_mul(U256::from(1_000))
            .and_then(|v| v.checked_sub(y_adjusted))
            .ok_or(LPError::Overflow)?;


        let left = balance0_adjusted.checked_mul(balance1_adjusted).ok_or(LPError::Overflow)?;
        let right = reserve0
            .checked_mul(reserve1)
            .and_then(|v| v.checked_mul(U256::from(1_000 * 1_000)))
            .ok_or(LPError::Overflow)?;
        if left < right {
            return Err(LPError::KConstant);
        }
        self._update((balance0, balance1), (reserve0, reserve1))?;
        // Emit Swap event
        self.emit_event(LPEvent::Swap {
            sender: msg::source(),
            amount_in: (amount0_in, amount1_in),
            amount_out: (amount0_out, amount1_out),
            to,
        })
        .ok();

        Ok(())
    }

    pub async fn skim(&mut self, to: ActorId) -> Result<(), LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get_mut();

        if msg::source() != state_lp.admin && msg::source() != state_lp.router {
            return Err(LPError::Unauthorized);
        }

        let token_pair = state_lp.token.clone();
        let balance0_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.0.clone())
            .await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.1.clone())
            .await;
        let Ok(balance1) = balance1_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let transfer_a_res = self
            ._safe_transfer(
                token_pair.0.clone(),
                to,
                balance0.checked_sub(reserve0).ok_or(LPError::Overflow)?,
            )
            .await;
        if transfer_a_res.is_err() {
            return Err(LPError::TransferFailed);
        }
        let transfer_b_res = self
            ._safe_transfer(
                token_pair.1.clone(),
                to,
                balance1.checked_sub(reserve1).ok_or(LPError::Overflow)?,
            )
            .await;
        if transfer_b_res.is_err() {
            return Err(LPError::TransferFailed);
        }

        self.emit_event(LPEvent::Skim {
            amount_a: (balance0 - reserve0),
            amount_b: (balance1 - reserve1),
            to,
        })
        .ok();
        Ok(())
    }

    pub async fn sync(&mut self) -> Result<(), LPError> {
        let (reserve0, reserve1, _) = self.get_reserves();
        let state_lp = StateLp::get_mut();

        if msg::source() != state_lp.admin && msg::source() != state_lp.router {
            return Err(LPError::Unauthorized);
        }

        let token_pair = state_lp.token.clone();
        let balance0_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.0.clone())
            .await;
        let Ok(balance0) = balance0_res else {
            return Err(LPError::CanNotConnectToVft);
        };
        let balance1_res = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(token_pair.1.clone())
            .await;
        let Ok(balance1) = balance1_res else {
            // state_lp.lock = false;
            return Err(LPError::CanNotConnectToVft);
        };
        self._update((balance0, balance1), (reserve0, reserve1))?;
        // state_lp.lock = false
        self.emit_event(LPEvent::Sync {
            reserve_a: state_lp.reserve.0,
            reserve_b: state_lp.reserve.1,
        })
        .ok();
        Ok(())
    }

    async fn _mint_fee(&mut self, reserve_0: U256, reserve_1: U256) -> Result<bool, LPError> {
        let state_lp = StateLp::get_mut();

        let fee_to_res = self
            .factory_client
            .get_fee_to()
            .recv(state_lp.factory)
            .await;
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
                let root_k = (reserve_0.checked_mul(reserve_1).ok_or(LPError::Overflow)?).integer_sqrt();
                let root_klast = _k_last.integer_sqrt();
                if root_k > root_klast {
                    let numerator = Storage::total_supply()
                        .checked_mul(root_k.checked_sub(root_klast).ok_or(LPError::Overflow)?)
                        .ok_or(LPError::Overflow)?;
                    let denominator = root_k.checked_mul(U256::from(5)).ok_or(LPError::Overflow)?;
                    let liquidity = numerator
                        .checked_div(denominator)
                        .ok_or(LPError::Overflow)?;
                    if liquidity > U256::zero() {
                        self._mint(fee_to, liquidity)?;
                    }
                }
            }
        } else if _k_last == U256::zero() {
            state_lp.k_last = U256::zero();
        } else {
            state_lp.k_last = U256::zero();
        }
        Ok(fee_on)
    }

    async fn _safe_transfer(
        &mut self,
        token: ActorId,
        to: ActorId,
        value: U256,
    ) -> Result<(), LPError> {
        if value.is_zero() {
            return Err(LPError::InvalidAmount);
        }

        let transfer_res = self
            .vft_client
            .transfer(to, value)
            // .with_gas_limit(5_000_000_000)
            .send_recv(token)
            .await;
        let Ok(transfer_status) = transfer_res else {
            return Err(LPError::TransferFailed);
        };
        if !transfer_status {
            return Err(LPError::TransferFailed);
        } else {
            Ok(())
        }
    }

    fn _mint(&mut self, to: ActorId, liquidity: U256) -> Result<(), LPError> {
        if liquidity <= U256::zero() {
            return Err(LPError::InsufficientLiquidityMinted);
        }

        let old_balance = self.vft_service.balance_of(to);
        let new_balance = old_balance
            .checked_add(liquidity)
            .ok_or(LPError::Overflow)?;
        let storage_balance = Storage::balances();
        storage_balance.insert(to, new_balance);
        //update total supply
        let total_supply = Storage::total_supply();
        *total_supply = total_supply
            .checked_add(liquidity)
            .ok_or(LPError::Overflow)?;

        self.emit_event(LPEvent::LPMint {
            to,
            amount: liquidity,
        })
        .ok();
        Ok(())
    }

    fn _burn(&mut self, from: ActorId, liquidity: U256) -> Result<(), LPError> {
        if liquidity <= U256::zero() {
            return Err(LPError::InsufficientLiquidityBurned);
        }

        let old_balance = self.vft_service.balance_of(from);

        if liquidity > old_balance {
            return Err(LPError::InsufficientLiquidityBurned);
        }

        let new_balance = old_balance
            .checked_sub(liquidity)
            .ok_or(LPError::Overflow)?;
        let storage_balance = Storage::balances();
        if !new_balance.is_zero() {
            storage_balance.insert(from, new_balance);
        } else {
            storage_balance.remove(&from);
        };
        let total_supply = Storage::total_supply();

        if *total_supply < liquidity {
            return Err(LPError::InsufficientLiquidityBurned);
        }

        *total_supply = total_supply
            .checked_sub(liquidity)
            .ok_or(LPError::Overflow)?;
        self.emit_event(LPEvent::LPBurn {
            from: from,
            amount: liquidity,
        })
        .ok();
        Ok(())
    }

    fn _update(&mut self, balance: (U256, U256), reverse: (U256, U256)) -> Result<(), LPError> {
        let state_lp = StateLp::get_mut();
        let block_timestamp = exec::block_timestamp() % 2u64.pow(32);
        let time_elapsed = block_timestamp.saturating_sub(state_lp.last_block_ts);
        if time_elapsed > 0 && reverse.0 != U256::zero() && reverse.1 != U256::zero() {
          
            state_lp.cumulative_price.0 = reverse
                .1
                .checked_div(reverse.0)
                .and_then(|v| v.checked_mul(U256::from(time_elapsed)))
                .and_then(|v| state_lp.cumulative_price.0.checked_add(v))
                .ok_or(LPError::Overflow)?;
          
            state_lp.cumulative_price.1 = reverse
                .0
                .checked_div(reverse.1)
                .and_then(|v| v.checked_mul(U256::from(time_elapsed)))
                .and_then(|v| state_lp.cumulative_price.1.checked_add(v))
                .ok_or(LPError::Overflow)?;
        }
        state_lp.reserve.0 = balance.0;
        state_lp.reserve.1 = balance.1;
        state_lp.last_block_ts = block_timestamp;
        Ok(())
    }
}

impl AsRef<VftService> for LPService {
    fn as_ref(&self) -> &VftService {
        &self.vft_service
    }
}
