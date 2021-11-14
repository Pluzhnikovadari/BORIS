use clap::{load_yaml, App};
use keyring::AccountKeyring;
use sp_core::crypto::Pair;
use sp_runtime::MultiAddress;
use std::env;

use substrate_api_client::{Api, XtStatus};


fn main() {
    let mut acc = String::new();
    let line = std::io::stdin().read_line(&mut acc).unwrap();
    env_logger::init();
    let url = get_node_url_from_cli();

    let mut from = AccountKeyring::Alice.pair();
    let mut fr = AccountKeyring::Alice.to_account_id();


    if acc == String::from("Alice\n") {
        from = AccountKeyring::Alice.pair();
        fr = AccountKeyring::Alice.to_account_id();
    } else if acc == String::from("Bob\n") {
        from = AccountKeyring::Bob.pair();
        fr = AccountKeyring::Bob.to_account_id();
    } else if acc == String::from("Charlie\n") {
        from = AccountKeyring::Charlie.pair();
        fr = AccountKeyring::Charlie.to_account_id();
    } else if acc == String::from("Dave\n") {
        from = AccountKeyring::Dave.pair();
        fr = AccountKeyring::Dave.to_account_id();
    } else if acc == String::from("Eve\n") {
        from = AccountKeyring::Eve.pair();
        fr = AccountKeyring::Eve.to_account_id();
    } else if acc == String::from("Ferdie\n") {
        from = AccountKeyring::Ferdie.pair();
        fr = AccountKeyring::Ferdie.to_account_id();
    }


    let api = Api::new(url)
        .map(|api| api.set_signer(from.clone()))
        .unwrap();


    let balance = api.get_account_data(&fr).unwrap().unwrap();
    println!("[+] Free Balance is {}\n", balance.free);
    
    /*
    match api.get_account_data(&fr).unwrap() {
        Some(alice) => println!("[+] Free Balance is is {}\n", alice.free),
        None => println!("[+] Free Balance is is 0\n"),
    }
    */

    let diff: u128 = 42;

    let xt = api.balance_set_balance(
        MultiAddress::Id(fr.clone()),
        42 - diff,
        42
    );


    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);
    

    let alice = api.get_account_data(&fr).unwrap().unwrap();
    println!("[+] Free Balance is now {}\n", alice.free);
    
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9945");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
