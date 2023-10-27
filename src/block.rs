use crate::utils;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub pre_hash: String, // 前一个Block的hash
    pub tx_hash: String,  // 本次Block中包含的交易的哈希
    pub time: i64,        // 时间戳
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader, // 区块头
    pub header_hash: String, // 区块头的hash
    pub data: String,        // 本次区块中包含的交易
}

impl Block {
    pub fn new(data: String, pre_hash: String) -> Self {
        let time = Utc::now().timestamp();
        // 计算交易的hash之前，要先将交易进行序列化
        let tx_hash = utils::get_hash(&(utils::serialize(&data)));
        let header = BlockHeader {
            pre_hash: pre_hash,
            tx_hash: tx_hash,
            time: time,
        };
        Block {
            header: header.clone(),
            // 计算区块头的hash之前，要先将区块头进行序列化
            header_hash: utils::get_hash(&(utils::serialize(&header))),
            data: data,
        }
    }
}
