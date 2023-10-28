use bincode;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

/// 序列化
pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

/// 反序列化
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

/// 自定义hash
pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod tests {
    use crate::v1::utils::{deserialize, serialize};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[test]
    fn ser_test() {
        let p = Point { x: 7, y: 8 };
        let se = serialize(&p);
        let de: Point = deserialize(&se);

        assert_eq!(de, p);
    }
}
