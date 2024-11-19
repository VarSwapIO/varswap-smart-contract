use gstd::exec;
use gstd::prog::ProgramGenerator;
use sails_rs::calls::{Call, Query};
use sails_rs::gstd::calls::GStdRemoting;
use sails_rs::{
    prelude::*,
    gstd::msg,
    collections::HashMap
};

use crate::states::factory_state::{FactoryError, FactoryEvent, InitPair, StateFactory, FACTORY};
use crate::clients::extended_new_vft::traits::Vft;
use crate::clients::extended_new_vft::Vft as VftClient;

pub struct FactoryService{
    pub vft_client:VftClient<GStdRemoting>
}

impl FactoryService {        
    pub fn seed(code_id_pair:CodeId, fee_to:ActorId, fee_to_setter:ActorId){
        unsafe {
            FACTORY = Some(StateFactory {
                code_id_pair,
                fee_to,
                fee_to_setter,
                pairs: HashMap::new(),
            });
        }
    }   

 }
#[service(events = FactoryEvent)]
 impl FactoryService {

    pub fn new(vft_client:VftClient<GStdRemoting>) -> Self {
        Self {
            vft_client
        }
    }

    pub fn set_fee_to(&mut self, new_fee_to:ActorId) -> Result<(),FactoryError>{
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.fee_to_setter {
            return Err(FactoryError::Unauthorized);
        };
        factory_state.fee_to = new_fee_to;
        Ok(())
    }

    pub fn set_fee_to_setter(&mut self, new_fee_setter:ActorId)-> Result<(), FactoryError>{
        let caller = msg::source();
        let factory_state = StateFactory::get_mut();
        if caller != factory_state.fee_to_setter {
            return Err(FactoryError::Unauthorized);
        };
        factory_state.fee_to_setter = new_fee_setter;
        self.notify_on(FactoryEvent::FeeToSetterSet(new_fee_setter)).unwrap();
        Ok(())
    }

    pub async fn create_pair(
        &mut self,
        token_a: ActorId,
        token_b: ActorId) -> Result<ActorId,FactoryError> {
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
        if factory_state.pairs.contains_key(&token_pair){
            return Err(FactoryError::PairExist);
        }

        let token_a_name_res = self.vft_client.name().recv(token_a).await;
        let Ok(token_a_name) = token_a_name_res else {
            return Err(FactoryError::VFTError);
        };
        let token_b_name_res = self.vft_client.name().recv(token_b).await;
        let Ok(token_b_name) = token_b_name_res else {
            return Err(FactoryError::VFTError);
        };

        let token_a_symbol_res = self.vft_client.symbol().recv(token_a).await;
        let Ok(token_a_symbol) = token_a_symbol_res else {
            return Err(FactoryError::VFTError);
        };

        let token_b_symbol_res = self.vft_client.symbol().recv(token_b).await;
        let Ok(token_b_symbol) = token_b_symbol_res else {
            return Err(FactoryError::VFTError);
        };

        let lp_name = format!("{}_{}_{}", token_a_name, token_b_name,"LP".to_string());
        let lp_symbol = format!("{}_{}_{}", token_a_symbol, token_b_symbol,"LP".to_string());
        let lp_decimals = 12;

        let payload = InitPair {
            name:lp_name.clone(),
            symbol:lp_symbol.clone(),
            decimals:lp_decimals,
            factory:exec::program_id(),
            token_a,
            token_b
           };
        let payload_bytes = ["New".encode(), payload.encode()].concat();
        let create_program_future =
        ProgramGenerator::create_program_bytes(
                    factory_state.code_id_pair,
                    payload_bytes,
                    0,
                )
                .map_err(|_| FactoryError::VFTError);
    
        let Ok((_, address)) = create_program_future else {
                return Err(FactoryError::VFTError);
        };

        //insert new pair_address
        factory_state.pairs.insert(token_pair.clone(), address.clone());

        //pair length
        let pair_number = factory_state.pairs.len().try_into().unwrap();

        self.notify_on(FactoryEvent::PairCreated { token_pair, pair_address: address.clone(), pair_number }).unwrap();
        
        Ok(address)
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


 }
