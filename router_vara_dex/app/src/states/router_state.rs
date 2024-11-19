use sails_rs::{prelude::*};


pub static mut ROUTER: Option<RouterState> = None;

#[derive(Debug, Default)]
pub struct RouterState {
  pub factory_address: ActorId,
  pub wvara_address: ActorId,
  pub admin:ActorId,
  pub fund_addr:ActorId,
  pub swap_fee_bps:u128
}

impl RouterState {
    pub fn get_mut() -> &'static mut Self {
        unsafe { ROUTER.as_mut().expect("State Router Error") }
    }
    pub fn get() -> &'static Self {
        unsafe { ROUTER.as_ref().expect("State Factory Error") }
    }
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RouterEvent {
    CreatePair {
        token_a: ActorId,
        token_b: ActorId,
        pair_address: ActorId,
    },
   AddLiquidity {
    token_a: ActorId,
    token_b: ActorId,
    amount_a: U256,
    amount_b: U256,
    to: ActorId,
    liquidity: U256,
   },
   AddLiquidityVARA {
    token_a: ActorId,
    amount_a: U256,
    amount_vara: U256,
    to: ActorId,
    liquidity: U256,
   },
   RemoveLiquidity {
    token_a: ActorId,
    token_b: ActorId,
    amount_a_received: U256,
    amount_b_received: U256,
    to: ActorId,
    liquidity: U256,
   },
   RemoveLiquidityVARA {
    token_a: ActorId,
    amount_a_received: U256,
    amount_vara_received: U256,
    to: ActorId,
    liquidity: U256,
   },
   SwapExactTokensForTokens {
    amount_in: U256,
    amount_out: U256,
    path: Vec<ActorId>,
    to: ActorId,
   },
   SwapTokensForExactTokens {
    amount_out: U256,
    amount_in: U256,
    path: Vec<ActorId>,
    to: ActorId,
   },
   SwapExactVARAForTokens {
    amount_in: U256,
    amount_out: U256,
    path: Vec<ActorId>,
    to: ActorId,
   },
   SwapTokensForExactVARA {
    amount_out: U256,
    amount_in: U256,
    path: Vec<ActorId>,
    to: ActorId,
   },
   SwapExactTokensForVARA {
    amount_in: U256,
    amount_out: U256,
    path: Vec<ActorId>,
    to: ActorId,
   },
   SwapVARAForExactTokens {
    amount_out: U256,
    amount_in: U256,
    path: Vec<ActorId>,
    to: ActorId,
   }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RouterError {
    PairAlreadyExists,
    TransferLiquidityFailed,
    TransferFromLiquidityFailed,
    TransferFromFailed,
    InsufficientFee,
    BurnLiquidityFailed,
    InsufficientVaraAmount,
    InsufficientTokenAmount,
    CreatePairFailed,
    WithdrawWvaraFailed,
    DepositWVARAFailed,
    SwapFailed,
    MintLiquidityFailed,
    Expired,
    PairNotFound,
    IdenticalAddresses,
    ZeroAddress,
    InsufficientBAmount,
    InsufficientAAmount,
    InsufficientLiquidity,
    InvalidPath,
    InsufficientOutputAmount,
    InsufficientInputAmount,
    InvalidLiquidityAmount,
    ExcessiveInputAmount,
    TransferFailed,
}