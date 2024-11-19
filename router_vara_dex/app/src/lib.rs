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
use services::router_service::RouterService;
use clients::lp_vara_dex_client::LpService as LpServiceClient;
#[derive(Default)]
pub struct RouterVaraDexProgram;

#[program]
impl RouterVaraDexProgram {
    pub fn new(factory:ActorId, wvara:ActorId, admin_addr:ActorId, fund_addr:ActorId, swap_fee_bps:u128) -> Self {
        RouterService::seed(factory, wvara, admin_addr, fund_addr, swap_fee_bps );
        Self
    }

    #[route("RouterService")]
    pub fn router_service(&self) -> RouterService {
        let factory_client = FactoryServiceClient::new(GStdRemoting);
        let vft_client = VftClient::new(GStdRemoting);
        let lp_client = LpServiceClient::new(GStdRemoting);
        RouterService::new(factory_client, vft_client, lp_client)
    }
}
