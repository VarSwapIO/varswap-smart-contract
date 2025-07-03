use gclient::{EventProcessor, GearApi, Result};
use sails_rs::{ActorId, Decode, Encode, U256};

use crate::{get_state, send_request, utils_gclient::*};
// use crate::utils::utils;
use crate::utils::workspace_cargo_toml_path;
use std::str::FromStr;
use std::{path::Path, fs};

const ONE_TOKEN: u128 = 1_000_000_000_000;


//0x4726c05451c1ee406f363039ea42b8951b9eb3f3e884a1200052b56c546258b9 // factory
//0xe881ae415d30e7651ad557c7a036b88efa88f5603796c89a3ea612122e73ac40 // wvara
//0x3efbbfe9657b8d733a031a35902f6806279c4452d79de55ab9d6c164af2126e9 // USDT
//0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585 // USDC



#[tokio::test]
#[ignore]
async fn testing_add_liquidity_vara() -> Result<()> {
    let api = GearApi::vara_testnet().await?; //0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
    let john_api = get_new_client_without_value(&api, USERS_STR[0]).await;


    let USDT_ADDRESS: ActorId = ActorId::from_str("0x3efbbfe9657b8d733a031a35902f6806279c4452d79de55ab9d6c164af2126e9").unwrap();
    let USDC_ADDRESS: ActorId = ActorId::from_str("0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585").unwrap();
    let FACTORY_ADDRESS: ActorId = ActorId::from_str("0x4726c05451c1ee406f363039ea42b8951b9eb3f3e884a1200052b56c546258b9").unwrap();
    let WVARA_ADDRESS: ActorId = ActorId::from_str("0xe881ae415d30e7651ad557c7a036b88efa88f5603796c89a3ea612122e73ac40").unwrap();

    // Set a listener
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let actor_id = john_api.get_actor_id();

    println!("actor_id: {:?}", actor_id); //0x80d6b692da7526380f6d7ccb5d838a4d255a32baccd38036d08b46c57613013f

     let (dex_message_id, dex_program_id) = upload_router_vara_dex(&john_api).await;
     assert!(listener.message_processed(dex_message_id).await?.succeed());

     // check if pair exists
     let mut pair_address = get_state!(
        api: &api,
        listener: listener,
        program_id: FACTORY_ADDRESS,
        service_name: "FactoryService",
        action: "GetPair",
        return_type: ActorId,
        payload: (WVARA_ADDRESS, USDC_ADDRESS)
     );

     println!("before_pair_address: {:?}", pair_address);

     if pair_address == ActorId::zero() {
        // create pair
        let create_pair_message_id = send_request!(
            api: &api,
            program_id: dex_program_id,
            service_name: "RouterService",
            action: "CreatePair",
            payload: (WVARA_ADDRESS, USDC_ADDRESS),
            value: ONE_TOKEN
        );
        assert!(listener.message_processed(create_pair_message_id).await?.succeed());
        pair_address = get_state!(
            api: &api,
            listener: listener,
            program_id: FACTORY_ADDRESS,
            service_name: "FactoryService",
            action: "GetPair",
            return_type: ActorId,
            payload: (WVARA_ADDRESS, USDC_ADDRESS)
        );
     }
     println!("after_pair_address: {:?}", pair_address);

     // get lp balance
     let lp_balance_before = get_state!(
        api: &api,
        listener: listener,
        program_id: pair_address,
        service_name: "LpService",
        action: "BalanceOf",
        return_type: U256,
        payload: (actor_id)
     );

     println!("lp_balance_before: {:?}", lp_balance_before);

     let amount_token_desired = U256::from(1000000000000000u64);
     let amount_token_min = U256::from(0);
     let amount_vara_min = U256::from(0);
     let deadline = 1000000000000000000 as u64;

     // approve usdc
     let approve_message_id = send_request!(
        api: &john_api,
        program_id: USDC_ADDRESS,
        service_name: "Vft",
        action: "Approve",
        payload: (dex_program_id, amount_token_desired ),
        value:0
     );

     assert!(listener.message_processed(approve_message_id).await?.succeed());

     // get usdc balance before add
     let usdc_balance_before = get_state!(
        api: &api,
        listener: listener,
        program_id: USDC_ADDRESS,
        service_name: "Vft",
        action: "BalanceOf",
        return_type: U256,
        payload: (actor_id)
     );

     println!("usdc_balance_before: {:?}", usdc_balance_before);


     // add liquidity vara
     let add_liquidity_message_id = send_request!(
        api: &john_api,
        program_id: dex_program_id,
        service_name: "RouterService",
        action: "AddLiquidityVara",
        payload: (USDC_ADDRESS, amount_token_desired, amount_token_min, amount_vara_min, actor_id, deadline),
        value: ONE_TOKEN
     );

     assert!(listener.message_processed(add_liquidity_message_id).await?.succeed());

     // get lp balance
     let lp_balance_after = get_state!(
        api: &api,
        listener: listener,
        program_id: pair_address,
        service_name: "LpService",
        action: "BalanceOf",
        return_type: U256,
        payload: (actor_id)
     );

     // get usdc balance after add
     let usdc_balance_after = get_state!(
        api: &api,
        listener: listener,
        program_id: USDC_ADDRESS,
        service_name: "Vft",
        action: "BalanceOf",
        return_type: U256,
        payload: (actor_id)
     );

     println!("lp_balance_after: {:?}", lp_balance_after);
     println!("usdc_balance_after: {:?}", usdc_balance_after);
     assert!(lp_balance_after > lp_balance_before);
     assert!(usdc_balance_after < usdc_balance_before);
    
    Ok(())
}

// #[tokio::test]
// #[ignore]
// async fn testing_add_liquidity() -> Result<()> {
//     let api = GearApi::vara_testnet().await?; //0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//     let john_api = get_new_client_without_value(&api, USERS_STR[0]).await;

//     let USDT_ADDRESS: ActorId = ActorId::from_str("0x3efbbfe9657b8d733a031a35902f6806279c4452d79de55ab9d6c164af2126e9").unwrap();
//     let USDC_ADDRESS: ActorId = ActorId::from_str("0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585").unwrap();
//     let FACTORY_ADDRESS: ActorId = ActorId::from_str("0x4726c05451c1ee406f363039ea42b8951b9eb3f3e884a1200052b56c546258b9").unwrap();
//     let WVARA_ADDRESS: ActorId = ActorId::from_str("0xe881ae415d30e7651ad557c7a036b88efa88f5603796c89a3ea612122e73ac40").unwrap();

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let actor_id = john_api.get_actor_id();

//     let (dex_message_id, dex_program_id) = upload_router_vara_dex(&john_api).await;
//     assert!(listener.message_processed(dex_message_id).await?.succeed());

//     // check if pair exists
//     let mut pair_address = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: FACTORY_ADDRESS,
//         service_name: "FactoryService",
//         action: "GetPair",
//         return_type: ActorId,
//         payload: (USDT_ADDRESS, USDC_ADDRESS)
//     );
//     if pair_address == ActorId::zero() {
//         let create_pair_message_id = send_request!(
//             api: &api,
//             program_id: dex_program_id,
//             service_name: "RouterService",
//             action: "CreatePair",
//             payload: (USDT_ADDRESS, USDC_ADDRESS),
//             value: ONE_TOKEN
//         );
//         assert!(listener.message_processed(create_pair_message_id).await?.succeed());
//         pair_address = get_state!(
//             api: &api,
//             listener: listener,
//             program_id: FACTORY_ADDRESS,
//             service_name: "FactoryService",
//             action: "GetPair",
//             return_type: ActorId,
//             payload: (USDT_ADDRESS, USDC_ADDRESS)
//         );
//     }

//     // get lp balance before add
//     let lp_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: pair_address,
//         service_name: "LpService",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdt balance before add
//     let usdt_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdc balance before add
//     let usdc_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     println!("usdt_balance_before: {:?}, usdc_balance_before: {:?}", usdt_balance_before, usdc_balance_before);

//     // approve usdt and usdc for router
//     let amount_token_desired = U256::from(1_000_000_000_000u64);
//     let amount_token_min = U256::from(0);
//     let amount_token2_desired = U256::from(1_000_000_000_000u64);
//     let amount_token2_min = U256::from(0);
//     let deadline = 1000000000000000000 as u64;

//     let approve_usdt_message_id = send_request!(
//         api: &john_api,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "Approve",
//         payload: (dex_program_id, amount_token_desired),
//         value: 0
//     );
//     assert!(listener.message_processed(approve_usdt_message_id).await?.succeed());

//     let approve_usdc_message_id = send_request!(
//         api: &john_api,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "Approve",
//         payload: (dex_program_id, amount_token2_desired),
//         value: 0
//     );
//     assert!(listener.message_processed(approve_usdc_message_id).await?.succeed());

//     // Add liquidity
//     let add_liquidity_message_id = send_request!(
//         api: &john_api,
//         program_id: dex_program_id,
//         service_name: "RouterService",
//         action: "AddLiquidity",
//         payload: (USDT_ADDRESS, USDC_ADDRESS, amount_token_desired, amount_token2_desired, amount_token_min, amount_token2_min, actor_id, deadline),
//         value: 0
//     );
//     assert!(listener.message_processed(add_liquidity_message_id).await?.succeed());

//     // get lp balance after add
//     let lp_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: pair_address,
//         service_name: "LpService",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdt balance after add
//     let usdt_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdc balance after add
//     let usdc_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     println!("usdt_balance_before: {:?}, usdt_balance_after: {:?}", usdt_balance_before, usdt_balance_after);
//     println!("usdc_balance_before: {:?}, usdc_balance_after: {:?}", usdc_balance_before, usdc_balance_after);

//     println!("lp_balance_before: {:?}, lp_balance_after: {:?}", lp_balance_before, lp_balance_after);
//     assert!(lp_balance_after > lp_balance_before);
//     assert!(usdt_balance_after < usdt_balance_before);
//     assert!(usdc_balance_after < usdc_balance_before);

//     Ok(())
// }



// #[tokio::test]
// #[ignore]
// async fn testing_swap_exact_tokens_for_tokens() -> Result<()> {
//     let api = GearApi::vara_testnet().await?; //0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//     let john_api = get_new_client_without_value(&api, USERS_STR[0]).await;

//     let USDT_ADDRESS: ActorId = ActorId::from_str("0x3efbbfe9657b8d733a031a35902f6806279c4452d79de55ab9d6c164af2126e9").unwrap();
//     let USDC_ADDRESS: ActorId = ActorId::from_str("0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585").unwrap();
//     let FACTORY_ADDRESS: ActorId = ActorId::from_str("0x4726c05451c1ee406f363039ea42b8951b9eb3f3e884a1200052b56c546258b9").unwrap();

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let actor_id = john_api.get_actor_id();
//     let (dex_message_id, dex_program_id) = upload_router_vara_dex(&john_api).await;
//     assert!(listener.message_processed(dex_message_id).await?.succeed());

//     // Approve USDT cho router
//     let amount_in = U256::from(1_000_000u64);
//     let approve_message_id = send_request!(
//         api: &john_api,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "Approve",
//         payload: (dex_program_id, amount_in),
//         value: 0
//     );
//     assert!(listener.message_processed(approve_message_id).await?.succeed());

//     // get usdt balance before swap
//     let usdt_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdc balance before swap
//     let usdc_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     println!("usdt_balance_before: {:?}, usdc_balance_before: {:?}", usdt_balance_before, usdc_balance_before);

//     // Swap
//     let amount_out_min = U256::from(0);
//     let deadline = 1000000000000000000 as u64;
//     let swap_message_id = send_request!(
//         api: &john_api,
//         program_id: dex_program_id,
//         service_name: "RouterService",
//         action: "SwapExactTokensForTokens",
//         payload: (amount_in, amount_out_min, vec![USDT_ADDRESS, USDC_ADDRESS], actor_id, deadline),
//         value: 0
//     );
//     assert!(listener.message_processed(swap_message_id).await?.succeed());

//     // get usdt balance after swap
//     let usdt_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdc balance after swap
//     let usdc_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     println!("usdt_balance_before: {:?}, usdt_balance_after: {:?}", usdt_balance_before, usdt_balance_after);
//     println!("usdc_balance_before: {:?}, usdc_balance_after: {:?}", usdc_balance_before, usdc_balance_after);

//     assert!(usdt_balance_after < usdt_balance_before);
//     assert!(usdc_balance_after > usdc_balance_before);

//     Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn testing_swap_exact_vara_for_tokens() -> Result<()> {
//     let api = GearApi::vara_testnet().await?; //0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//     let john_api = get_new_client_without_value(&api, USERS_STR[0]).await;

//     let USDC_ADDRESS: ActorId = ActorId::from_str("0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585").unwrap();
//     let WVARA_ADDRESS: ActorId = ActorId::from_str("0xe881ae415d30e7651ad557c7a036b88efa88f5603796c89a3ea612122e73ac40").unwrap();

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let actor_id = john_api.get_actor_id();
//     let (dex_message_id, dex_program_id) = upload_router_vara_dex(&john_api).await;
//     assert!(listener.message_processed(dex_message_id).await?.succeed());

//     // get usdt balance before swap
//     let usdc_balance_before = get_state!(
//         api: &john_api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );
//     println!("usdc_balance_before: {:?}", usdc_balance_before);

//     // Swap
//     let amount_vara = ONE_TOKEN / 100;
//     let amount_out_min = U256::from(0);
//     let deadline = 1000000000000000000 as u64;
//     let swap_message_id = send_request!(
//         api: &john_api,
//         program_id: dex_program_id,
//         service_name: "RouterService",
//         action: "SwapExactVaraForTokens",
//         payload: (amount_out_min, vec![WVARA_ADDRESS, USDC_ADDRESS], actor_id, deadline),
//         value: amount_vara
//     );
//     assert!(listener.message_processed(swap_message_id).await?.succeed());

//     // get usdt balance after swap
//     let usdc_balance_after = get_state!(
//         api: &john_api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );
//     println!("usdc_balance_after: {:?}", usdc_balance_after);

//     assert!(usdc_balance_after > usdc_balance_before);

//     Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn testing_remove_liquidity() -> Result<()> {
//     let api = GearApi::vara_testnet().await?; //0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//     let john_api = get_new_client_without_value(&api, USERS_STR[0]).await;

//     let USDT_ADDRESS: ActorId = ActorId::from_str("0x3efbbfe9657b8d733a031a35902f6806279c4452d79de55ab9d6c164af2126e9").unwrap();
//     let USDC_ADDRESS: ActorId = ActorId::from_str("0x770745e6e294f1b501dfbb1f02fd9b8330e12eaba37d6c6460c3468918875585").unwrap();
//     let FACTORY_ADDRESS: ActorId = ActorId::from_str("0x4726c05451c1ee406f363039ea42b8951b9eb3f3e884a1200052b56c546258b9").unwrap();

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let actor_id = john_api.get_actor_id();

//     let (dex_message_id, dex_program_id) = upload_router_vara_dex(&john_api).await;
//     assert!(listener.message_processed(dex_message_id).await?.succeed());

//     // get pair address
//     let pair_address = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: FACTORY_ADDRESS,
//         service_name: "FactoryService",
//         action: "GetPair",
//         return_type: ActorId,
//         payload: (USDT_ADDRESS, USDC_ADDRESS)
//     );

//     // get lp balance before remove
//     let lp_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: pair_address,
//         service_name: "LpService",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdt balance before remove
//     let usdt_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdc balance before remove
//     let usdc_balance_before = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     println!("usdt_balance_before: {:?}, usdc_balance_before: {:?}", usdt_balance_before, usdc_balance_before);

//     // approve lp for router
//     let approve_message_id = send_request!(
//         api: &john_api,
//         program_id: pair_address,
//         service_name: "LpService",
//         action: "Approve",
//         payload: (dex_program_id, lp_balance_before),
//         value: 0
//     );
//     assert!(listener.message_processed(approve_message_id).await?.succeed());

//     // Remove all liquidity
//     let amount_lp = lp_balance_before;
//     let amount_token_min = U256::from(0);
//     let amount_token2_min = U256::from(0);
//     let deadline = 1000000000000000000 as u64;

//     let remove_liquidity_message_id = send_request!(
//         api: &john_api,
//         program_id: dex_program_id,
//         service_name: "RouterService",
//         action: "RemoveLiquidity",
//         payload: (USDT_ADDRESS, USDC_ADDRESS, amount_lp, amount_token_min, amount_token2_min, actor_id, deadline),
//         value: 0
//     );
//     assert!(listener.message_processed(remove_liquidity_message_id).await?.succeed());

//     // get lp balance after remove
//     let lp_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: pair_address,
//         service_name: "LpService",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );
//     println!("lp_balance_before: {:?}, lp_balance_after: {:?}", lp_balance_before, lp_balance_after);

//     // get usdt balance after remove
//     let usdt_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDT_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     // get usdc balance after remove
//     let usdc_balance_after = get_state!(
//         api: &api,
//         listener: listener,
//         program_id: USDC_ADDRESS,
//         service_name: "Vft",
//         action: "BalanceOf",
//         return_type: U256,
//         payload: (actor_id)
//     );

//     println!("usdt_balance_before: {:?}, usdt_balance_after: {:?}", usdt_balance_before, usdt_balance_after);
//     println!("usdc_balance_before: {:?}, usdc_balance_after: {:?}", usdc_balance_before, usdc_balance_after);

//     assert!(usdt_balance_after > usdt_balance_before);
//     assert!(usdc_balance_after > usdc_balance_before);

//     assert!(lp_balance_after == U256::from(0));

//     Ok(())
// }