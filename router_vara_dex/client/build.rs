use sails_client_gen::ClientGenerator;
use std::{env, path::PathBuf};

fn main() {
    // Path where the client will be generated 
    // 'OUT_DIR' points to a temporary directory used by the compiler 
    // to store files generated at compile time. 
    let out_dir_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Path where the file "factory_vara_dex.idl" will be created
    let idl_path = out_dir_path.join("router_vara_dex.idl");
    let client_path = out_dir_path.join("router_vara_dex_client.rs");

    // Generate the IDL file
    // sails_idl_gen::generate_idl_to_file::<contract::ContractProgram>(&idl_path)
    sails_idl_gen::generate_idl_to_file::<router_vara_dex::RouterVaraDexProgram>(&idl_path)
        .unwrap();

    // Generate the client code
    ClientGenerator::from_idl_path(&idl_path)
        .with_mocks("mocks")
        .generate_to(&client_path)
        .unwrap()
}