#![deny(unused_crate_dependencies)]
//#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::let_unit_value)]

//mod ffi_types;

//use crate::ffi_types::*;

use std::sync::{Mutex, TryLockError};

lazy_static! {
    static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

lazy_static! {
    static ref R: Mutex<Option<tokio::sync::oneshot::Receiver<String>>> = Mutex::new(None);
}

use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use serde_json::value::Value;

use lazy_static::lazy_static;
//use regex::Regex;

#[uniffi::export]
pub fn try_read() -> String {
    let guard = R.lock();
    match guard {
        Ok(mut mg) => match *mg {
            Some(ref mut a) => match a.try_recv() {
                Ok(b) => b,
                Err(e) => format!("error: {:?}", e),
            },
            None => return String::from("channel is empty"),
        },
        Err(_) => return String::from("guard error on read"),
    }
}

#[uniffi::export]
pub fn rpc_call(addr: &str) -> String {
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    let addr = addr.to_owned();
    RT.spawn(async move {
        let client = match WsClientBuilder::default()
            .build(&addr)
            .await {
                Ok(a) => a,
                Err(e) => {
                    tx.send(format!("client start error, {:?}", e));
                    panic!();
                },
            };
        let params = rpc_params![];
        let block_hash_data: Value = match client.request("chain_getBlockHash", params).await {
            Ok(a) => a,
            Err(e) => {
                tx.send(format!("gethash error {:?}", e));
                panic!()
            },
        };
        if let Value::String(a) = block_hash_data {
            tx.send(a);
        } else {
//           Ok("Unexpected block hash format.".to_string()
        }
    });
    let guard = R.lock();
    match guard {
        Ok(mut a) => *a = Some(rx),
        Err(_) => return String::from("guard error"),
    }
    String::from("Success")
}

uniffi::setup_scaffolding!();

//uniffi::include_scaffolding!(concat!(env!("OUT_DIR"), "/siltti"));
//include!(concat!(env!("OUT_DIR"), "/siltti.uniffi.rs"));

