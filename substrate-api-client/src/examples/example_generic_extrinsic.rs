/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use clap::{load_yaml, App};
use keyring::AccountKeyring;
use sp_core::crypto::Pair;

use substrate_api_client::{
    compose_extrinsic, UncheckedExtrinsicV4, Api, XtStatus,
};
use std::fs;


fn main() {
    let num = fs::read_to_string("./data.txt")
        .expect("Something went wrong reading the file");
    let number: u128 = match num.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error");
            return;
        },
    };
    env_logger::init();
    let url = get_node_url_from_cli();

    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url).map(|api| api.set_signer(from)).unwrap();

    let to = AccountKeyring::Bob.to_account_id();

    #[allow(clippy::redundant_clone)]
    let xt: UncheckedExtrinsicV4<_> = compose_extrinsic!(
        api.clone(),
        "Balances",
        "transfer",
        GenericAddress::Id(to),
        Compact(number as u128)
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);
    
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9946");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
