#[cfg(test)]
mod tests {
    // This is a new file for router tests.

    use std::{env, path::PathBuf};
    // use gstd::{ActorId, CodeId};
    use sails_rs::{
        calls::{Activation, Call, Query},
        futures::future::Remote,
        gtest::{calls::*, System},
        prelude::*,
    };

    use router_vara_dex::clients::{
        extended_new_vft::{
            traits::{Vft as _, WvaraVftFactory as _},
            *,
        },
        factory_vara_dex_client::{
            traits::{FactoryService as FS, FactoryVaraDexFactory as FactoryFactory},
            *,
        },
        lp_vara_dex_client::{
            traits::{LpService as _, LpVaraDexFactory as _},
            *,
        },
    };

    use client::{
        traits::{RouterService as _, RouterVaraDexFactory as _},
        RouterService as RouterClient, RouterVaraDexFactory as RouterFactory, *,
    };

    use crate::utils;

    const ADMIN_ID: u64 = 10;
    const USER_ID: u64 = 11;
    const FUND_ID: u64 = 15;
    const SWAPPER_ID: u64 = 16;

    async fn init_router() -> (GTestRemoting, ActorId, ActorId, ActorId) {
        let (program_space, router_code_id) = utils::program_space_and_code_id(
            ADMIN_ID, 
            vec![
                ADMIN_ID,
                USER_ID,
                FUND_ID,
                SWAPPER_ID
            ],
            1000 * utils::ONE_TOKEN
        );

        
        let wvara_code_id = program_space.system().submit_code_file(
            "./wvara_vft_wasm.opt.wasm"
        );
        let factory_code_id = program_space.system().submit_code_file(
            "./factory_wasm.opt.wasm"
        );

        // Deploy wvara
        let wvara_factory = WvaraVftFactory::new(program_space.clone());
        let wvara_id = wvara_factory
            .new("Wrapped VARA".to_string(), "WVARA".to_string(), 18)
            .send_recv(wvara_code_id, "init wvara")
            .await
            .unwrap();

        // Deploy factory
        let factory_factory = FactoryVaraDexFactory::new(program_space.clone());
        let factory_id = factory_factory
            .new(
                CodeId::zero(), // hoặc code_id cho LP pair nếu cần
                ADMIN_ID.into(),
                ADMIN_ID.into(),
                ADMIN_ID.into(),
            )
            .send_recv(factory_code_id, "init factory")
            .await
            .unwrap();

        // Deploy router
        let router_factory = RouterFactory::new(program_space.clone());
        let router_id = router_factory
            .new(
                factory_id,
                wvara_id,
                ADMIN_ID.into(),
                FUND_ID.into(),
                300, // 0.3%
            )
            .send_recv(router_code_id, "init router")
            .await
            .unwrap();

        // set router cho factory nếu cần
        let mut factory = FactoryService::new(program_space.clone());
        factory
            .set_router(router_id)
            .send(factory_id)
            .await
            .unwrap();

        (program_space, router_id, factory_id, wvara_id)
    }


    #[tokio::test]
    async fn test_initialization_and_getters() {
        
        let (remoting, router_id, factory_id, wvara_id) = init_router().await;
        let router = RouterClient::new(remoting);

        // test getters
        let admin = router.get_admin().recv(router_id).await.unwrap();
        assert_eq!(admin, ADMIN_ID.into());

        let factory = router.get_factory().recv(router_id).await.unwrap();
        assert_eq!(factory, factory_id);

        let wvara = router.get_wvara().recv(router_id).await.unwrap();
        assert_eq!(wvara, wvara_id);

        let fund_addr = router.get_fund_addr().recv(router_id).await.unwrap();
        assert_eq!(fund_addr, FUND_ID.into());

        let swap_fee_bps = router.get_swap_fee_bps().recv(router_id).await.unwrap();
        assert_eq!(swap_fee_bps, 300);
    }

    #[tokio::test]
    async fn test_admin_functions() {
        let (remoting, router_id, _, _) = init_router().await;
        let mut router = RouterClient::new(remoting.clone());
        // update_new_factorty
        let new_factory: ActorId = 98.into();
        router
            .update_new_factorty(new_factory)
            .send(router_id)
            .await
            .unwrap();
        let factory = router.get_factory().recv(router_id).await.unwrap();
        assert_eq!(factory, new_factory);

        // update_new_wrapvara
        let new_wvara: ActorId = 97.into();
        router
            .update_new_wrapvara(new_wvara)
            .send(router_id)
            .await
            .unwrap();
        let wvara = router.get_wvara().recv(router_id).await.unwrap();
        assert_eq!(wvara, new_wvara);

        // update_fund_addr
        let new_fund: ActorId = 96.into();
        router
            .update_fund_addr(new_fund)
            .send(router_id)
            .await
            .unwrap();
        let fund = router.get_fund_addr().recv(router_id).await.unwrap();
        assert_eq!(fund, new_fund);

        // update_swap_fee_bps
        let new_fee: u128 = 500; // 0.5%
        router
            .update_swap_fee_bps(new_fee)
            .send(router_id)
            .await
            .unwrap();
        let fee = router.get_swap_fee_bps().recv(router_id).await.unwrap();
        assert_eq!(fee, new_fee);

        // update_new_admin
        let new_admin: ActorId = 99.into();
        router
            .update_new_admin(new_admin)
            .send(router_id)
            .await
            .unwrap();
        let admin = router.get_admin().recv(router_id).await.unwrap();
        assert_eq!(admin, new_admin);
    }

    // #[tokio::test]
    // async fn test_add_liquidity() {
    //     let system = init_system();
    //     let (remoting, router_id, factory_id, _) = init_router(system).await;
    //     let mut router = RouterClient::new(remoting.clone());
    //     let mut factory = FactoryService::new(remoting.clone());

    //     let vft_code_id = remoting
    //         .system()
    //         .submit_code_file("../../factory_vara_dex/app/tests/extended_vft.opt.wasm");

    //     let token_a_id = deploy_vft(remoting.clone(), "Token A", "TKA", vft_code_id, "TKA").await;
    //     let token_b_id = deploy_vft(remoting.clone(), "Token B", "TKB", vft_code_id, "TKB").await;

    //     // Create VFT clients to interact with the tokens
    //     let mut vft_a = Vft::new(remoting.clone());
    //     let mut vft_b = Vft::new(remoting.clone());

    //     // Mint some tokens to the user
    //     let mint_amount = 100_000_000_000_000_000u128.into();
    //     vft_a
    //         .mint(ADMIN_ID.into(), mint_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();
    //     vft_b
    //         .mint(ADMIN_ID.into(), mint_amount)
    //         .send(token_b_id)
    //         .await
    //         .unwrap();

    //     let balance_a = vft_a.balance_of(ADMIN_ID.into()).recv(token_a_id).await.unwrap();
    //     let balance_b = vft_b.balance_of(ADMIN_ID.into()).recv(token_b_id).await.unwrap();
    //     assert_eq!(balance_a, mint_amount);
    //     assert_eq!(balance_b, mint_amount);

    //     // Approve the router to spend the user's tokens
    //     let approve_a = vft_a
    //         .approve(router_id, mint_amount)
    //         .send_recv(token_a_id)
    //         .await
    //         .unwrap();
    //     // check approve
    //     assert!(approve_a);

    //     let approve_b = vft_b
    //         .approve(router_id, mint_amount)
    //         .send_recv(token_b_id)
    //         .await
    //         .unwrap();
    //     // check approve
    //     assert!(approve_b);

    //     println!("token_a_id: {:?}", token_a_id);
    //     println!("token_b_id: {:?}", token_b_id);
    //     println!("factory_id: {:?}", factory_id);
    //     println!("router_id: {:?}", router_id);

    //     // Use router to create pair
    //     let pair = factory
    //         .create_pair(token_a_id, token_b_id)
    //         .send_recv(factory_id)
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     // check pair
    //     assert_ne!(pair, ActorId::zero());

    //     // Add liquidity
    //     let amount_a_desired: U256 = 100_000_000_000_000_000u128.into();
    //     let amount_b_desired: U256 = 100_000_000_000_000_000u128.into();
    //     let deadline = remoting.system().block_timestamp() + 3600;

    //     let (amount_a, amount_b, liquidity) = router
    //         .add_liquidity(
    //             token_a_id,
    //             token_b_id,
    //             amount_a_desired,
    //             amount_b_desired,
    //             0.into(),
    //             0.into(),
    //             ADMIN_ID.into(),
    //             deadline,
    //         )
    //         .send_recv(router_id)
    //         .await
    //         .unwrap()
    //         .unwrap();

    //     assert_eq!(amount_a, amount_a_desired);
    //     assert_eq!(amount_b, amount_b_desired);
    //     assert!(liquidity > 0.into());

    //     let lp_token = LpService::new(remoting.clone());
    //     let user_lp_balance = lp_token
    //         .balance_of(ADMIN_ID.into())
    //         .recv(pair)
    //         .await
    //         .unwrap();
    //     assert_eq!(user_lp_balance, liquidity);
    // }

    // #[tokio::test]
    // async fn test_remove_liquidity() {
    //     let system = init_system();
    //     let (remoting, router_id, factory_id, _) = init_router(system).await;
    //     let factory = FactoryService::new(remoting.clone());

    //     let mut user_router = RouterClient::new(remoting.clone());

    //     let token_a_id = deploy_vft(remoting.clone(), "Token A", "TKA").await;
    //     let token_b_id = deploy_vft(remoting.clone(), "Token B", "TKB").await;

    //     // Create VFT clients to interact with the tokens
    //     let mut vft_a = Vft::new(remoting.clone());
    //     let mut vft_b = Vft::new(remoting.clone());

    //     // Mint some tokens to the user
    //     let mint_amount: U256 = 1_000_000_000_000_000_000u128.into();
    //     vft_a
    //         .mint(ADMIN_ID.into(), mint_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();
    //     vft_b
    //         .mint(ADMIN_ID.into(), mint_amount)
    //         .send(token_b_id)
    //         .await
    //         .unwrap();

    //     // Approve the router to spend the user's tokens
    //     vft_a
    //         .approve(router_id, mint_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();
    //     vft_b
    //         .approve(router_id, mint_amount)
    //         .send(token_b_id)
    //         .await
    //         .unwrap();

    //     // Use admin router to create pair
    //     let mut admin_router = RouterClient::new(remoting.clone());
    //     admin_router
    //         .create_pair(token_a_id, token_b_id)
    //         .send(router_id)
    //         .await
    //         .unwrap();

    //     // Add liquidity to get LP tokens
    //     let amount_to_add: U256 = 100_000_000_000_000_000u128.into();
    //     let deadline = remoting.system().block_timestamp() + 3600;
    //     let (_, _, liquidity_received) = user_router
    //         .add_liquidity(
    //             token_a_id,
    //             token_b_id,
    //             amount_to_add,
    //             amount_to_add,
    //             0.into(),
    //             0.into(),
    //             ADMIN_ID.into(),
    //             deadline,
    //         )
    //         .send_recv(router_id)
    //         .await
    //         .unwrap()
    //         .unwrap();

    //     // Approve the router to spend user's LP tokens
    //     let pair_address = factory
    //         .get_pair(token_a_id, token_b_id)
    //         .recv(factory_id)
    //         .await
    //         .unwrap();
    //     let mut lp_token = LpService::new(remoting.clone());
    //     lp_token
    //         .approve(router_id, liquidity_received)
    //         .send(pair_address)
    //         .await
    //         .unwrap();

    //     // Remove liquidity
    //     let (amount_a_out, amount_b_out) = user_router
    //         .remove_liquidity(
    //             token_a_id,
    //             token_b_id,
    //             liquidity_received,
    //             0.into(),
    //             0.into(),
    //             ADMIN_ID.into(),
    //             deadline,
    //         )
    //         .send_recv(router_id)
    //         .await
    //         .unwrap()
    //         .unwrap();

    //     assert!(amount_a_out > 0.into());
    //     assert!(amount_b_out > 0.into());

    //     // Check user's LP token balance is now 0
    //     let user_lp_balance = lp_token
    //         .balance_of(ADMIN_ID.into())
    //         .recv(pair_address)
    //         .await
    //         .unwrap();
    //     assert_eq!(user_lp_balance, 0.into());
    // }

    // #[tokio::test]
    // async fn test_swap_exact_tokens_for_tokens() {
    //     let system = init_system();
    //     let (remoting, router_id, _factory_id, _) = init_router(system).await;

    //     // Liquidity provider setup
    //     let mut lp_router = RouterClient::new(remoting.clone());
    //     let token_a_id = deploy_vft(remoting.clone(), "Token A", "TKA").await;
    //     let token_b_id = deploy_vft(remoting.clone(), "Token B", "TKB").await;

    //     // Use admin to mint tokens for LP
    //     let mut admin_vft_a = Vft::new(remoting.clone());
    //     let mut admin_vft_b = Vft::new(remoting.clone());
    //     let mint_amount: U256 = 1_000_000_000_000_000_000u128.into();
    //     admin_vft_a
    //         .mint(ADMIN_ID.into(), mint_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();
    //     admin_vft_b
    //         .mint(ADMIN_ID.into(), mint_amount)
    //         .send(token_b_id)
    //         .await
    //         .unwrap();

    //     // LP approves router
    //     let mut vft_a_lp = Vft::new(remoting.clone());
    //     let mut vft_b_lp = Vft::new(remoting.clone());
    //     vft_a_lp
    //         .approve(router_id, mint_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();
    //     vft_b_lp
    //         .approve(router_id, mint_amount)
    //         .send(token_b_id)
    //         .await
    //         .unwrap();

    //     // Admin creates pair
    //     let mut admin_router = RouterClient::new(remoting.clone());
    //     admin_router
    //         .create_pair(token_a_id, token_b_id)
    //         .send(router_id)
    //         .await
    //         .unwrap();

    //     // LP adds liquidity
    //     let amount_to_add: U256 = 100_000_000_000_000_000u128.into();
    //     let deadline = remoting.system().block_timestamp() + 3600;
    //     lp_router
    //         .add_liquidity(
    //             token_a_id,
    //             token_b_id,
    //             amount_to_add,
    //             amount_to_add,
    //             0.into(),
    //             0.into(),
    //             ADMIN_ID.into(),
    //             deadline,
    //         )
    //         .send(router_id)
    //         .await
    //         .unwrap();

    //     // Swapper setup

    //     let mut swapper_router = RouterClient::new(remoting.clone());
    //     let mut vft_a_swapper = Vft::new(remoting.clone());
    //     let mut vft_b_swapper = Vft::new(remoting.clone());
    //     let swap_amount: U256 = 1_000_000_000_000_000u128.into();

    //     // Admin mints tokens for the swapper
    //     admin_vft_a
    //         .mint(ADMIN_ID.into(), swap_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();

    //     // Swapper approves router
    //     vft_a_swapper
    //         .approve(router_id, swap_amount)
    //         .send(token_a_id)
    //         .await
    //         .unwrap();

    //     // Perform swap
    //     let path = vec![token_a_id, token_b_id];
    //     let amounts = swapper_router
    //         .swap_exact_tokens_for_tokens(
    //             swap_amount,
    //             0.into(), // amount_out_min
    //             path,
    //             ADMIN_ID.into(),
    //             deadline,
    //         )
    //         .send_recv(router_id)
    //         .await
    //         .unwrap()
    //         .unwrap();

    //     // Check balances
    //     let swapper_balance_a = vft_a_swapper
    //         .balance_of(ADMIN_ID.into())
    //         .recv(token_a_id)
    //         .await
    //         .unwrap();
    //     let swapper_balance_b = vft_b_swapper
    //         .balance_of(ADMIN_ID.into())
    //         .recv(token_b_id)
    //         .await
    //         .unwrap();

    //     assert_eq!(swapper_balance_a, 0.into());
    //     assert_eq!(swapper_balance_b, amounts[1]);
    //     assert!(swapper_balance_b > 0.into());
    // }
}
