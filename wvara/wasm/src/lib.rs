#![no_std]
// include!(concat!(env!("OUT_DIR"), "/wvara_vft_client.rs"));

#[cfg(target_arch = "wasm32")]
pub use wvara_vft_app::wasm::*;
