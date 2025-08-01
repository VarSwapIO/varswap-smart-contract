// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct FactoryVaraDexFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}
impl<R> FactoryVaraDexFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::FactoryVaraDexFactory for FactoryVaraDexFactory<R> {
    type Args = R::Args;
    fn new(
        &self,
        code_id_pair: CodeId,
        fee_to: ActorId,
        fee_to_setter: ActorId,
        admin: ActorId,
    ) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, factory_vara_dex_factory::io::New>::new(
            self.remoting.clone(),
            (code_id_pair, fee_to, fee_to_setter, admin),
        )
    }
}

pub mod factory_vara_dex_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());
        impl New {
            #[allow(dead_code)]
            pub fn encode_call(
                code_id_pair: CodeId,
                fee_to: ActorId,
                fee_to_setter: ActorId,
                admin: ActorId,
            ) -> Vec<u8> {
                <New as ActionIo>::encode_call(&(code_id_pair, fee_to, fee_to_setter, admin))
            }
        }
        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = (CodeId, ActorId, ActorId, ActorId);
            type Reply = ();
        }
    }
}
pub struct FactoryService<R> {
    remoting: R,
}
impl<R> FactoryService<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::FactoryService for FactoryService<R> {
    type Args = R::Args;
    fn add_bridged_asset(
        &mut self,
        token_address: ActorId,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> impl Call<Output = Result<BridgedAsset, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::AddBridgedAsset>::new(
            self.remoting.clone(),
            (token_address, name, symbol, decimals),
        )
    }
    fn add_pair(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
        pair_address: ActorId,
    ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::AddPair>::new(
            self.remoting.clone(),
            (token_a, token_b, pair_address),
        )
    }
    fn create_pair(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
    ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::CreatePair>::new(
            self.remoting.clone(),
            (token_a, token_b),
        )
    }
    fn remove_bridged_asset(
        &mut self,
        token_address: ActorId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::RemoveBridgedAsset>::new(
            self.remoting.clone(),
            token_address,
        )
    }
    fn remove_pair(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::RemovePair>::new(
            self.remoting.clone(),
            (token_a, token_b),
        )
    }
    fn set_admin(
        &mut self,
        new_admin: ActorId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::SetAdmin>::new(self.remoting.clone(), new_admin)
    }
    fn set_fee_to(
        &mut self,
        new_fee_to: ActorId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::SetFeeTo>::new(self.remoting.clone(), new_fee_to)
    }
    fn set_fee_to_setter(
        &mut self,
        new_fee_setter: ActorId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::SetFeeToSetter>::new(
            self.remoting.clone(),
            new_fee_setter,
        )
    }
    fn set_router(
        &mut self,
        router: ActorId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::SetRouter>::new(self.remoting.clone(), router)
    }
    fn update_code_id_pair(
        &mut self,
        new_code_id_pair: CodeId,
    ) -> impl Call<Output = Result<(), FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::UpdateCodeIdPair>::new(
            self.remoting.clone(),
            new_code_id_pair,
        )
    }
    fn get_admin(&self) -> impl Query<Output = ActorId, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetAdmin>::new(self.remoting.clone(), ())
    }
    fn get_all_pairs(&self) -> impl Query<Output = Vec<(ActorId, ActorId)>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetAllPairs>::new(self.remoting.clone(), ())
    }
    fn get_all_pairs_address(&self) -> impl Query<Output = Vec<ActorId>, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetAllPairsAddress>::new(self.remoting.clone(), ())
    }
    fn get_code_id_pair(&self) -> impl Query<Output = CodeId, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetCodeIdPair>::new(self.remoting.clone(), ())
    }
    fn get_fee_to(&self) -> impl Query<Output = ActorId, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetFeeTo>::new(self.remoting.clone(), ())
    }
    fn get_fee_to_setter(&self) -> impl Query<Output = ActorId, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetFeeToSetter>::new(self.remoting.clone(), ())
    }
    fn get_pair(
        &self,
        token_a: ActorId,
        token_b: ActorId,
    ) -> impl Query<Output = ActorId, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetPair>::new(
            self.remoting.clone(),
            (token_a, token_b),
        )
    }
    fn get_pair_length(&self) -> impl Query<Output = u64, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetPairLength>::new(self.remoting.clone(), ())
    }
    fn get_router(&self) -> impl Query<Output = ActorId, Args = R::Args> {
        RemotingAction::<_, factory_service::io::GetRouter>::new(self.remoting.clone(), ())
    }
}

pub mod factory_service {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AddBridgedAsset(());
        impl AddBridgedAsset {
            #[allow(dead_code)]
            pub fn encode_call(
                token_address: ActorId,
                name: String,
                symbol: String,
                decimals: u8,
            ) -> Vec<u8> {
                <AddBridgedAsset as ActionIo>::encode_call(&(token_address, name, symbol, decimals))
            }
        }
        impl ActionIo for AddBridgedAsset {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 60, 65, 100,
                100, 66, 114, 105, 100, 103, 101, 100, 65, 115, 115, 101, 116,
            ];
            type Params = (ActorId, String, String, u8);
            type Reply = Result<super::BridgedAsset, super::FactoryError>;
        }
        pub struct AddPair(());
        impl AddPair {
            #[allow(dead_code)]
            pub fn encode_call(
                token_a: ActorId,
                token_b: ActorId,
                pair_address: ActorId,
            ) -> Vec<u8> {
                <AddPair as ActionIo>::encode_call(&(token_a, token_b, pair_address))
            }
        }
        impl ActionIo for AddPair {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 28, 65, 100,
                100, 80, 97, 105, 114,
            ];
            type Params = (ActorId, ActorId, ActorId);
            type Reply = Result<ActorId, super::FactoryError>;
        }
        pub struct CreatePair(());
        impl CreatePair {
            #[allow(dead_code)]
            pub fn encode_call(token_a: ActorId, token_b: ActorId) -> Vec<u8> {
                <CreatePair as ActionIo>::encode_call(&(token_a, token_b))
            }
        }
        impl ActionIo for CreatePair {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 40, 67, 114,
                101, 97, 116, 101, 80, 97, 105, 114,
            ];
            type Params = (ActorId, ActorId);
            type Reply = Result<ActorId, super::FactoryError>;
        }
        pub struct RemoveBridgedAsset(());
        impl RemoveBridgedAsset {
            #[allow(dead_code)]
            pub fn encode_call(token_address: ActorId) -> Vec<u8> {
                <RemoveBridgedAsset as ActionIo>::encode_call(&token_address)
            }
        }
        impl ActionIo for RemoveBridgedAsset {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 72, 82, 101,
                109, 111, 118, 101, 66, 114, 105, 100, 103, 101, 100, 65, 115, 115, 101, 116,
            ];
            type Params = ActorId;
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct RemovePair(());
        impl RemovePair {
            #[allow(dead_code)]
            pub fn encode_call(token_a: ActorId, token_b: ActorId) -> Vec<u8> {
                <RemovePair as ActionIo>::encode_call(&(token_a, token_b))
            }
        }
        impl ActionIo for RemovePair {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 40, 82, 101,
                109, 111, 118, 101, 80, 97, 105, 114,
            ];
            type Params = (ActorId, ActorId);
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct SetAdmin(());
        impl SetAdmin {
            #[allow(dead_code)]
            pub fn encode_call(new_admin: ActorId) -> Vec<u8> {
                <SetAdmin as ActionIo>::encode_call(&new_admin)
            }
        }
        impl ActionIo for SetAdmin {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 32, 83, 101,
                116, 65, 100, 109, 105, 110,
            ];
            type Params = ActorId;
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct SetFeeTo(());
        impl SetFeeTo {
            #[allow(dead_code)]
            pub fn encode_call(new_fee_to: ActorId) -> Vec<u8> {
                <SetFeeTo as ActionIo>::encode_call(&new_fee_to)
            }
        }
        impl ActionIo for SetFeeTo {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 32, 83, 101,
                116, 70, 101, 101, 84, 111,
            ];
            type Params = ActorId;
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct SetFeeToSetter(());
        impl SetFeeToSetter {
            #[allow(dead_code)]
            pub fn encode_call(new_fee_setter: ActorId) -> Vec<u8> {
                <SetFeeToSetter as ActionIo>::encode_call(&new_fee_setter)
            }
        }
        impl ActionIo for SetFeeToSetter {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 56, 83, 101,
                116, 70, 101, 101, 84, 111, 83, 101, 116, 116, 101, 114,
            ];
            type Params = ActorId;
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct SetRouter(());
        impl SetRouter {
            #[allow(dead_code)]
            pub fn encode_call(router: ActorId) -> Vec<u8> {
                <SetRouter as ActionIo>::encode_call(&router)
            }
        }
        impl ActionIo for SetRouter {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 36, 83, 101,
                116, 82, 111, 117, 116, 101, 114,
            ];
            type Params = ActorId;
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct UpdateCodeIdPair(());
        impl UpdateCodeIdPair {
            #[allow(dead_code)]
            pub fn encode_call(new_code_id_pair: CodeId) -> Vec<u8> {
                <UpdateCodeIdPair as ActionIo>::encode_call(&new_code_id_pair)
            }
        }
        impl ActionIo for UpdateCodeIdPair {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 64, 85, 112,
                100, 97, 116, 101, 67, 111, 100, 101, 73, 100, 80, 97, 105, 114,
            ];
            type Params = CodeId;
            type Reply = Result<(), super::FactoryError>;
        }
        pub struct GetAdmin(());
        impl GetAdmin {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetAdmin as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetAdmin {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 32, 71, 101,
                116, 65, 100, 109, 105, 110,
            ];
            type Params = ();
            type Reply = ActorId;
        }
        pub struct GetAllPairs(());
        impl GetAllPairs {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetAllPairs as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetAllPairs {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 44, 71, 101,
                116, 65, 108, 108, 80, 97, 105, 114, 115,
            ];
            type Params = ();
            type Reply = Vec<(ActorId, ActorId)>;
        }
        pub struct GetAllPairsAddress(());
        impl GetAllPairsAddress {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetAllPairsAddress as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetAllPairsAddress {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 72, 71, 101,
                116, 65, 108, 108, 80, 97, 105, 114, 115, 65, 100, 100, 114, 101, 115, 115,
            ];
            type Params = ();
            type Reply = Vec<ActorId>;
        }
        pub struct GetCodeIdPair(());
        impl GetCodeIdPair {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetCodeIdPair as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetCodeIdPair {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 52, 71, 101,
                116, 67, 111, 100, 101, 73, 100, 80, 97, 105, 114,
            ];
            type Params = ();
            type Reply = CodeId;
        }
        pub struct GetFeeTo(());
        impl GetFeeTo {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetFeeTo as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetFeeTo {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 32, 71, 101,
                116, 70, 101, 101, 84, 111,
            ];
            type Params = ();
            type Reply = ActorId;
        }
        pub struct GetFeeToSetter(());
        impl GetFeeToSetter {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetFeeToSetter as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetFeeToSetter {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 56, 71, 101,
                116, 70, 101, 101, 84, 111, 83, 101, 116, 116, 101, 114,
            ];
            type Params = ();
            type Reply = ActorId;
        }
        pub struct GetPair(());
        impl GetPair {
            #[allow(dead_code)]
            pub fn encode_call(token_a: ActorId, token_b: ActorId) -> Vec<u8> {
                <GetPair as ActionIo>::encode_call(&(token_a, token_b))
            }
        }
        impl ActionIo for GetPair {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 28, 71, 101,
                116, 80, 97, 105, 114,
            ];
            type Params = (ActorId, ActorId);
            type Reply = ActorId;
        }
        pub struct GetPairLength(());
        impl GetPairLength {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetPairLength as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetPairLength {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 52, 71, 101,
                116, 80, 97, 105, 114, 76, 101, 110, 103, 116, 104,
            ];
            type Params = ();
            type Reply = u64;
        }
        pub struct GetRouter(());
        impl GetRouter {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetRouter as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetRouter {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101, 36, 71, 101,
                116, 82, 111, 117, 116, 101, 114,
            ];
            type Params = ();
            type Reply = ActorId;
        }
    }

    #[allow(dead_code)]
    #[cfg(not(target_arch = "wasm32"))]
    pub mod events {
        use super::*;
        use sails_rs::events::*;
        #[derive(PartialEq, Debug, Encode, Decode)]
        #[codec(crate = sails_rs::scale_codec)]
        pub enum FactoryServiceEvents {
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
            FeeToSetterSet(ActorId),
            /// Should be returned from [`Action::FeeTo`].
            FeeToSet(ActorId),
            Pair(ActorId),
            RouterSet(ActorId),
            AdminSet(ActorId),
            CodeIdPairUpdated(CodeId),
            PairRemoved {
                token_pair: (ActorId, ActorId),
            },
            BridgedAssetAdded {
                token_address: ActorId,
                name: String,
                symbol: String,
                decimals: u8,
            },
            BridgedAssetRemoved {
                token_address: ActorId,
            },
        }
        impl EventIo for FactoryServiceEvents {
            const ROUTE: &'static [u8] = &[
                56, 70, 97, 99, 116, 111, 114, 121, 83, 101, 114, 118, 105, 99, 101,
            ];
            const EVENT_NAMES: &'static [&'static [u8]] = &[
                &[44, 80, 97, 105, 114, 67, 114, 101, 97, 116, 101, 100],
                &[
                    56, 70, 101, 101, 84, 111, 83, 101, 116, 116, 101, 114, 83, 101, 116,
                ],
                &[32, 70, 101, 101, 84, 111, 83, 101, 116],
                &[16, 80, 97, 105, 114],
                &[36, 82, 111, 117, 116, 101, 114, 83, 101, 116],
                &[32, 65, 100, 109, 105, 110, 83, 101, 116],
                &[
                    68, 67, 111, 100, 101, 73, 100, 80, 97, 105, 114, 85, 112, 100, 97, 116, 101,
                    100,
                ],
                &[44, 80, 97, 105, 114, 82, 101, 109, 111, 118, 101, 100],
                &[
                    68, 66, 114, 105, 100, 103, 101, 100, 65, 115, 115, 101, 116, 65, 100, 100,
                    101, 100,
                ],
                &[
                    76, 66, 114, 105, 100, 103, 101, 100, 65, 115, 115, 101, 116, 82, 101, 109,
                    111, 118, 101, 100,
                ],
            ];
            type Event = Self;
        }
        pub fn listener<R: Listener<Vec<u8>>>(remoting: R) -> impl Listener<FactoryServiceEvents> {
            RemotingListener::<_, FactoryServiceEvents>::new(remoting)
        }
    }
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgedAsset {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum FactoryError {
    Unauthorized,
    UnexpectedFTEvent,
    MessageSendError,
    NotFound,
    PairExist,
    PairCreationFailed,
    PairNotExist,
    VFTError,
    BridgedAssetExist,
}

pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait FactoryVaraDexFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(
            &self,
            code_id_pair: CodeId,
            fee_to: ActorId,
            fee_to_setter: ActorId,
            admin: ActorId,
        ) -> impl Activation<Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait FactoryService {
        type Args;
        fn add_bridged_asset(
            &mut self,
            token_address: ActorId,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> impl Call<Output = Result<BridgedAsset, FactoryError>, Args = Self::Args>;
        fn add_pair(
            &mut self,
            token_a: ActorId,
            token_b: ActorId,
            pair_address: ActorId,
        ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = Self::Args>;
        fn create_pair(
            &mut self,
            token_a: ActorId,
            token_b: ActorId,
        ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = Self::Args>;
        fn remove_bridged_asset(
            &mut self,
            token_address: ActorId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn remove_pair(
            &mut self,
            token_a: ActorId,
            token_b: ActorId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn set_admin(
            &mut self,
            new_admin: ActorId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn set_fee_to(
            &mut self,
            new_fee_to: ActorId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn set_fee_to_setter(
            &mut self,
            new_fee_setter: ActorId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn set_router(
            &mut self,
            router: ActorId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn update_code_id_pair(
            &mut self,
            new_code_id_pair: CodeId,
        ) -> impl Call<Output = Result<(), FactoryError>, Args = Self::Args>;
        fn get_admin(&self) -> impl Query<Output = ActorId, Args = Self::Args>;
        fn get_all_pairs(&self) -> impl Query<Output = Vec<(ActorId, ActorId)>, Args = Self::Args>;
        fn get_all_pairs_address(&self) -> impl Query<Output = Vec<ActorId>, Args = Self::Args>;
        fn get_code_id_pair(&self) -> impl Query<Output = CodeId, Args = Self::Args>;
        fn get_fee_to(&self) -> impl Query<Output = ActorId, Args = Self::Args>;
        fn get_fee_to_setter(&self) -> impl Query<Output = ActorId, Args = Self::Args>;
        fn get_pair(
            &self,
            token_a: ActorId,
            token_b: ActorId,
        ) -> impl Query<Output = ActorId, Args = Self::Args>;
        fn get_pair_length(&self) -> impl Query<Output = u64, Args = Self::Args>;
        fn get_router(&self) -> impl Query<Output = ActorId, Args = Self::Args>;
    }
}

#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;

#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
pub mod mockall {
    use super::*;
    use sails_rs::mockall::*;
    mock! { pub FactoryService<A> {} #[allow(refining_impl_trait)] #[allow(clippy::type_complexity)] impl<A> traits::FactoryService for FactoryService<A> { type Args = A; fn add_bridged_asset (&mut self, token_address: ActorId,name: String,symbol: String,decimals: u8,) -> MockCall<A, Result<BridgedAsset, FactoryError>>;fn add_pair (&mut self, token_a: ActorId,token_b: ActorId,pair_address: ActorId,) -> MockCall<A, Result<ActorId, FactoryError>>;fn create_pair (&mut self, token_a: ActorId,token_b: ActorId,) -> MockCall<A, Result<ActorId, FactoryError>>;fn remove_bridged_asset (&mut self, token_address: ActorId,) -> MockCall<A, Result<(), FactoryError>>;fn remove_pair (&mut self, token_a: ActorId,token_b: ActorId,) -> MockCall<A, Result<(), FactoryError>>;fn set_admin (&mut self, new_admin: ActorId,) -> MockCall<A, Result<(), FactoryError>>;fn set_fee_to (&mut self, new_fee_to: ActorId,) -> MockCall<A, Result<(), FactoryError>>;fn set_fee_to_setter (&mut self, new_fee_setter: ActorId,) -> MockCall<A, Result<(), FactoryError>>;fn set_router (&mut self, router: ActorId,) -> MockCall<A, Result<(), FactoryError>>;fn update_code_id_pair (&mut self, new_code_id_pair: CodeId,) -> MockCall<A, Result<(), FactoryError>>;fn get_admin (& self, ) -> MockQuery<A, ActorId>;fn get_all_pairs (& self, ) -> MockQuery<A, Vec<(ActorId,ActorId,)>>;fn get_all_pairs_address (& self, ) -> MockQuery<A, Vec<ActorId>>;fn get_code_id_pair (& self, ) -> MockQuery<A, CodeId>;fn get_fee_to (& self, ) -> MockQuery<A, ActorId>;fn get_fee_to_setter (& self, ) -> MockQuery<A, ActorId>;fn get_pair (& self, token_a: ActorId,token_b: ActorId,) -> MockQuery<A, ActorId>;fn get_pair_length (& self, ) -> MockQuery<A, u64>;fn get_router (& self, ) -> MockQuery<A, ActorId>; } }
}
