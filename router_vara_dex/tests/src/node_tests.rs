use gclient::{EventProcessor, GearApi, Result};
use sails_rs::{ActorId, Decode, Encode, U256};

use crate::utils_gclient::*;
// use crate::utils::utils;
use crate::utils::workspace_cargo_toml_path;
use std::{path::Path, fs};

#[tokio::test]
#[ignore]
async fn testing_gear_api() -> Result<()> {
    let gear_path = workspace_cargo_toml_path().join("target").join("tmp").join("gear");
    
    if !Path::new(&gear_path).exists() {
        println!("Creando directorio para gear");
        fs::create_dir_all(&gear_path);
    }

    println!("Iniciando testing de api in: {:?}", gear_path);
    // let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    // let api = GearApi::dev_from_path(gear_path).await?;
    let api = GearApi::dev().await?;
    println!("Se inicio la Gear Api"); 
    let john_api = get_new_client(&api, USERS_STR[0]).await;

    println!("Avr se acabo el pdo de la api");

    Ok(())
}

// #[tokio::test]
// #[ignore]
// async fn test_basic_function() -> Result<()> {
//     let api = GearApi::dev().await?;
//     let john_api = get_new_client(&api, USERS_STR[0]).await;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     // Init
//     let (message_id, program_id) = init(&api).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Mint
//     let actor = api.get_actor_id();
//     let value: U256 = 1_000.into();
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vft", action: "Mint", payload: (actor, value));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check Balance
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vft", action: "BalanceOf", return_type: U256, payload: (actor));
//     assert_eq!(balance_value, value);

//     // Burn
//     let burn_value: U256 = 100.into();
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vft", action: "Burn", payload: (actor, burn_value));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check Balance
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vft", action: "BalanceOf", return_type: U256, payload: (actor));
//     assert_eq!(balance_value, value - burn_value);

//     // Transfer
//     let transfer_value: U256 = 100.into();
//     let john_actor_id = john_api.get_actor_id();
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vft", action: "Transfer", payload: (john_actor_id, burn_value));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check Balance
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vft", action: "BalanceOf", return_type: U256, payload: (actor));

//     assert_eq!(balance_value, value - burn_value - transfer_value);
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vft", action: "BalanceOf", return_type: U256, payload: (john_actor_id));

//     assert_eq!(balance_value, transfer_value);

//     // Approve
//     let approve_value: U256 = 100.into();
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vft", action: "Approve", payload: (john_actor_id, approve_value));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // TransferFrom
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vft", action: "TransferFrom", payload: (actor, john_actor_id, approve_value));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check Balance
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vft", action: "BalanceOf", return_type: U256, payload: (john_actor_id));
//     assert_eq!(balance_value, transfer_value + approve_value);
//     Ok(())
// }