
// #![no_std]
use sails_rs::{collections::*,prelude::*};

pub static mut LP_STAKING: Option<StateLpStaking> = None;

#[derive(Debug, Default)]
pub struct StateLpStaking {
    pub total_user:u64,
    pub total_amount:U256,
    pub acc_x_per_share: U256,
    pub x_per_second:U256,
    pub minimum_deposit_amount:U256,
    pub last_reward_timestamp:u64,
    pub end_timestamp:u64,
    pub staked_token:ActorId,
    pub reward_token:ActorId,
    pub admin:ActorId,
    pub user_info:HashMap<ActorId,UserInfo>,
    pub precision_factor : U256,
    pub lock : bool
}
#[derive(Encode, Decode, TypeInfo, Debug, Clone, Copy)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct UserInfo {
    pub amount:U256,
    pub reward_debt:U256,
    pub unclaimed_reward:U256,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PoolStakingInfo {
   pub total_user:u64,
   pub total_amount:U256,
   pub acc_x_per_share: U256,
   pub x_per_second:U256,
   pub minimum_deposit_amount:U256,
   pub last_reward_timestamp:u64,
   pub end_timestamp:u64,
   pub staked_token:ActorId,
   pub reward_token:ActorId,
   pub admin:ActorId,
   pub precision_factor : U256
}


impl StateLpStaking {
    pub fn get_mut() -> &'static mut Self {
        unsafe { LP_STAKING.as_mut().expect("State LpStaking Error") }
    }
    pub fn get() -> &'static Self {
        unsafe { LP_STAKING.as_ref().expect("State LpStaking Error") }
    }
}



#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum LpStakingEvent {
    Deposit {
        user:ActorId,
        amount:U256,
        total_lp_staked:U256,
        staked_token:ActorId
    },
    Withdraw {
        user:ActorId,
        amount:U256,
        total_lp_staked:U256,
        staked_token:ActorId
    },
    TokenRecovery {
        token:ActorId,
        amount:U256
    }

}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum LpStakingError {
    ErrorNotAdmin,
    ErrorCoinNotPublished,
    ErrorInvalidLpToken,
    ErrorLpTokenExist,
    ErrorWithdrawInsufficient,
    ErrorInvalidMoveRate,
    ErrorPidNotExist,
    ErrorCoinNotRegistered,
    ErrorMoveRewardOverflow,
    ErrorInvalidCoinDecimal,
    ErrorPoolUserInfoNotExist,
    ErrorZeroAccount,
    ErrorUpkeepElapsedOverCap,
    ErrorInputBalance,
    EPoolStillLive,
    EConnectToken,
    ETransferTokenFailed,
    TransferTokenFromFailed,
    TransferTokenFailed,
    TransferFromLiquidityFailed,
    EAmountWithdrawToHight,
    TransferLiquidityFailed,
    EPoolEnd,
    LPStakingStatusIncorrect,
    ErrorInsufficientBalance
}