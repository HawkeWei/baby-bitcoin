pub mod v1;
fn v1_test() {
    println!("This is bitcoin v1 example...");

    let mut bc = v1::blockchain::BlockChain::new_blockchain();
    bc.add_block(String::from("Alice -> Bob : 1 BTC"));
    bc.add_block(String::from("Bob -> Cat : 0.5 BTC"));

    for block in bc.blocks {
        println!("----------------------------------");
        println!("{:?}", block);
        println!("");
    }
}

pub mod v2;
use crate::v2::block::Transaction;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
fn v2_test() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    info!("This is bitcoin v2 example...");

    info!("Start!");
    let mut bc = v2::blockchain::BlockChain::new_blockchain().unwrap();

    // 创建一笔交易
    let from = "0xabcd".to_string();
    let to = "0xabce".to_string();
    let sign = format!("{from} -> {to}: 9 btc");
    let tx = Transaction::new(0, from, to, 9, 1, sign).unwrap();
    bc.add_block(vec![tx]).unwrap();

    let from = "0xabce".to_string();
    let to = "0xabcf".to_string();
    let sign = format!("{from} -> {to}: 6 btc");
    let tx = Transaction::new(1, from, to, 6, 1, sign).unwrap();
    bc.add_block(vec![tx]).unwrap();

    info!(?bc.blocks);
    info!("End!");
}

fn main() {
    // v1_test();

    v2_test();
}
