use gclient::{EventProcessor, GearApi, Result};
use sails_rs::{ActorId, Decode, Encode, U256};

use crate::{get_state, send_request, utils, utils_gclient::*};
// use crate::utils::utils;
use crate::utils::workspace_cargo_toml_path;
use std::str::FromStr;
use std::{path::Path, fs};

const ONE_TOKEN: u128 = 1_000_000_000_000;

#[tokio::test]
#[ignore]
async fn test_create_pair() -> Result<()> {
    // Set an api and a new client
    let api = GearApi::vara_testnet().await?;
    let john_api = get_new_client(&api, USERS_STR[0]).await;

    // Set a listener
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);


    // Init the vara dex factory contract
    let (dex_message_id, dex_program_id) = upload_factory_vara_dex(&api).await;

    assert!(listener.message_processed(dex_message_id).await?.succeed());

    // Set a router before creating pair

    let message_id = send_request!(
        api: &api,
        program_id: dex_program_id,
        service_name: "FactoryService",
        action: "SetRouter",
        payload: (ActorId::from(ROUTER_ID)),
        value: 0
    );

    assert!(listener.message_processed(message_id).await?.succeed());


    // Get actor id
    let actor = john_api.get_actor_id();

    println!("actor: {:?}", actor);

    let vft_program_id_a = ActorId::from_str("0x3efbbfe9657b8d733a031a35902f6806279c4452d79de55ab9d6c164af2126e9").unwrap(); // USDT on testnet
    let vft_program_id_b = ActorId::from_str("0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585").unwrap(); // USDC on testnet
    
    // Creating pair
    let message_id = send_request!(
        api: &api, 
        program_id: dex_program_id, 
        service_name: "FactoryService", 
        action: "CreatePair", 
        payload: (vft_program_id_a, vft_program_id_b),
        value: ONE_TOKEN
    );

    assert!(listener.message_processed(message_id).await?.succeed());


    // Check pair length
    let pair_length = get_state!(
        api: &api,
        listener: listener,
        program_id: dex_program_id,
        service_name: "FactoryService",
        action: "GetPairLength",
        return_type: u64,
        payload: (actor)
    );
    assert_eq!(pair_length, 1);


    Ok(())
}