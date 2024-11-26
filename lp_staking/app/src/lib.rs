#![no_std]

use sails_rs::{prelude::*,
    gstd::calls::GStdRemoting
};

use clients::extended_new_vft::Vft as VftClient;
use clients::lp_vara_dex_client::LpService as LpServiceClient;

pub mod states;
pub mod services;
pub mod clients;

#[derive(Default)]
pub struct LpStakingProgram(());

#[sails_rs::program]
impl LpStakingProgram {
    // Program's constructor
    pub fn new(end_time:u64, staked_token:ActorId, reward_token:ActorId,x_per_second:U256, admin:ActorId) -> Self {
        services::lp_staking_services::LpStakingService::seed(end_time, staked_token, reward_token, x_per_second, admin);
        Self(())
    }

    // Exposed service
    #[sails_rs::route("LpStakingService")]
    pub fn lp_staking(&self) -> services::lp_staking_services::LpStakingService {
        let vft_client = VftClient::new(GStdRemoting);
        let lp_client = LpServiceClient::new(GStdRemoting);
        services::lp_staking_services::LpStakingService::new(vft_client,lp_client)
    }
}
