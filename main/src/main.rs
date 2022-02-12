use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("start mining...");
    thread::sleep(Duration::from_secs(10));
    bc.add_block("a -> b: sbtc".to_string());
    println!("produce a block!");

    thread::sleep(Duration::from_secs(10));
    bc.add_block("c -> d: sbtc".to_string());
    println!("produce a block!");

    for b in bc.blocks {
        println!("++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
