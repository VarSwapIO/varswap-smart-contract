
use gstd::msg;
use sails_rs::{gstd::service, prelude::*};
mod funcs;
use crate::services;
use vft_service::{Service as VftService, Storage};


#[derive(Encode, Decode, TypeInfo)]
pub enum Event {
    Deposit { dst: ActorId, wad: U256 },
    Withdraw { src: ActorId, wad: U256 },
}
#[derive(Clone)]
pub struct WvaraService {
    vft: VftService,
}

impl WvaraService {
    pub fn seed(name: String, symbol: String, decimals: u8) -> Self {
        WvaraService {
            vft: <VftService>::seed(name, symbol, decimals),
        }
    }
}

#[service(extends = VftService, events = Event)]
impl WvaraService {
    pub fn new() -> Self {
        Self {
            vft: VftService::new(),
        }
    }

    pub fn deposit(&mut self) -> bool {
        let from = msg::source();
        let value = msg::value();
        let mutated = services::utils::panicking(|| {
            funcs::deposit(Storage::balances(), Storage::total_supply(), from, U256::from(value))
        });
        if mutated {
            let _ = self.notify_on(Event::Deposit { dst: from, wad: U256::from(value)});
        }
        mutated
    }

    pub fn withdraw(&mut self, value: U256) -> bool {
        let to = msg::source();
        let mutated = services::utils::panicking(|| {
            funcs::withdraw(Storage::balances(), Storage::total_supply(), to, value.clone())
        });
        if mutated {
            //transfer the value to the caller
            let _ = msg::send(to, b"withdraw", U256::low_u128(&value));
            let _ = self.notify_on(Event::Withdraw { src: to, wad: value });
        }
        mutated
    }
    
}

impl AsRef<VftService> for WvaraService {
    fn as_ref(&self) -> &VftService {
        &self.vft
    }
}
