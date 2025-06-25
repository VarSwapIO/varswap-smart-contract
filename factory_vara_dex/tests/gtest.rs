#![no_std]
use gstd::{ActorId, CodeId};
use sails_rs::{
    calls::{Activation, Call, Query},
    gtest::{calls::*, System},
    prelude::*,
};

use factory_vara_dex::clients::{
    factory_vara_dex_client::{
        traits::{FactoryService as FS, FactoryVaraDexFactory as _},
        *,
    },
    wvara_client::{traits::WvaraVftFactory as __, *},
};

const ADMIN_ID: u64 = 10;
const USER_ID: u64 = 11;
const FEE_TO_SETTER_ID: u64 = 12;
const FEE_TO_ID: u64 = 13;
const ROUTER_ID: u64 = 14;

fn init_system() -> System {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 1_000_000_000_000_000);
    system.mint_to(USER_ID, 1_000_000_000_000_000);
    system.mint_to(FEE_TO_SETTER_ID, 1_000_000_000_000_000);
    system.mint_to(FEE_TO_ID, 1_000_000_000_000_000);
    system
}

async fn init_factory(system: System) -> (GTestRemoting, ActorId) {
    let remoting = GTestRemoting::new(system, ADMIN_ID.into());

    let pair_code_id = remoting.system().submit_code_file(
        "../../lp_vara_dex/target/wasm32-gear/release/application_builder.opt.wasm",
    );
    let factory_code_id = remoting
        .system()
        .submit_code_file("../target/wasm32-gear/release/application_builder.opt.wasm");

    let factory_factory = FactoryVaraDexFactory::new(remoting.clone());

    let factory_id = factory_factory
        .new(
            pair_code_id,
            ADMIN_ID.into(),
            ADMIN_ID.into(),
            ADMIN_ID.into(),
        )
        .send_recv(factory_code_id, "init factory")
        .await
        .unwrap();

    (remoting, factory_id)
}

#[cfg(test)]
#[tokio::test]
async fn test_initialization_and_getters() {
    let system = init_system();
    let (remoting, factory_id) = init_factory(system).await;
    let factory = FactoryService::new(remoting);
    // test getters
    let admin = factory.get_admin().recv(factory_id).await.unwrap();
    assert_eq!(admin, ADMIN_ID.into());

    let fee_to = factory.get_fee_to().recv(factory_id).await.unwrap();
    assert_eq!(fee_to, ADMIN_ID.into());

    let fee_to_setter = factory.get_fee_to_setter().recv(factory_id).await.unwrap();
    assert_eq!(fee_to_setter, ADMIN_ID.into());

    let pair_length = factory.get_pair_length().recv(factory_id).await.unwrap();
    assert_eq!(pair_length, 0);

    let all_pairs = factory.get_all_pairs().recv(factory_id).await.unwrap();
    assert!(all_pairs.is_empty());
}
#[cfg(test)]
#[tokio::test]
async fn test_admin_functions() {
    let system = init_system();
    let (remoting, factory_id) = init_factory(system).await;
    let mut factory = FactoryService::new(remoting);

    // set_fee_to
    let new_fee_to: ActorId = 20.into();
    factory
        .set_fee_to(new_fee_to)
        .send(factory_id)
        .await
        .unwrap();
    let fee_to = factory.get_fee_to().recv(factory_id).await.unwrap();
    assert_eq!(fee_to, new_fee_to);

    // set_fee_to_setter
    let new_fee_to_setter: ActorId = 21.into();
    factory
        .set_fee_to_setter(new_fee_to_setter)
        .send(factory_id)
        .await
        .unwrap();
    let fee_to_setter = factory.get_fee_to_setter().recv(factory_id).await.unwrap();
    assert_eq!(fee_to_setter, new_fee_to_setter);

    // set_router
    let new_router: ActorId = 22.into();
    factory
        .set_router(new_router)
        .send(factory_id)
        .await
        .unwrap(); // from admin
    let router = factory.get_router().recv(factory_id).await.unwrap();
    assert_eq!(router, new_router);

    // update_code_id_pair
    let new_code_id_pair = CodeId::from([2; 32]);
    factory
        .update_code_id_pair(new_code_id_pair)
        .send(factory_id)
        .await
        .unwrap();
    let code_id_pair = factory.get_code_id_pair().recv(factory_id).await.unwrap();
    assert_eq!(code_id_pair, new_code_id_pair);

    // set_admin
    let new_admin: ActorId = 23.into();
    factory.set_admin(new_admin).send(factory_id).await.unwrap(); // from old admin
    let admin = factory.get_admin().recv(factory_id).await.unwrap();
    assert_eq!(admin, new_admin);
}

async fn deploy_vft(remoting: &GTestRemoting, name: &str, symbol: &str) -> ActorId {
    let vft_code_id = remoting
        .system()
        .submit_code_file("tests/extended_vft.opt.wasm");

    let vft_factory = WvaraVftFactory::new(remoting.clone());
    vft_factory
        .new(name.to_string(), symbol.to_string(), 18)
        .send_recv(vft_code_id, "init vft")
        .await
        .unwrap()
}
#[cfg(test)]
#[tokio::test]
async fn test_create_pair() {
    let system = init_system();
    let (remoting, factory_id) = init_factory(system).await;
    let mut factory = FactoryService::new(remoting.clone());

    // Set router before creating pair
    factory
        .set_router(ROUTER_ID.into())
        .send(factory_id)
        .await
        .unwrap();

    let token_a_id = deploy_vft(&remoting, "Token A", "TKA").await;
    let token_b_id = deploy_vft(&remoting, "Token B", "TKB").await;

    let pair_address = factory
        .create_pair(token_a_id, token_b_id)
        .send_recv(factory_id)
        .await
        .unwrap()
        .unwrap();

    assert!(!pair_address.is_zero());

    let pair_length = factory.get_pair_length().recv(factory_id).await.unwrap();
    assert_eq!(pair_length, 1);

    let retrieved_pair_address = factory
        .get_pair(token_a_id, token_b_id)
        .recv(factory_id)
        .await
        .unwrap();
    assert_eq!(retrieved_pair_address, pair_address);

    let all_pairs = factory.get_all_pairs().recv(factory_id).await.unwrap();
    let (p_a, p_b) = all_pairs[0];
    let token_pair = if token_b_id > token_a_id {
        (token_b_id, token_a_id)
    } else {
        (token_a_id, token_b_id)
    };
    assert_eq!((p_a, p_b), token_pair);

    let all_pairs_address = factory
        .get_all_pairs_address()
        .recv(factory_id)
        .await
        .unwrap();
    assert_eq!(all_pairs_address[0], pair_address);

    // Test creating same pair again should fail
    let res = factory
        .create_pair(token_a_id, token_b_id)
        .send_recv(factory_id)
        .await;
    assert!(res.is_err());
}
#[cfg(test)]
#[tokio::test]
async fn test_bridged_assets() {
    let system = init_system();
    let (remoting, factory_id) = init_factory(system).await;
    let mut factory = FactoryService::new(remoting);

    let token_address: ActorId = 100.into();
    let name = "Bridged Token".to_string();
    let symbol = "BTK".to_string();
    let decimals = 6;

    let bridged_asset = factory
        .add_bridged_asset(token_address, name.clone(), symbol.clone(), decimals)
        .send_recv(factory_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(bridged_asset.name, name);
    assert_eq!(bridged_asset.symbol, symbol);
    assert_eq!(bridged_asset.decimals, decimals);

    // Remove the bridged asset
    factory
        .remove_bridged_asset(token_address)
        .send(factory_id)
        .await
        .unwrap();

    // Adding it again should be successful
    let bridged_asset_again = factory
        .add_bridged_asset(token_address, name.clone(), symbol.clone(), decimals)
        .send_recv(factory_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(bridged_asset_again.name, name);
}
#[cfg(test)]
#[tokio::test]
async fn test_manual_pair_management() {
    let system = init_system();
    let (remoting, factory_id) = init_factory(system).await;
    let mut factory = FactoryService::new(remoting);

    let token_a: ActorId = 200.into();
    let token_b: ActorId = 201.into();
    let pair_address: ActorId = 300.into();

    let added_pair_address = factory
        .add_pair(token_a, token_b, pair_address)
        .send_recv(factory_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(added_pair_address, pair_address);

    let mut pair_length = factory.get_pair_length().recv(factory_id).await.unwrap();
    assert_eq!(pair_length, 1);

    factory
        .remove_pair(token_a, token_b)
        .send_recv(factory_id)
        .await
        .unwrap()
        .unwrap();

    pair_length = factory.get_pair_length().recv(factory_id).await.unwrap();
    assert_eq!(pair_length, 0);
}
