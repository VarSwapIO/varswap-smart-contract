use gstd::{exec, msg, ActorId};
use parity_scale_codec::{Decode, Encode};
use sails_rs::calls::{Action, Call, Query};
use sails_rs::gstd::calls::GStdRemoting;
use sails_rs::{gstd::service, prelude::*};

use crate::states::router_state::{RouterError, RouterEvent, RouterState, ROUTER};
use crate::clients::factory_vara_dex_client::FactoryService as FactoryServiceClient;
use crate::clients::factory_vara_dex_client::traits::FactoryService;
use crate::clients::extended_new_vft::Vft as VftClient;
use crate::clients::extended_new_vft::traits::Vft;
use crate::clients::lp_vara_dex_client::LpService as LpServiceClient;
use crate::clients::lp_vara_dex_client::traits::LpService;

pub struct RouterService {
    pub factory_client: FactoryServiceClient<GStdRemoting>,
    pub vft_client: VftClient<GStdRemoting>,
    pub lp_client: LpServiceClient<GStdRemoting>,
}

impl RouterService {
    pub fn seed(factory_address: ActorId, wvara_address: ActorId, admin_addr:ActorId, fund_addr:ActorId, swap_fee_bps:u128) {
        unsafe {
            ROUTER = Some(RouterState { factory_address, wvara_address, admin:admin_addr, fund_addr, swap_fee_bps });
        }
    }
}

#[service(events = RouterEvent)]
impl RouterService {
    pub fn new(
        factory_client: FactoryServiceClient<GStdRemoting>, 
        vft_client: VftClient<GStdRemoting>, 
        lp_client: LpServiceClient<GStdRemoting>) -> Self {
        Self { factory_client, vft_client, lp_client }
    }


    //admin functions

    pub fn update_new_admin(&mut self, new_addr:ActorId) -> Result<bool,RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        router_state.admin = new_addr;
        Ok(true)
    }

    pub fn update_new_factorty(&mut self, new_factory_addr:ActorId) -> Result<bool, RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        router_state.factory_address = new_factory_addr;
        Ok(true)
    }

    pub fn update_new_wrapvara(&mut self, new_wvara_addr:ActorId) -> Result<bool, RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        router_state.wvara_address = new_wvara_addr;
        Ok(true)
    }

    pub fn update_fund_addr(&mut self, new_fund_addr:ActorId) -> Result<bool, RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        router_state.fund_addr = new_fund_addr;
        Ok(true)
    }

    pub fn update_swap_fee_bps(&mut self, new_swap_fee_bps:u128) -> Result<bool, RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        router_state.swap_fee_bps = new_swap_fee_bps;
        Ok(true)
    }

    pub async fn refund_token(&mut self, token_addr:ActorId, amount:U256) -> Result<bool, RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        let _ = self._transfer(token_addr, sender, amount).await?;
        Ok(true)
    }

    pub async fn refund_vara(&mut self, amount:u128) -> Result<bool, RouterError> {
        let router_state = RouterState::get_mut();
        let sender = msg::source();
        if sender == ActorId::zero() || sender != router_state.admin{
            return  Err(RouterError::IdenticalAddresses);
        }
        let _ = msg::send_bytes(sender, b"Refund Vara", amount);
        Ok(true)
    }

    //view functions
    pub fn sort_tokens(&self, token_a: ActorId, token_b: ActorId) -> Result<(ActorId, ActorId),RouterError> {
        if token_a == token_b {
            return Err(RouterError::IdenticalAddresses);
        }
        let (token0, token1) = if token_a > token_b { (token_a, token_b) } else { (token_b, token_a) };
        if token0 == ActorId::zero() {
            return Err(RouterError::ZeroAddress);
        }
        Ok((token0, token1))
    }

    pub async fn pair_for(&self, token_a: ActorId, token_b: ActorId) -> Result<ActorId,RouterError> {
        let router_state = RouterState::get();
        let pair_res = self.factory_client.get_pair(token_a, token_b).recv(router_state.factory_address).await;
        match pair_res {
            Ok(pair) => Ok(pair),
            Err(_) => Err(RouterError::PairNotFound),
        }
    }

    pub async fn get_reserves(&self, token_a: ActorId, token_b: ActorId) -> Result<(U256, U256, ActorId),RouterError> {
        let (token0, _) = self.sort_tokens(token_a, token_b)?;
        let pair = self.pair_for(token_a, token_b).await?;
        let res = self.lp_client.get_reserves().recv(pair).await;
        match res {
            Ok((reserve0, reserve1, _)) => {
                if token_a == token0 { Ok((reserve0, reserve1, pair)) } else { Ok((reserve1, reserve0, pair)) }
            }
            Err(_) => Err(RouterError::PairNotFound),
        }
    }

    pub fn quote(&self, amount_a: U256, reserve_a: U256, reserve_b: U256) -> Result<U256,RouterError> {
        if amount_a == U256::zero() {
            return Err(RouterError::InsufficientAAmount);
        }
        if reserve_a == U256::zero() || reserve_b == U256::zero() {
            return Err(RouterError::InsufficientLiquidity);
        }
        Ok(amount_a * reserve_b / reserve_a)
    }

    pub fn get_amount_out(&self,amount_in: U256, reserve_in: U256, reserve_out: U256) -> Result<U256,RouterError> {
        if amount_in == U256::zero() {
            return Err(RouterError::InsufficientInputAmount);
        }
        if reserve_in == U256::zero() || reserve_out == U256::zero() {
            return Err(RouterError::InsufficientLiquidity);
        }
        let amount_in_with_fee = amount_in * U256::from(997);
        let numerator = amount_in_with_fee * reserve_out;
        let denominator = reserve_in * U256::from(1000) + amount_in_with_fee;
        Ok(numerator / denominator)
    }

    pub fn get_amount_in(&self,amount_out: U256, reserve_in: U256, reserve_out: U256) -> Result<U256,RouterError> {
        if amount_out == U256::zero() {
            return Err(RouterError::InsufficientOutputAmount);
        }
        if reserve_in == U256::zero() || reserve_out == U256::zero() {
            return Err(RouterError::InsufficientLiquidity);
        }
        let numerator = reserve_in * amount_out * U256::from(1000);
        let denominator = (reserve_out - amount_out) * U256::from(997);
        Ok((numerator / denominator) + U256::from(1))
    }

    pub async fn get_amounts_out(&self, amount_in: U256, path: Vec<ActorId>) -> Result<Vec<U256>,RouterError> {
        if path.len() < 2 {
            return Err(RouterError::InvalidPath);
        }
        let mut amounts = vec![amount_in];
        for i in 0..path.len() - 1 {
            let (reserve_in, reserve_out,_) = self.get_reserves(path[i], path[i + 1]).await?;
            amounts.push(self.get_amount_out(amounts[i], reserve_in, reserve_out)?);
        }
        Ok(amounts)
    }

    pub async fn get_amounts_in(&self, amount_out: U256, path: Vec<ActorId>) -> Result<Vec<U256>,RouterError> {
        if path.len() < 2 {
            return Err(RouterError::InvalidPath);
        }
        let mut amounts = vec![U256::zero(); path.len()];
        let len = amounts.len();
        amounts[len- 1] = amount_out;
        for i in (1..path.len()).rev() {
            let (reserve_in, reserve_out,_) = self.get_reserves(path[i - 1], path[i]).await?;
            amounts[i - 1] = self.get_amount_in(amounts[i], reserve_in, reserve_out)?;
        }
        Ok(amounts)
    }

    // private functions
    async fn _add_liquidity(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
    ) -> Result<(U256, U256), RouterError> {
        

        let (reserve_a, reserve_b,_) = self.get_reserves(token_a, token_b).await?;

        if reserve_a.is_zero() && reserve_b.is_zero() {
            Ok((amount_a_desired, amount_b_desired))
        } else {
            let amount_b_optimal = self.quote(amount_a_desired, reserve_a, reserve_b)?;
            if amount_b_optimal <= amount_b_desired {
                if amount_b_optimal < amount_b_min {
                    return Err(RouterError::InsufficientBAmount);
                }
                Ok((amount_a_desired, amount_b_optimal))
            } else {
                let amount_a_optimal = self.quote(amount_b_desired, reserve_b, reserve_a)?;
                assert!(amount_a_optimal <= amount_a_desired);
                if amount_a_optimal < amount_a_min {
                    return Err(RouterError::InsufficientAAmount);
                }
                Ok((amount_a_optimal, amount_b_desired))
            }
        }
    }

    async fn _swap(
        &mut self,
        amounts: Vec<U256>,
        path: Vec<ActorId>,
        to: ActorId,
    ) -> Result<(), RouterError> {
        for i in 0..path.len() - 1 {
            let (input, output) = (path[i], path[i + 1]);
            let (token0, _) = self.sort_tokens(input, output)?;
            let amount_out = amounts[i + 1];
            let (amount0_out, amount1_out) = if input == token0 {
                (U256::zero(), amount_out)
            } else {
                (amount_out, U256::zero())
            };
            let to = if i < path.len() - 2 {
                self.pair_for(output, path[i + 2]).await?
            } else {
                to
            };
            let pair = self.pair_for(input, output).await?;
            let swap_res = self.lp_client.swap(amount0_out, amount1_out, to).send_recv(pair).await;
            if swap_res.is_err() {
                return Err(RouterError::SwapFailed);
            }
        }
        Ok(())
    }

    async fn _transfer_from(&mut self, token:ActorId, from:ActorId, to:ActorId, value:U256) -> Result<(),RouterError>{
        let send_token_res = self.vft_client.transfer_from( from, to, value).send_recv(token).await;
        let Ok(transfer_token_status) = send_token_res else {
            return Err(RouterError::TransferFromFailed);
        };
        if !transfer_token_status {
            return Err(RouterError::TransferFromFailed);
        }
        Ok(())
    }

    async fn _transfer(&mut self, token:ActorId, to: ActorId, value:U256) -> Result<(),RouterError>{
        let transfer_wvara_res =  self.vft_client.transfer(to, value).send_recv(token).await;
        let Ok(transfer_wvara_status) = transfer_wvara_res else {
            return Err(RouterError::TransferFailed);
        };
        if !transfer_wvara_status {
            return Err(RouterError::TransferFailed);
        };
        Ok(())
    }

    async fn _wrap_vara(&mut self, vara_amount:u128) -> Result<(),RouterError> {
        let router_state = RouterState::get();
        let wrapped_vara = router_state.wvara_address;
        let deposit_res = self.vft_client.deposit().with_value(vara_amount).send_recv(wrapped_vara).await;
        let Ok(deposit_status) = deposit_res else {
            return Err(RouterError::DepositWVARAFailed);
        };
        if !deposit_status {
            return Err(RouterError::DepositWVARAFailed);
        };
        Ok(())
    }

    async fn _unwrap_vara(&mut self, vara_amount:U256) -> Result<(),RouterError> {
        let router_state = RouterState::get();
        let wrapped_vara = router_state.wvara_address;
        let withdraw_res = self.vft_client.withdraw(vara_amount).send_recv(wrapped_vara).await;
        let Ok(withdraw_status) = withdraw_res else {
            return Err(RouterError::WithdrawWvaraFailed);
        };
        if !withdraw_status {
            return Err(RouterError::WithdrawWvaraFailed);
        };
        Ok(())
    }
    // public functions

    async fn transfer_from_liquidity(&mut self, pair:ActorId, from:ActorId, to:ActorId, liquidity:U256) -> Result<(),RouterError>{
        let transfer_liquidity_res = self.lp_client.transfer_from(from, to, liquidity).send_recv(pair).await;
        let Ok(transfer_liquidity_status) = transfer_liquidity_res else {
            return Err(RouterError::TransferFromLiquidityFailed);
        };
        if !transfer_liquidity_status {
            return Err(RouterError::TransferFromLiquidityFailed);
        };
        Ok(())
    }

    pub async fn create_pair(&mut self, token_a:ActorId, token_b:ActorId) -> Result<(),RouterError>{
        let router_state = RouterState::get();
        if self.pair_for(token_a, token_b).await?.is_zero() {
            let create_fee = msg::value();
            let create_pair_res = self.factory_client.create_pair(token_a, token_b).with_value(create_fee).send_recv(router_state.factory_address).await;
            let Ok(pair_address) = create_pair_res.unwrap() else {
                return Err(RouterError::CreatePairFailed);
            };
            self.notify_on(RouterEvent::CreatePair { token_a, token_b, pair_address }).unwrap();
        }else {
            return Err(RouterError::PairAlreadyExists);
        }
        Ok(())
    }

    pub async fn add_liquidity(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: ActorId,
        deadline: u64,
    ) -> Result<(U256, U256, U256), RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        let (amount_a, amount_b) = self._add_liquidity(
            token_a, token_b, amount_a_desired, amount_b_desired, amount_a_min, amount_b_min,
        ).await?;

        let pair = self.pair_for(token_a, token_b).await?;
        let _ = self._transfer_from(token_a, msg::source(), pair, amount_a).await?;
        let _ = self._transfer_from(token_b, msg::source(), pair, amount_b).await?;
        // Mint LP tokens
        let mint_liquidity_res = self.lp_client.mint(to).send_recv(pair).await;
        let Ok(liquidity) = mint_liquidity_res.unwrap() else {
            return Err(RouterError::MintLiquidityFailed);
        };
        self.notify_on(RouterEvent::AddLiquidity { token_a, token_b, amount_a, amount_b, to, liquidity: liquidity.clone() }).unwrap();
        Ok((amount_a, amount_b, liquidity))
    }

    pub async fn add_liquidity_vara(
        &mut self,
        token: ActorId,
        amount_token_desired: U256,
        amount_token_min: U256,
        amount_vara_min: U256,
        to: ActorId,
        deadline: u64,
    ) -> Result<(U256, U256, U256), RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        let router_state = RouterState::get();
        let wrapped_vara = router_state.wvara_address;
        let amount_vara_desired = U256::from(msg::value());
        let (amount_token, amount_vara) = self._add_liquidity(
            token,
            wrapped_vara,
            amount_token_desired,
            amount_vara_desired,
            amount_token_min,
            amount_vara_min,
        ).await?;
        let pair = self.pair_for(token, wrapped_vara).await?;
        if pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self._transfer_from(token, msg::source(), pair, amount_token).await?;
        let _ = self._wrap_vara(amount_vara.as_u128()).await?;
        let _ = self._transfer(wrapped_vara, pair, amount_vara).await?;
        // Mint LP tokens
        let liquidity_res = self.lp_client.mint(to).send_recv(pair).await;
        let Ok(liquidity) = liquidity_res.unwrap() else {
            return Err(RouterError::MintLiquidityFailed);
        };
        if amount_vara.as_u128() < msg::value() {
            let refund = msg::value() - amount_vara.as_u128();
            let _ = msg::send_bytes(msg::source(), b"Transfer Vara", refund);
        }
        self.notify_on(RouterEvent::AddLiquidityVARA { token_a: token, amount_a: amount_token, amount_vara, to, liquidity: liquidity.clone() }).unwrap();

        Ok((amount_token, amount_vara, liquidity))
    }

    pub async fn remove_liquidity_vara(
        &mut self,
        token: ActorId,
        liquidity: U256,
        amount_token_min: U256,
        amount_vara_min: U256,
        to: ActorId,
        deadline: u64,
    ) -> Result<(U256, U256), RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }

        let router_state = RouterState::get();
        let wrapped_vara = router_state.wvara_address;
        let pair = self.pair_for(token, wrapped_vara).await?;
        if pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self.transfer_from_liquidity(pair, msg::source(), pair, liquidity).await?;
        // Burn LP tokens
        let burn_res = self.lp_client.burn(exec::program_id()).send_recv(pair).await;
        let Ok((amount0, amount1)) = burn_res.unwrap() else {
            return Err(RouterError::BurnLiquidityFailed);
        };
        let (token0, _) = self.sort_tokens(token, wrapped_vara)?;
        let (amount_token, amount_vara) = if token == token0 {
            (amount0, amount1)
        } else {
            (amount1, amount0)
        };
        if amount_token < amount_token_min {
            return Err(RouterError::InsufficientTokenAmount);
        }
        if amount_vara < amount_vara_min {
            return Err(RouterError::InsufficientVaraAmount);
        }
        let _ = self._transfer(token, to, amount_token).await?;
        let _ = self._unwrap_vara(amount_vara).await?;
        let _ = msg::send_bytes(to, b"Transfer Vara", amount_vara.as_u128());
        self.notify_on(RouterEvent::RemoveLiquidityVARA { token_a: token, amount_a_received: amount_token, amount_vara_received: amount_vara, to, liquidity }).unwrap();

        Ok((amount_token, amount_vara))
    }

    pub async fn remove_liquidity(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: ActorId,
        deadline: u64,
    ) -> Result<(U256, U256), RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }

        let pair = self.pair_for(token_a, token_b).await?;
        
        // Transfer LP tokens to pair
        let _ = self.transfer_from_liquidity(pair, msg::source(), pair, liquidity).await?;

        // Burn LP tokens
        let burn_res = self.lp_client.burn(to).send_recv(pair).await;
        let Ok((amount0, amount1)) = burn_res.unwrap() else {
            return Err(RouterError::MintLiquidityFailed);
        };

        let (token0, _) = self.sort_tokens(token_a, token_b)?;
        let (amount_a, amount_b) = if token_a == token0 {
            (amount0, amount1)
        } else {
            (amount1, amount0)
        };

        if amount_a < amount_a_min {
            return Err(RouterError::InsufficientAAmount);
        }
        if amount_b < amount_b_min {
            return Err(RouterError::InsufficientBAmount);
        }

        self.notify_on(RouterEvent::RemoveLiquidity{token_a, token_b, amount_a_received: amount_a, amount_b_received: amount_b, to, liquidity}).unwrap();
        Ok((amount_a, amount_b))
    }

    pub async fn swap_exact_tokens_for_tokens(
        &mut self,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<ActorId>,
        to: ActorId,
        deadline: u64,
    ) -> Result<Vec<U256>, RouterError> {

        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }

        let amounts = self.get_amounts_out(amount_in, path.clone()).await?;

        let amount_out = amounts[amounts.len() - 1];

        if amount_out < amount_out_min {
            return Err(RouterError::InsufficientOutputAmount);
        }

        // Transfer tokens from sender to first pair
        let first_pair = self.pair_for(path[0], path[1]).await?;
        if first_pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self._transfer_from(path[0], msg::source(), first_pair, amounts[0]).await?;
        // Perform the swap
        self._swap(amounts.clone(), path.clone(), to).await?;

        self.notify_on(RouterEvent::SwapExactTokensForTokens{amount_in, amount_out, path: path.clone(), to}).unwrap();

        Ok(amounts)
    }

    pub async fn swap_tokens_for_exact_tokens(
        &mut self,
        amount_out: U256,
        amount_in_max: U256,
        path: Vec<ActorId>,
        to: ActorId,
        deadline: u64,
    ) -> Result<Vec<U256>, RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        let amounts = self.get_amounts_in(amount_out, path.clone()).await?;
        if amounts[0] > amount_in_max {
            return Err(RouterError::ExcessiveInputAmount);
        }
        let first_pair = self.pair_for(path[0], path[1]).await?;
        if first_pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self._transfer_from(path[0], msg::source(), first_pair, amounts[0]).await?;

        self._swap(amounts.clone(), path.clone(), to).await?;

        self.notify_on(RouterEvent::SwapTokensForExactTokens{amount_out, amount_in: amounts[0], path: path.clone(), to}).unwrap();

        Ok(amounts)
    }

    pub async fn swap_exact_vara_for_tokens(
        &mut self,
        amount_out_min: U256,
        path: Vec<ActorId>,
        to: ActorId,
        deadline: u64,
    ) -> Result<Vec<U256>, RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        let router_state = RouterState::get();
        let wrapped_vara = router_state.wvara_address;
        let vara_amount = msg::value();
        let amounts = self.get_amounts_out(U256::from(vara_amount), path.clone()).await?;
        let amount_out = amounts[amounts.len() - 1];
        if amount_out < amount_out_min {
            return Err(RouterError::InsufficientOutputAmount);
        }
        
        let first_pair = self.pair_for(wrapped_vara, path[1]).await?;
        if first_pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        //deposit vara to wvara
        let _ = self._wrap_vara(vara_amount).await?;
        let _ = self._transfer(wrapped_vara, first_pair, U256::from(vara_amount)).await?;
        self._swap(amounts.clone(), path.clone(), to).await?;
        self.notify_on(RouterEvent::SwapExactVARAForTokens { amount_in:U256::from(vara_amount), amount_out, path: path.clone(), to  }).unwrap();
        Ok(amounts)
    }

    pub async fn swap_tokens_for_exact_vara(
        &mut self,
        amount_out: U256,
        amount_in_max: U256,
        path: Vec<ActorId>,
        to: ActorId,
        deadline: u64,
    ) -> Result<Vec<U256>, RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        let amounts = self.get_amounts_in(amount_out, path.clone()).await?;
        if amounts[0] > amount_in_max {
            return Err(RouterError::ExcessiveInputAmount);
        }
        let first_pair = self.pair_for(path[0], path[1]).await?;
        if first_pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self._transfer_from(path[0], msg::source(), first_pair, amounts[0]).await?;
        self._swap(amounts.clone(), path.clone(), exec::program_id()).await?;
        let amount_vara_out = amounts[amounts.len() - 1];
        let _ = self._unwrap_vara(amount_vara_out).await?;
        let _ = msg::send_bytes(to, b"Transfer Vara", amount_vara_out.as_u128());
        self.notify_on(RouterEvent::SwapTokensForExactVARA { amount_out: amount_vara_out, amount_in: amounts[0], path: path.clone(), to }).unwrap();

        Ok(amounts)
    }

    pub async fn swap_exact_tokens_for_vara(
        &mut self,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<ActorId>,
        to: ActorId,
        deadline: u64,
    ) -> Result<Vec<U256>, RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        let amounts = self.get_amounts_out(amount_in, path.clone()).await?;
        let amount_vara_out = amounts[amounts.len() - 1];
        if amount_vara_out < amount_out_min {
            return Err(RouterError::InsufficientOutputAmount);
        }
        let first_pair = self.pair_for(path[0], path[1]).await?;
        if first_pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self._transfer_from(path[0], msg::source(), first_pair, amount_in).await?;
        self._swap(amounts.clone(), path.clone(), exec::program_id()).await?;

        let _ = self._unwrap_vara(amount_vara_out).await?;
        let _ = msg::send_bytes(to, b"Transfer Vara", amount_vara_out.as_u128());
        self.notify_on(RouterEvent::SwapExactTokensForVARA { amount_in, amount_out: amount_vara_out, path, to }).unwrap();

        Ok(amounts)
    }

    pub async fn swap_vara_for_exact_tokens(
        &mut self,
        amount_out: U256,
        path: Vec<ActorId>,
        to: ActorId,
        deadline: u64,
    ) -> Result<Vec<U256>, RouterError> {
        if deadline < exec::block_timestamp() {
            return Err(RouterError::Expired);
        }
        
        let router_state = RouterState::get();
        let wrapped_vara = router_state.wvara_address;
        let vara_amount = msg::value();

        let amounts = self.get_amounts_in(amount_out, path.clone()).await?;
        if amounts[0].as_u128() > vara_amount {
            return Err(RouterError::ExcessiveInputAmount);
        };

        let _ = self._wrap_vara(amounts[0].as_u128()).await?;
        let first_pair = self.pair_for(wrapped_vara, path[1]).await?;
        if first_pair.is_zero() {
            return Err(RouterError::PairNotFound);
        }
        let _ = self._transfer(wrapped_vara, first_pair, amounts[0]).await?;

        self._swap(amounts.clone(), path.clone(), to).await?;
        let amount_out = amounts[amounts.len()-1];

        // Refund excess VARA
        if vara_amount > amounts[0].as_u128() {
            let refund_amount = vara_amount - amounts[0].as_u128();
            let _ = msg::send_bytes(to, b"Transfer Vara", refund_amount);
        }
        self.notify_on(RouterEvent::SwapVARAForExactTokens { amount_out , amount_in: amounts[0], path: path.clone(), to }).unwrap();

        Ok(amounts)
    }





}
