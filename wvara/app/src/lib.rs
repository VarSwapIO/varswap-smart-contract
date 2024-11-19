#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use services::wvara_ft::WvaraService;
pub struct Program(());

#[program]
impl Program {
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        WvaraService::seed(name, symbol, decimals);
        Self(())
    }

    pub fn vft(&self) -> WvaraService {
        WvaraService::new()
    }
}
