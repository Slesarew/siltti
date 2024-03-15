#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod error;
pub mod process_input;
pub mod sign_with_companion;
pub mod storage;

#[cfg(test)]
mod tests;

use smoldot_light::{
    platform::DefaultPlatform, AddChainConfig, ChainId, Client,
};

use std::{
    fs::{File, read_dir},
    io::Read,
    iter,
    num::NonZeroU32,
};

pub fn start_smol() -> String {
    let mut client = Client::new(DefaultPlatform::new(
            env!("CARGO_PKG_NAME").into(),
            env!("CARGO_PKG_VERSION").into(),
        ));

    let mut chain_handles = Vec::new();
    let specs = read_dir("../chain-specs").unwrap();
    for specfile in specs {
        if let Ok(specfile) = specfile {
       
            println!("connecting {:#?} ...", specfile.file_name());
        let mut spec = String::new();
        match File::open(specfile.path()) {
            Ok(mut file) => file.read_to_string(&mut spec).unwrap(),
            Err(e) => panic!("{}", e),
        };
            
        let chain_config = AddChainConfig {
            user_data: (),
            specification: &spec,
            database_content: "",
            potential_relay_chains: iter::empty(),
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(u32::max_value()).unwrap(),
                max_subscriptions: u32::max_value(),
            },
        };
            let chain_entry = client.add_chain(chain_config).unwrap();

        chain_handles.push(chain_entry)
        }
    }
    format!("{:?}", chain_handles.iter().map(|a| a.chain_id).collect::<Vec<ChainId>>())
}

