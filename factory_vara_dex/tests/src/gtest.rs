#[cfg(test)]
mod tests {
    use std::{path::PathBuf, env};
    use sails_rs::{
        calls::{Activation, Call, Query}, futures::future::Remote, gtest::{calls::*, System}, prelude::*
    };

    use client::{
        traits:: {
            FactoryVaraDexFactory,
            FactoryService
        },
        FactoryVaraDexFactory as Factory,
        FactoryService as FactoryDexClient,
        FactoryError
    };
    use factory_vara_dex::clients::extended_new_vft::{
        traits::{
            WvaraVftFactory,
            Vft
        },
        WvaraVftFactory as FactoryWVaraVft,
        Vft as WVaraVftClient,
    };

    use crate::utils;
    
    const ADMIN_ID: u64 = 10;
    const USER_ID: u64 = 11;
    const FEE_TO_SETTER_ID: u64 = 12;
    const FEE_TO_ID: u64 = 13;
    const ROUTER_ID: u64 = 14;

    async fn init_factory() -> (GTestRemoting, ActorId) {
        let (program_space, code_id) = utils::program_space_and_code_id(
            ADMIN_ID, 
            vec![
                ADMIN_ID,
                USER_ID,
                FEE_TO_ID,
                FEE_TO_SETTER_ID
            ],
            1000
        );

        let factory = Factory::new(program_space.clone());
        let contract_id = factory
            .new(
                CodeId::zero(),
                ADMIN_ID.into(),
                ADMIN_ID.into(),
                ADMIN_ID.into()
            )
            .send_recv(code_id, "123")
            .await
            .unwrap();


        (program_space, contract_id)
    }


    #[tokio::test]
    async fn test_initialization_and_getters() {
        let (program_space, contract_id) = init_factory().await;
        let mut client = FactoryDexClient::new(program_space);

        // test getters

        let temp = client.get_admin()
            .recv(contract_id)
            .await;

        let admin: ActorId = match temp {
            Ok(res) => res,
            Err(error) => {
                std::panic!("{}", format!("Error: {}", error.to_string()));
            }
        };

        assert_eq!(admin, ADMIN_ID.into());

        let temp = client.get_fee_to()
            .recv(contract_id)
            .await;

        let fee_to: ActorId = match temp {
            Ok(res) => res,
            Err(error) => std::panic!("Error: {}", error.to_string())
        };

        assert_eq!(fee_to, ADMIN_ID.into());

        let temp = client.get_fee_to_setter()
            .recv(contract_id)
            .await;

        let fee_to_setter: ActorId = match temp {
            Ok(res) => res,
            Err(error) => std::panic!("Error: {}", error.to_string())
        };

        assert_eq!(fee_to_setter, ADMIN_ID.into());

        let temp = client.get_pair_length()
            .recv(contract_id)
            .await;

        let pair_length = match temp {
            Ok(res) => res,
            Err(error) => std::panic!("Error: {}", error.to_string())
        };

        assert_eq!(pair_length, 0);

        let temp = client.get_all_pairs()
            .recv(contract_id)
            .await;

        let all_pairs: Vec<(ActorId, ActorId)> = match temp {
            Ok(res) => res,
            Err(error) => std::panic!("Error: {}", error.to_string())
        };

        assert!(all_pairs.is_empty());
    }

    #[tokio::test]
    async fn test_admin_functions() {
        let (program_space, factory_id) = init_factory().await;
        let mut client = FactoryDexClient::new(program_space);

        // set_fee_to
        let new_fee_to: ActorId = 20.into();
        client
            .set_fee_to(new_fee_to)
            .send(factory_id)
            .await
            .unwrap();
        let fee_to = client.get_fee_to().recv(factory_id).await.unwrap();
        assert_eq!(fee_to, new_fee_to);

        // set_fee_to_setter
        let new_fee_to_setter: ActorId = 21.into();
        client
            .set_fee_to_setter(new_fee_to_setter)
            .send(factory_id)
            .await
            .unwrap();
        let fee_to_setter = client.get_fee_to_setter().recv(factory_id).await.unwrap();
        assert_eq!(fee_to_setter, new_fee_to_setter);

        // set_router
        let new_router: ActorId = 22.into();
        client
            .set_router(new_router)
            .send(factory_id)
            .await
            .unwrap(); // from admin
        let router = client.get_router().recv(factory_id).await.unwrap();
        assert_eq!(router, new_router);

        // update_code_id_pair
        let new_code_id_pair = CodeId::from([2; 32]);
        client
            .update_code_id_pair(new_code_id_pair)
            .send(factory_id)
            .await
            .unwrap();
        let code_id_pair = client.get_code_id_pair().recv(factory_id).await.unwrap();
        assert_eq!(code_id_pair, new_code_id_pair);

        // set_admin
        let new_admin: ActorId = 23.into();
        client.set_admin(new_admin).send(factory_id).await.unwrap(); // from old admin
        let admin = client.get_admin().recv(factory_id).await.unwrap();
        assert_eq!(admin, new_admin);
    }

    #[tokio::test]
    async fn test_bridged_assets() {
        let (program_space, factory_id) = init_factory().await;
        let mut client = FactoryDexClient::new(program_space);

        let token_address: ActorId = 100.into();
        let name = "Bridged Token".to_string();
        let symbol = "BTK".to_string();
        let decimals = 6;

        let bridged_asset = client
            .add_bridged_asset(token_address, name.clone(), symbol.clone(), decimals)
            .send_recv(factory_id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(bridged_asset.name, name);
        assert_eq!(bridged_asset.symbol, symbol);
        assert_eq!(bridged_asset.decimals, decimals);

        // Remove the bridged asset
        client
            .remove_bridged_asset(token_address)
            .send(factory_id)
            .await
            .unwrap();

        // Adding it again should be successful
        let bridged_asset_again = client
            .add_bridged_asset(token_address, name.clone(), symbol.clone(), decimals)
            .send_recv(factory_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(bridged_asset_again.name, name);
    }

    #[tokio::test]
    async fn test_manual_pair_management() {
        let (program_space, factory_id) = init_factory().await;
        let mut client = FactoryDexClient::new(program_space);

        let token_a: ActorId = 200.into();
        let token_b: ActorId = 201.into();
        let pair_address: ActorId = 300.into();

        let added_pair_address = client
            .add_pair(token_a, token_b, pair_address)
            .send_recv(factory_id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(added_pair_address, pair_address);

        let mut pair_length = client.get_pair_length().recv(factory_id).await.unwrap();
        assert_eq!(pair_length, 1);

        client
            .remove_pair(token_a, token_b)
            .send_recv(factory_id)
            .await
            .unwrap()
            .unwrap();

        pair_length = client.get_pair_length().recv(factory_id).await.unwrap();
        assert_eq!(pair_length, 0);
    }




    // IMPORTANT <-----------------------------

    // Tests which contracts generate other contracts, have communication with other contracts, etc., need to be in node tests (gclient - integration tests)
    // gtest is used to verify only the contract code itself (gtest - unit tests)

    // async fn deploy_vft(remoting: &GTestRemoting, name: &str, symbol: &str) -> ActorId {


    //     let vft_factory_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
    //         .join("src")
    //         .join("extended_vft.opt.wasm");

    //     let vft_code_id = remoting
    //         .system()
    //         .submit_code_file(vft_factory_path);

    //     let factory = FactoryWVaraVft::new(remoting.clone()); //WvaraVftFactory::new(remoting);
    //     let contract_id: ActorId = factory
    //         .new(name.to_string(), symbol.to_string(), 18)
    //         .send_recv(vft_code_id, format!("1234{}", name))
    //         .await
    //         .unwrap();

    //     contract_id
    // }

    // #[tokio::test]
    // async fn test_create_pair() {
    //     // let system = init_system();
    //     // let (remoting, factory_id) = init_factory(system).await;
    //     // let mut factory = FactoryService::new(remoting.clone());

    //     let (program_space, factory_id) = init_factory().await;
    //     let mut client = FactoryDexClient::new(program_space.clone());

    //     // Set router before creating pair
    //     client
    //         .set_router(ROUTER_ID.into())
    //         .send(factory_id)
    //         .await
    //         .unwrap();

    //     let token_a_id = deploy_vft(&program_space.clone(), "Token A", "TKA").await;
    //     let token_b_id = deploy_vft(&program_space.clone(), "Token B", "TKB").await;

        
    //     let temp = client
    //         .create_pair(token_a_id, token_b_id)
    //         .send_recv(factory_id)
    //         .await;

    //     let pair_address: Result<ActorId, FactoryError> = match temp {
    //         Ok(res) => res,
    //         Err(error) => std::panic!("Error: {}", error.to_string())
    //     };

    //     assert!(pair_address.is_ok());

    //     println!("{:?}", pair_address.unwrap());


    //     assert!(!pair_address.is_zero());

    //     let pair_length = client.get_pair_length().recv(factory_id).await.unwrap();
    //     assert_eq!(pair_length, 1);

    //     let retrieved_pair_address = client
    //         .get_pair(token_a_id, token_b_id)
    //         .recv(factory_id)
    //         .await
    //         .unwrap();
    //     assert_eq!(retrieved_pair_address, pair_address);

    //     let all_pairs = client.get_all_pairs().recv(factory_id).await.unwrap();
    //     let (p_a, p_b) = all_pairs[0];
    //     let token_pair = if token_b_id > token_a_id {
    //         (token_b_id, token_a_id)
    //     } else {
    //         (token_a_id, token_b_id)
    //     };
    //     assert_eq!((p_a, p_b), token_pair);

    //     let all_pairs_address = client
    //         .get_all_pairs_address()
    //         .recv(factory_id)
    //         .await
    //         .unwrap();
    //     assert_eq!(all_pairs_address[0], pair_address);

    //     // Test creating same pair again should fail
    //     let res = client
    //         .create_pair(token_a_id, token_b_id)
    //         .send_recv(factory_id)
    //         .await;
    //     assert!(res.is_err());
    // }
}