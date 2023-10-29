pub mod v1;
use v1::blockchain::BlockChain;
fn v1_test() {
    println!("This is bitcoin example...");

    let mut bc = BlockChain::new_blockchain();
    bc.add_block(String::from("Alice -> Bob : 1 BTC"));
    bc.add_block(String::from("Bob -> Cat : 0.5 BTC"));

    for block in bc.blocks {
        println!("----------------------------------");
        println!("{:?}", block);
        println!("");
    }
}

pub mod v2;

fn main() {
    v1_test();
}
