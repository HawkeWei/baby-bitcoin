use anyhow::{Ok, Result};
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

/// 序列化
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).map_err(anyhow::Error::msg)?;
    Ok(serialized)
}

/// 反序列化
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).map_err(anyhow::Error::msg)?;
    Ok(deserialized)
}

/// 自定义hash
/// 使用时，先调用上面的序列化函数，转成u8数组，再调用hash函数
pub fn get_hash(value: &[u8]) -> Result<String> {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    Ok(hasher.result_str())
}
