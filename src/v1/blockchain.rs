use crate::v1::block::Block;
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(data, pre_block.header_hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> Block {
        Block::new("THIS IS GENESIS BLOCK".to_string(), String::from(""))
    }

    pub fn new_blockchain() -> Self {
        Self {
            blocks: vec![Self::new_genesis_block()],
        }
    }
}
