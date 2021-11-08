use clap::{load_yaml, App};
use keyring::AccountKeyring;
use sp_core::crypto::Pair;
use sp_runtime::MultiAddress;

use substrate_api_client::{Api, XtStatus};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url)
        .map(|api| api.set_signer(from.clone()))
        .unwrap();

    let fr = AccountKeyring::Alice.to_account_id();
    
    match api.get_account_data(&fr).unwrap() {
        Some(alice) => println!("[+] Alice's Free Balance is is {}\n", alice.free),
        None => println!("[+] Alice's Free Balance is is 0\n"),
    }

    
    let xt = api.balance_set_balance(
        MultiAddress::Id(fr.clone()),
        42,
        42
    );

    println!("[+] Composed extrinsic: {:?}\n", xt);

    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);
    
    let alice = api.get_account_data(&fr).unwrap().unwrap();
    println!("[+] Alice's Free Balance is now {}\n", alice.free);
    
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
