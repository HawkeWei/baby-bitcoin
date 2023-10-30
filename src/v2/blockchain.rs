use crate::v2::block::{Block, Transaction};
use anyhow::{Ok, Result};
use tracing::{event, Level};

const INIT_DIFFICULT: u64 = 0;
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    /// 创世块
    fn new_genesis_block() -> Result<Block> {
        event!(Level::INFO, "Genesis block...");
        let first_tx = Transaction::new(
            0,
            "0x000".to_string(),
            "0x000".to_string(),
            0,
            0,
            "0".to_string(),
        )?;
        let first_block = Block::new(vec![first_tx], String::from("0"), INIT_DIFFICULT)?;
        Ok(first_block)
    }
    /// 创建区块链
    pub fn new() -> Result<BlockChain> {
        event!(Level::INFO, "New BlockChain...");
        Ok(Self {
            blocks: vec![Self::new_genesis_block()?],
        })
    }

    /// 添加新块
    pub fn add_block(&mut self, txs: Vec<Transaction>) -> Result<()> {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash = pre_block.header_hash.clone();
        let difficult = 0;
        let new_block = Block::new(txs, pre_hash, difficult)?;
        self.blocks.push(new_block);

        Ok(())
    }
}
