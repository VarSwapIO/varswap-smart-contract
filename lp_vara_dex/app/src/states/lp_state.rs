use sails_rs::prelude::*;

pub static mut LP: Option<StateLp> = None;

pub const MINIMUM_LIQUIDITY: u128 = 10u128.pow(3);

#[derive(Debug, Default)]
pub struct StateLp {
    pub name:String,
    pub symbol:String,
    pub decimals:u8,
    pub factory: ActorId,
    pub token: (ActorId, ActorId),
    pub reserve: (U256, U256),
    pub cumulative_price: (U256, U256),
    pub last_block_ts: u64,
    pub k_last: U256,
    
}

impl StateLp {
    pub fn get_mut() -> &'static mut Self {
        unsafe { LP.as_mut().expect("State Lp Error") }
    }
    pub fn get() -> &'static Self {
        unsafe { LP.as_ref().expect("State Lp Error") }
    }
}
#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LPEvent {
  
    Mint {
        sender: ActorId,
        amount:(U256,U256)
    },
  
   Burn {
        sender: ActorId,
        amount:(U256,U256),
        to:ActorId
   },
    /// Should be returned from
    /// [`InnerAction::SwapExactTokensForTokens`]/[`InnerAction::SwapTokensForExactTokens`].
    Swap {
       sender:ActorId,
       amount_in:(U256,U256),
       amount_out:(U256,U256),
       to:ActorId
    },
    GetReserves {
        reserve_a: u128,
        reserve_b: u128,
        block_timestamp_last: u64,
    },
    /// Should be returned from [`InnerAction::Sync`].
    Sync {
        /// The current amount of the A token in the contract's reserve.
        reserve_a: U256,
        /// The current amount of the B token in the contract's reserve.
        reserve_b: U256,
    },
    /// Should be returned from [`InnerAction::Skim`].
    Skim {
        /// A skimmed amount of the A token.
        amount_a: U256,
        /// A skimmed amount of the A token.
        amount_b: U256,
        /// A recipient of skimmed tokens.
        to: ActorId,
    },
}
#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LPError {

    /// An insufficient amount of the A or B token was provided.
    InsufficientAmount,
    /// A specified amount limit of the former tokens has been exceeded.
    InsufficientFormerAmount,
    /// A specified amount limit of the latter tokens has been exceeded.
    InsufficientLatterAmount,
    /// An insufficient amount of liquidity tokens was provided, or the contract
    /// doesn't have enough of them to continue an action.
    InsufficientLiquidity,
    /// An invalid recipient was specified.
    InvalidRecipient,
    ZeroActorId,
    /// One of the contract's FT contracts failed to complete a transfer
    /// action.
    ///
    /// Most often, the reason is that a user didn't give an approval to the
    /// contract or didn't have enough tokens to transfer.
    TransferFailed,
    /// An overflow occurred during calculations.
    Overflow,
    /// A specified deadline for an action was exceeded.
    DeadlineExceeded,
    IdenticalTokens,
    /// linked Factory contract.
    FeeToGettingFailed,
    InvalidTokens,
    InvalidRouter,
    CanNotConnectToVft,
    InsufficientLiquidityMinted,
    InsufficientLiquidityBurned,
    InsufficientOutputAmount,
    InsufficientInputAmount,
    KConstant,
    InvalidTo,
    CanNotConnectToFactory,
}

