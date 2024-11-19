use sails_rs::{collections::HashMap, prelude::*};

pub static mut FACTORY: Option<StateFactory> = None;

#[derive(Debug, Default)]
pub struct StateFactory {
    pub code_id_pair: CodeId,
    pub fee_to: ActorId,
    pub fee_to_setter: ActorId,
    pub pairs: HashMap<(ActorId, ActorId), ActorId>,
}

impl StateFactory {
    pub fn get_mut() -> &'static mut Self {
        unsafe { FACTORY.as_mut().expect("State Factory Error") }
    }
    pub fn get() -> &'static Self {
        unsafe { FACTORY.as_ref().expect("State Factory Error") }
    }
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitPair {
    pub factory:ActorId, 
    pub token_a:ActorId, 
    pub token_b:ActorId, 
    pub name:String, 
    pub symbol:String, 
    pub decimals:u8
}


#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    /// Should be returned from [`Action::CreatePair`].
    PairCreated {
        /// A pair of SFT [`ActorId`]s.
        token_pair: (ActorId, ActorId),
        /// [`ActorId`] of a created Pair contract.
        pair_address: ActorId,
        /// A number of Pair contracts (including a created one) inside the
        /// Factory contract.
        pair_number: u64,
    },

    /// Should be returned from [`Action::FeeToSetter`].
    FeeToSetterSet(
        /// New `fee_to_setter`.
        ActorId,
    ),

    /// Should be returned from [`Action::FeeTo`].
    FeeToSet(
        /// New `fee_to`.
        ActorId,
    ),
    Pair(ActorId),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryError {
    Unauthorized,
    UnexpectedFTEvent,
    MessageSendError,
    NotFound,
    PairExist,
    PairCreationFailed,
    PairNotExist,
    VFTError,
}
