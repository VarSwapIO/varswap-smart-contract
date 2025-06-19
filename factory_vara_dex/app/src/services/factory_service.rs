use core::str::FromStr;

use gstd::exec;
use gstd::prog::ProgramGenerator;
use sails_rs::calls::{Call, Query};
use sails_rs::gstd::calls::GStdRemoting;
use sails_rs::{collections::HashMap, gstd::msg, prelude::*};

use crate::clients::extended_new_vft::traits::Vft;
use crate::clients::extended_new_vft::Vft as VftClient;
use crate::states::factory_state::{FactoryError, FactoryEvent, InitPair, StateFactory, FACTORY, BridgedAsset};

pub struct FactoryService {
    pub vft_client: VftClient<GStdRemoting>,
}

impl FactoryService {
    pub fn seed(code_id_pair: CodeId, fee_to: ActorId, fee_to_setter: ActorId, admin: ActorId) {
        let mut seed_factory = StateFactory {
            code_id_pair,
                fee_to,
                fee_to_setter,
                admin,
                router: ActorId::zero(),
                pairs: HashMap::new(),
                bridged_assets: HashMap::new(),
        };
        seed_factory.bridged_assets.insert(
            ActorId::from_str("0xb78e078fa0947e4e3a21f0edf7104f7208119d547cc91dc28dbc0d80cc072c0c").unwrap(), 
            BridgedAsset {
            name: String::from_str("Wrapped USDC").unwrap(),
            symbol: String::from_str("WUSDC").unwrap(),
            decimals: 18,
        });
        seed_factory.bridged_assets.insert(
            ActorId::from_str("0x2156679a6147013e5217fa3b8210d0ce4986c54aaffcfa70c4a171c7a8b6afd9").unwrap(), 
            BridgedAsset {
            name: String::from_str("Wrapped USDT").unwrap(),
            symbol: String::from_str("WUSDT").unwrap(),
            decimals: 18,
        });
        seed_factory.bridged_assets.insert(
            ActorId::from_str("0xaa6bc2ad1b660f6e7aaf3cb3418e6f66fe8c78f55400051b1d8bef0483976a42").unwrap(), 
            BridgedAsset {
            name: String::from_str("Wrapped Ethereum").unwrap(),
            symbol: String::from_str("WETH").unwrap(),
            decimals: 18,
        });


        unsafe {
                FACTORY = Some(seed_factory);
        }
    }
}
#[service(events = FactoryEvent)]
impl FactoryService {
    pub fn new(vft_client: VftClient<GStdRemoting>) -> Self {
        Self { vft_client }
    }

    pub fn set_fee_to(&mut self, new_fee_to: ActorId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.fee_to_setter {
            return Err(FactoryError::Unauthorized);
        };
        factory_state.fee_to = new_fee_to;
        Ok(())
    }

    pub fn add_bridged_asset(&mut self, token_address: ActorId, name: String, symbol: String, decimals: u8) -> Result<BridgedAsset, FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        // check exist
        if factory_state.bridged_assets.contains_key(&token_address) {
            return Err(FactoryError::BridgedAssetExist);
        }
        factory_state.bridged_assets.insert(token_address, BridgedAsset { name: name.clone(), symbol: symbol.clone(), decimals });
        self.emit_event(FactoryEvent::BridgedAssetAdded { token_address, name: name.clone(), symbol: symbol.clone(), decimals }).ok();
        Ok(BridgedAsset { name, symbol, decimals })
    }

    pub fn remove_bridged_asset(&mut self, token_address: ActorId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        factory_state.bridged_assets.remove(&token_address);
        self.emit_event(FactoryEvent::BridgedAssetRemoved { token_address }).ok();
        Ok(())
    }

    pub fn add_pair(&mut self, token_a: ActorId, token_b: ActorId, pair_address: ActorId) -> Result<ActorId, FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };

        //check pair exists
        if factory_state.pairs.contains_key(&token_pair) {
            return Err(FactoryError::PairExist);
        }

        factory_state.pairs.insert(token_pair, pair_address);
        Ok(pair_address)
    }

    pub fn set_fee_to_setter(&mut self, new_fee_setter: ActorId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.fee_to_setter {
            return Err(FactoryError::Unauthorized);
        };
        factory_state.fee_to_setter = new_fee_setter;
        self.emit_event(FactoryEvent::FeeToSetterSet(new_fee_setter))
            .ok();
        Ok(())
    }

    pub async fn create_pair(
        &mut self,
        token_a: ActorId,
        token_b: ActorId,
    ) -> Result<ActorId, FactoryError> {
        let factory_state = StateFactory::get_mut();

        if token_a == token_b {
            return Err(FactoryError::VFTError);
        }

        if token_a.is_zero() || token_b.is_zero() {
            return Err(FactoryError::VFTError);
        }

        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };

        //check pair exists
        if factory_state.pairs.contains_key(&token_pair) {
            return Err(FactoryError::PairExist);
        }
        // check token a and token b is bridged asset , if not get from  contract

        let mut token_a_name = String::new();
        let mut token_a_symbol = String::new();
        let mut token_b_name = String::new();
        let mut token_b_symbol = String::new();

        if factory_state.bridged_assets.contains_key(&token_a) {
            token_a_name = factory_state.bridged_assets.get(&token_a).unwrap().name.clone();
            token_a_symbol = factory_state.bridged_assets.get(&token_a).unwrap().symbol.clone();
        } else {
            let token_a_name_res = self.vft_client.name().recv(token_a).await;
            let Ok(token_a_name_get) = token_a_name_res else {
                return Err(FactoryError::VFTError);
            }; 
            token_a_name = token_a_name_get;
            let token_a_symbol_res = self.vft_client.symbol().recv(token_a).await;
            let Ok(token_a_symbol_get) = token_a_symbol_res else {
                return Err(FactoryError::VFTError);
            };
            token_a_symbol = token_a_symbol_get;
        }

        if factory_state.bridged_assets.contains_key(&token_b) {
            token_b_name = factory_state.bridged_assets.get(&token_b).unwrap().name.clone();
            token_b_symbol = factory_state.bridged_assets.get(&token_b).unwrap().symbol.clone();
        } else {
            let token_b_name_res = self.vft_client.name().recv(token_b).await;
            let Ok(token_b_name_get) = token_b_name_res else {
                return Err(FactoryError::VFTError);
            };
            token_b_name = token_b_name_get;
            let token_b_symbol_res = self.vft_client.symbol().recv(token_b).await;
            let Ok(token_b_symbol_get) = token_b_symbol_res else {
                return Err(FactoryError::VFTError);
            };
            token_b_symbol = token_b_symbol_get;
        }

        // let token_a_name_res = self.vft_client.name().recv(token_a).await;
        // let Ok(token_a_name) = token_a_name_res else {
        //     return Err(FactoryError::VFTError);
        // };
        // let token_a_symbol_res = self.vft_client.symbol().recv(token_a).await;
        // let Ok(token_a_symbol) = token_a_symbol_res else {
        //     return Err(FactoryError::VFTError);
        // };
        
        // let token_b_name_res = self.vft_client.name().recv(token_b).await;
        // let Ok(token_b_name) = token_b_name_res else {
        //     return Err(FactoryError::VFTError);
        // };

        // let token_b_symbol_res = self.vft_client.symbol().recv(token_b).await;
        // let Ok(token_b_symbol) = token_b_symbol_res else {
        //     return Err(FactoryError::VFTError);
        // };

        let lp_name = format!("{}_{}_{}", token_a_name, token_b_name, "LP".to_string());
        let lp_symbol = format!("{}_{}_{}", token_a_symbol, token_b_symbol, "LP".to_string());
        let lp_decimals = 12;

        let payload = InitPair {
            name: lp_name.clone(),
            symbol: lp_symbol.clone(),
            decimals: lp_decimals,
            factory: exec::program_id(),
            token_a,
            token_b,
            admin: factory_state.admin,
            router: factory_state.router,
        };
        let payload_bytes = ["New".encode(), payload.encode()].concat();
        let create_program_future_res = ProgramGenerator::create_program_bytes_with_gas_for_reply(
            factory_state.code_id_pair,
            payload_bytes,
            10_000_000_000, // gas limit
            0,
            10_000_000_000, // gas for reply
        )
        .map_err(|_| FactoryError::VFTError);

        let Ok(create_program_future) = create_program_future_res else {
            return Err(FactoryError::VFTError);
        };

        let pair_address = create_program_future.program_id;

        //insert new pair_address
        factory_state
            .pairs
            .insert(token_pair.clone(), pair_address.clone());

        //pair length
        let pair_number = factory_state.pairs.len().try_into().unwrap();

        self.emit_event(FactoryEvent::PairCreated {
            token_pair,
            pair_address: pair_address.clone(),
            pair_number,
        })
        .ok();

        Ok(pair_address)
    }

    pub fn set_router(&mut self, router: ActorId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        factory_state.router = router;
        self.emit_event(FactoryEvent::RouterSet(router)).ok();
        Ok(())
    }

    pub fn set_admin(&mut self, new_admin: ActorId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        factory_state.admin = new_admin;
        self.emit_event(FactoryEvent::AdminSet(new_admin)).ok();
        Ok(())
    }

    pub fn update_code_id_pair(&mut self, new_code_id_pair: CodeId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        factory_state.code_id_pair = new_code_id_pair;
        self.emit_event(FactoryEvent::CodeIdPairUpdated(new_code_id_pair)).ok();
        Ok(())
    }
    pub  fn remove_pair(&mut self, token_a: ActorId, token_b: ActorId) -> Result<(), FactoryError> {
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.admin {
            return Err(FactoryError::Unauthorized);
        }
        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };
        factory_state.pairs.remove(&token_pair);
        self.emit_event(FactoryEvent::PairRemoved { token_pair }).ok();
        Ok(())
    }

    //view function

    pub fn get_fee_to(&self) -> ActorId {
        let state = StateFactory::get();
        state.fee_to
    }

    pub fn get_fee_to_setter(&self) -> ActorId {
        let state = StateFactory::get();
        state.fee_to_setter
    }

    pub fn get_pair(&self, token_a: ActorId, token_b: ActorId) -> ActorId {
        let state = StateFactory::get();

        //get value
        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };
        let pair_address = state.pairs.get(&token_pair).cloned().unwrap_or_default();
        pair_address
    }

    pub fn get_pair_length(&self) -> u64 {
        let state = StateFactory::get();
        state.pairs.len().try_into().unwrap()
    }

    pub fn get_all_pairs(&self) -> Vec<(ActorId, ActorId)> {
        let state = StateFactory::get();
        state.pairs.keys().cloned().collect()
    }
    pub fn get_all_pairs_address(&self) -> Vec<ActorId> {
        let state = StateFactory::get();
        state.pairs.values().cloned().collect()
    }
    pub fn get_code_id_pair(&self) -> CodeId {
        let state = StateFactory::get();
        state.code_id_pair
    }
    pub fn get_admin(&self) -> ActorId {
        let state = StateFactory::get();
        state.admin
    }
    pub fn get_router(&self) -> ActorId {
        let state = StateFactory::get();
        state.router
    }
}
