#![no_std]
pub mod services;
pub mod clients;
pub mod states;

use sails_rs::{
    prelude::*,
    gstd::calls::GStdRemoting
};
use services::factory_service::FactoryService;
use clients::extended_new_vft::Vft as VftClient;

#[derive(Default)]
pub struct FactoryVaraDexProgram;

#[program]
impl FactoryVaraDexProgram {
    
    pub fn new(code_id_pair: CodeId, fee_to: ActorId, fee_to_setter: ActorId) -> Self {
        FactoryService::seed(code_id_pair, fee_to, fee_to_setter);
        Self
    }
    #[route("FactoryService")]
    pub fn factory_service(&self) -> FactoryService {
        let vft_client = VftClient::new(GStdRemoting);
        FactoryService::new(vft_client)
    }
}
