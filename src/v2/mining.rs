use anyhow::Result;
use tracing::{event, Level};

use crate::v2::block::Block;

pub fn mining(block: Block, difficult: u64) -> Result<Block> {
    event!(Level::INFO, "Start Mining...");
    /// 实现挖矿逻辑，Todo
    Ok(block)
}
