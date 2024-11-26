
use sails_rs::calls::{Call, Query};
use sails_rs::gstd::calls::GStdRemoting;
use sails_rs::gstd::exec::{self, block_timestamp};
use sails_rs::{
    prelude::*,
    collections::*,
    gstd::msg
};

use crate::states::lp_staking_state::{LpStakingError, LpStakingEvent, StateLpStaking, UserInfo, LP_STAKING, PoolStakingInfo};
use crate::clients::extended_new_vft::Vft as VftClient;
use crate::clients::extended_new_vft::traits::Vft;
use crate::clients::lp_vara_dex_client::LpService as LpServiceClient;
use crate::clients::lp_vara_dex_client::traits::LpService;


pub struct LpStakingService {
    pub vft_client: VftClient<GStdRemoting>,
    pub lp_client: LpServiceClient<GStdRemoting>,
}

impl LpStakingService {

    pub fn seed(end_time:u64, staked_token:ActorId, reward_token:ActorId,x_per_second:U256, admin:ActorId){
        unsafe {
            LP_STAKING = Some(StateLpStaking { 
                total_user: 0, 
                total_amount: U256::from(0), 
                acc_x_per_share: U256::from(0), 
                x_per_second, 
                last_reward_timestamp: block_timestamp()/1000, 
                end_timestamp: end_time, 
                staked_token, 
                reward_token,
                admin,
                user_info : HashMap::new(),
                precision_factor: U256::exp10(18) 
            }
            )
        }
    }
}
#[sails_rs::service(events = LpStakingEvent)]
impl LpStakingService {

    pub fn new(
        vft_client: VftClient<GStdRemoting>, 
        lp_client: LpServiceClient<GStdRemoting>) -> Self {
        Self { vft_client, lp_client }
    }

    // admin function
    pub fn set_admin(&mut self, new_admin:ActorId) -> Result<bool, LpStakingError>{
        let state = StateLpStaking::get_mut();
        let sender = msg::source();
        if sender != state.admin {
            return Err(LpStakingError::ErrorNotAdmin);
        }
        state.admin = new_admin;
        Ok(true)
    }

    pub async fn recover_token(&mut self, token:ActorId) -> Result<bool,LpStakingError> {
        let state = StateLpStaking::get_mut();
        let sender = msg::source();
        if sender != state.admin {
            return Err(LpStakingError::ErrorNotAdmin);
        }

        let get_balance_res = self.vft_client.balance_of(exec::program_id()).recv(token).await;
        let Ok(token_balance) = get_balance_res else {
            return Err(LpStakingError::EConnectToken);
        };

        let _ = self._transfer(token, sender, token_balance).await;

        // let transfer_res = self.vft_client.transfer(sender, token_balance).send_recv(token).await;
        // let Ok(transfer_status) = transfer_res else {
        //     return Err(LpStakingError::ETransferTokenFailed);
        // };
        // if !transfer_status {
        //     return Err(LpStakingError::ETransferTokenFailed);
        // }
        self.notify_on(LpStakingEvent::TokenRecovery { token, amount: token_balance }).unwrap();
        Ok(true)
    }

    pub fn update_end_pool(&mut self, new_time_end:u64) -> Result<bool, LpStakingError> {
        let state = StateLpStaking::get_mut();
        let sender = msg::source();
        if sender != state.admin {
            return Err(LpStakingError::ErrorNotAdmin);
        }

        state.end_timestamp = new_time_end;

        Ok(true)
    }

    pub fn update_reward_per_second(&mut self, new_reward_per_second:U256) -> Result<bool, LpStakingError> {
        let state = StateLpStaking::get_mut();
        let sender = msg::source();
        if sender != state.admin {
            return Err(LpStakingError::ErrorNotAdmin);
        }

        state.x_per_second = new_reward_per_second;

        Ok(true)
    }

    pub fn change_reward_token(&mut self, new_reward_token:ActorId) -> Result<bool, LpStakingError> {
        let state = StateLpStaking::get_mut();
        let sender = msg::source();
        if sender != state.admin {
            return Err(LpStakingError::ErrorNotAdmin);
        }

        state.reward_token = new_reward_token;

        Ok(true)
    }


    // private function

    async fn _transfer_from(&mut self, token:ActorId, from:ActorId, to:ActorId, value:U256) -> Result<(),LpStakingError>{
        let send_token_res = self.vft_client.transfer_from( from, to, value).send_recv(token).await;
        let Ok(transfer_token_status) = send_token_res else {
            return Err(LpStakingError::TransferTokenFromFailed);
        };
        if !transfer_token_status {
            return Err(LpStakingError::TransferTokenFromFailed);
        }else{
            Ok(())
        }
        
    }

    async fn _transfer(&mut self, token:ActorId, to: ActorId, value:U256) -> Result<(),LpStakingError>{
        let transfer_wvara_res =  self.vft_client.transfer(to, value).send_recv(token).await;
        let Ok(transfer_wvara_status) = transfer_wvara_res else {
            return Err(LpStakingError::TransferTokenFailed);
        };
        if !transfer_wvara_status {
            return Err(LpStakingError::TransferTokenFailed);
        }else {
            Ok(())
        }
        
    }

    async fn transfer_from_liquidity(&mut self, pair:ActorId, from:ActorId, to:ActorId, liquidity:U256) -> Result<(),LpStakingError>{
        let transfer_liquidity_res = self.lp_client.transfer_from(from, to, liquidity).send_recv(pair).await;
        let Ok(transfer_liquidity_status) = transfer_liquidity_res else {
            return Err(LpStakingError::TransferFromLiquidityFailed);
        };
        if !transfer_liquidity_status {
            return Err(LpStakingError::TransferFromLiquidityFailed);
        }else{
            Ok(())
        }
    }

    async fn transfer_liquidity(&mut self, pair:ActorId, to:ActorId, liquidity:U256) -> Result<(),LpStakingError>{
        let transfer_liquidity_res = self.lp_client.transfer(to, liquidity).send_recv(pair).await;
        let Ok(transfer_liquidity_status) = transfer_liquidity_res else {
            return Err(LpStakingError::TransferLiquidityFailed);
        };
        if !transfer_liquidity_status {
            return Err(LpStakingError::TransferLiquidityFailed);
        }else{
            Ok(())
        }
        
    }

    // user function

    pub async fn deposit(&mut self, amount:U256) -> Result<bool, LpStakingError> {

        let sender = msg::source();
        let state = StateLpStaking::get_mut();
        if !state.user_info.contains_key(&sender){
            state.user_info.insert(sender, UserInfo{
                amount:U256::zero(),
                reward_debt:U256::zero()
            });

            state.total_user = state.total_user + 1;
        };

        if state.end_timestamp < block_timestamp() /1000 {
            return Err(LpStakingError::EPoolEnd);
        }

        let user_info = state.user_info.get_mut(&sender).unwrap();

        // update_pool
        let total_supply_res = self.lp_client.balance_of(exec::program_id()).recv(state.staked_token).await;
        let Ok(staked_token_supply) = total_supply_res else {
            return Err(LpStakingError::EConnectToken);
        };
        if staked_token_supply > U256::zero() {
            state.acc_x_per_share = state.acc_x_per_share + (state.x_per_second * state.precision_factor) / staked_token_supply;
        };
        state.last_reward_timestamp = block_timestamp()/1000;

        //
        if user_info.amount > U256::zero() {
            let pending = (user_info.amount * state.acc_x_per_share) / state.precision_factor - user_info.reward_debt;
            if pending > U256::zero() {
               let transfer_token_res = self._transfer(state.reward_token, sender, pending).await;
               if transfer_token_res.is_err(){
                return Err(transfer_token_res.err().unwrap());
               }
            };
        }

        if amount > U256::zero() {
            let transfer_lp_res = self.transfer_from_liquidity(state.staked_token, sender, exec::program_id(), amount).await;
            if transfer_lp_res.is_ok() {
                user_info.amount = user_info.amount + amount;
                state.total_amount = state.total_amount + amount;
            }else{
                return Err(transfer_lp_res.err().unwrap());
            }
        }

        user_info.reward_debt = (user_info.amount * state.acc_x_per_share) / state.precision_factor;

        self.notify_on(LpStakingEvent::Deposit { user: sender, amount, total_lp_staked: user_info.amount, staked_token:state.staked_token }).unwrap();

        Ok(true)
    }

    pub async fn withdraw(&mut self, _amount:U256) -> Result<bool, LpStakingError> {
        let sender = msg::source();
        let state = StateLpStaking::get_mut();
        let user_info = state.user_info.get_mut(&sender).unwrap();

        if user_info.amount < _amount {
            return Err(LpStakingError::EAmountWithdrawToHight);
        }
         // update_pool
        let total_supply_res = self.lp_client.balance_of(exec::program_id()).recv(state.staked_token).await;
        let Ok(staked_token_supply) = total_supply_res else {
             return Err(LpStakingError::EConnectToken);
        };
        if staked_token_supply > U256::zero() {
            state.acc_x_per_share = state.acc_x_per_share + (state.x_per_second * state.precision_factor) / staked_token_supply;
        };
        state.last_reward_timestamp = block_timestamp()/1000;

        //

        let pending = (user_info.amount * state.acc_x_per_share) / state.precision_factor - user_info.reward_debt;
        if _amount > U256::zero() { 
            let transfer_lp_res = self.transfer_liquidity(state.staked_token, sender, _amount).await;
            if transfer_lp_res.is_ok() {
                user_info.amount = user_info.amount - _amount;
                state.total_amount = state.total_amount - _amount;
            }else {
                return Err(transfer_lp_res.err().unwrap());
            }
        };

        if pending > U256::zero() {
            let transfer_token_res = self._transfer(state.reward_token, sender, pending).await;
               if transfer_token_res.is_err(){
                return Err(transfer_token_res.err().unwrap());
               }
        };

        user_info.reward_debt = (user_info.amount * state.acc_x_per_share) / state.precision_factor;
        self.notify_on(LpStakingEvent::Withdraw { user: sender, amount: _amount, total_lp_staked: user_info.amount, staked_token: state.staked_token }).unwrap();

        Ok(true)
    }


    //view function
    pub async  fn pending_reward(&self, _user:ActorId) -> U256{
        let state = StateLpStaking::get();
        let user_info = state.user_info.get(&_user).unwrap();
        let total_supply_res = self.lp_client.balance_of(exec::program_id()).recv(state.staked_token).await;
        let Ok(staked_token_supply) = total_supply_res else {
             return U256::zero();
        };
        if block_timestamp()/ 1000 > state.last_reward_timestamp && staked_token_supply != U256::zero() {
            let adjusted_token_per_share = state.acc_x_per_share + (state.x_per_second * state.precision_factor) / staked_token_supply;
            return (user_info.amount * adjusted_token_per_share) / state.precision_factor - user_info.reward_debt;
        }else{
            return (user_info.amount * state.acc_x_per_share) / state.precision_factor - user_info.reward_debt;
        }
    }

    pub fn user_info(&self, _user:ActorId) -> UserInfo {
        let state = StateLpStaking::get();
        if !state.user_info.contains_key(&_user){
            UserInfo{
                amount:U256::zero(),
                reward_debt:U256::zero()
            }
        }else{
            let info = state.user_info.get(&_user).unwrap();
            UserInfo{
                amount:info.amount,
                reward_debt:info.reward_debt
            }
        }
    }

    pub fn pool_info(&self) -> PoolStakingInfo {
        let state = StateLpStaking::get();

        let mut  users_info = Vec::new();
        for val in state.user_info.values() {
            users_info.push(*val);
        }

       PoolStakingInfo {
            total_user: state.total_user, 
            total_amount: state.total_amount, 
            acc_x_per_share: state.acc_x_per_share, 
            x_per_second:state.x_per_second, 
            last_reward_timestamp: state.last_reward_timestamp, 
            end_timestamp:state.end_timestamp, 
            staked_token:state.staked_token, 
            reward_token:state.reward_token,
            admin:state.admin,
            users_info,
            precision_factor: state.precision_factor 
        }
    }

    pub async fn check_liquidity_balance(&self,_user:ActorId) -> U256 {
        let state = StateLpStaking::get();
        let res = self.lp_client.balance_of(_user).recv(state.staked_token).await;
        let Ok(balance) = res else {
            return U256::zero();
        };
        balance
    }

    pub async fn check_reward_balance(&self,_user:ActorId) -> U256 {
        let state = StateLpStaking::get();
        let res = self.vft_client.balance_of(_user).recv(state.reward_token).await;
        let Ok(balance) = res else {
            return U256::zero();
        };
        balance
    }

    // pub async fn test_transfer_liquidity(&mut self,_to:ActorId,_amount:U256) -> Result<bool, LpStakingError> {
    //     let state = StateLpStaking::get_mut();
    //     let res = self.transfer_liquidity(state.staked_token, _to, _amount).await;

    //     if res.is_ok(){
    //         Ok(true)
    //     }else{
    //         return Err(res.err().unwrap());
    //     }
    // }

    // pub async fn test_transfer_from_liquidity(&mut self,_amount:U256) -> Result<bool, LpStakingError> {
    //     let state = StateLpStaking::get_mut();
    //     let sender = msg::source();
    //     let res = self.transfer_from_liquidity(state.staked_token, sender, exec::program_id(),_amount).await;
    //     if res.is_ok(){
    //         Ok(true)
    //     }else{
    //         return Err(res.err().unwrap());
    //     }
    // }

}