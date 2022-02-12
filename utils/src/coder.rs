use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn encode<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let encoded = bincode::serialize(value).unwrap();
    encoded
}

pub fn decode<'a, T>(bytes: &'a [u8]) -> T 
where
    T: Deserialize<'a>, 
{
    let decoded: T = bincode::deserialize(&bytes[..]).unwrap();
    decoded
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{encode, decode};

    #[test]
    fn coder_works() {
        let point = Point{x: 1, y: 1};
        let se = encode(&point);
        let de: Point = decode(&se);

        assert_eq!(de, point);
    }
}
