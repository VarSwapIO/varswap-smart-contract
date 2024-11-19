use wvara_vft_app::Program;
use std::{env, path::PathBuf};
use sails_client_gen::ClientGenerator;

fn main() {
    gwasm_builder::build();

    let idl_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("wvara_vft.idl");
    
        let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Generate IDL file for the app
    sails_idl_gen::generate_idl_to_file::<Program>(&idl_file_path).unwrap();

    // Generate client code from IDL file
    ClientGenerator::from_idl_path(&idl_file_path)
    .with_mocks("with_mocks")
    .generate_to(cargo_toml_path.join("extended_new_vft.rs"))
    .unwrap();
}
