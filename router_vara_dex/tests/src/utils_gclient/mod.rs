use std::str::FromStr;

use gclient::GearApi;
use gear_core::ids::{MessageId, ProgramId};
use sails_rs::{ActorId, Encode, CodeId};
use crate::utils::{self, ONE_TOKEN};

pub const USERS_STR: &[&str] = &["//John", "//Mike", "//Dan"];
pub const ADMIN_ID: u64 = 10;
pub const ROUTER_ID: u64 = 14;

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
}

pub async fn get_new_client(api: &GearApi, name: &str) -> GearApi {
    let alice_balance = api
        .total_balance(api.account_id())
        .await
        .expect("Error total balance");
    let amount = alice_balance / 5;
    api.transfer_keep_alive(
        api.get_specific_actor_id(name)
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        amount,
    )
    .await
    .expect("Error transfer");

    api.clone().with(name).expect("Unable to change signer.")
}

pub async fn get_new_client_without_value(api: &GearApi, name: &str) -> GearApi {
    api.clone().with(name).expect("Unable to change signer.")
}

pub async fn upload_router_vara_dex(api: &GearApi) -> (MessageId, ProgramId) {
    let filepath = utils::workspace_cargo_toml_path()
        .join("target")
        .join("wasm32-gear")
        .join("release")
        .join("application_builder.opt.wasm");

    let actor_id = api.get_actor_id();

    let constructor = (
        ActorId::from_str("0x4726c05451c1ee406f363039ea42b8951b9eb3f3e884a1200052b56c546258b9").unwrap(), // factory
        ActorId::from_str("0xe881ae415d30e7651ad557c7a036b88efa88f5603796c89a3ea612122e73ac40").unwrap(), // wvara
        actor_id,
        actor_id,
        300 as u128
    );

    let request = ["New".encode(), constructor.encode()].concat();

    // let path = "./../../../target/wasm32-gear/release/application_builder.opt.wasm";
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(filepath.clone()).unwrap(),
            request.clone(),
            0,
            true,
        )
        .await
        .expect("Error calculate upload gas");

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(filepath).unwrap(),
            gclient::now_micros().to_le_bytes(),
            request,
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error upload program bytes");

    (message_id, program_id)
}

pub async fn upload_vft(api: &GearApi, name: &str, symbol: &str) -> (MessageId, ProgramId) {
    let filepath = utils::crate_cargo_toml_path()
        .join("src")
        .join("extended_vft.opt.wasm");

    let constructor = (
        name,
        symbol,
        18
    );

    let request = ["New".encode(), constructor.encode()].concat();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(filepath.clone()).unwrap(),
            request.clone(),
            0,
            true,
        )
        .await
        .expect("Error calculate upload gas");

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(filepath).unwrap(),
            gclient::now_micros().to_le_bytes(),
            request,
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error upload program bytes");

    (message_id, program_id)
}

#[macro_export]
macro_rules! send_request {
    (api: $api:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, payload: ($($val:expr),*) ) => {
        $crate::send_request!(api: $api, program_id: $program_id, service_name: $name, action: $action, payload: ($($val),*), value: $value:expr)
    };

    (api: $api:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, payload: ($($val:expr),*), value: $value:expr) => {
        {
            let request = [
                $name.encode(),
                $action.to_string().encode(),
                ($($val),*).encode(),
            ].concat();

            let gas_info = $api
                .calculate_handle_gas(None, $program_id, request.clone(), $value, true)
                .await?;

            let (message_id, _) = $api
                .send_message_bytes($program_id, request.clone(), gas_info.min_limit, $value)
                .await?;

            message_id
        }
    };
}

#[macro_export]
macro_rules! get_state {

    (api: $api:expr, listener: $listener:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, return_type: $return_type:ty, payload: ($($val:expr),*)) => {
        {
            let request = [
                $name.encode(),
                $action.to_string().encode(),
                ($($val),*).encode(),
            ].concat();

            let gas_info = $api
                .calculate_handle_gas(None, $program_id, request.clone(), 0, true)
                .await
                .expect("Error send message bytes");

            let (message_id, _) = $api
                .send_message_bytes($program_id, request.clone(), gas_info.min_limit, 0)
                .await
                .expect("Error listen reply");

            let (_, raw_reply, _) = $listener
                .reply_bytes_on(message_id)
                .await
                .expect("Error listen reply");

            let decoded_reply = <(String, String, $return_type)>::decode(&mut raw_reply.unwrap().as_slice()).expect("Erroe decode reply");
            decoded_reply.2
        }
    };
}
