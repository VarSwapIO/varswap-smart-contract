#![no_std]
pub mod services;
pub mod clients;
pub mod states;

use sails_rs::{
    prelude::*,
    gstd::{
        calls::GStdRemoting,
    }
};

use clients::extended_new_vft::Vft as VftClient;
use clients::factory_vara_dex_client::FactoryService as FactoryServiceClient;
use services::lp_service::LPService;

#[derive(Default)]
pub struct LpVaraDexProgram;

#[program]
impl LpVaraDexProgram {
    pub fn new(factory:ActorId, token_a:ActorId, token_b:ActorId, name:String, symbol:String, decimals:u8) -> Self {
        LPService::seed(factory, token_a, token_b, name, symbol, decimals);
        Self
    }

    #[route("LpService")]
    pub fn lp_service(&self) -> LPService {
        let factory_client = FactoryServiceClient::new(GStdRemoting);
        let vft_client = VftClient::new(GStdRemoting);
        LPService::new(vft_client, factory_client)
    }
}
