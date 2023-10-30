use crate::v2::mining;
use crate::v2::utils;
use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use tracing::{event, Level};

/// 区块头中新增挖矿难度和nonce
#[derive(Clone, Debug, Serialize)]
pub struct BlockHeader {
    pub pre_hash: String,
    pub tx_hash: String,
    pub time: i64,
    pub difficult: u64,
    pub nonce: u64,
}

/// 新增交易tx数据结构
#[derive(Serialize, Debug)]
pub struct Transaction {
    pub nonce: u64,   // 放重放
    pub from: String, // 发送者
    pub to: String,   // 接收者
    pub amount: u64,  // 金额
    pub fee: u64,     // 手续费
    pub sign: String, // 签名
    pub hash: String, // 交易的hash
}

impl Transaction {
    pub fn new(
        nonce: u64,
        from: String,
        to: String,
        amount: u64,
        fee: u64,
        sign: String,
    ) -> Result<Self> {
        event!(Level::INFO, "New transaction...");
        let mut tx = Self {
            nonce,
            from,
            to,
            amount,
            fee,
            sign,
            hash: "".to_string(),
        };
        // 先将tx序列化，再求其hash，最后赋值给tx.hash
        tx.hash = utils::get_hash(&utils::serialize(&tx).unwrap())?;

        Ok(tx)
    }
}

/// 在区块中，将简易版的data改成交易类型
pub struct Block {
    pub header: BlockHeader,
    pub header_hash: String,
    pub txs: Vec<Transaction>,
}

impl Block {
    pub fn new(txs: Vec<Transaction>, pre_hash: String, difficult: u64) -> Result<Self> {
        event!(Level::INFO, "Start creating new block...");
        let txs_hash = utils::get_hash(&(utils::serialize(&txs).unwrap()))?;
        let header = BlockHeader {
            pre_hash: pre_hash,
            tx_hash: txs_hash,
            time: Utc::now().timestamp(),
            difficult: difficult,
            nonce: 0,
        };
        let block = Block {
            header: header.clone(),
            header_hash: utils::get_hash(&(utils::serialize(&header).unwrap()))?,
            txs: txs,
        };

        /// 新增挖矿出块
        mining::mining(block, difficult)
    }
}
