#![no_std]

// Incorporate the generated code based on the idl file
include!(concat!(env!("OUT_DIR"), "/router_vara_dex_client.rs"));
