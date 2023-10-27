use bitcoin::blockchain::BlockChain;
fn main() {
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
