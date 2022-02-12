use chrono::prelude::*;
use utils::coder;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String, // transactions data merkle root hash
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String, // transactions data
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::encode(&(self.header));
        self.hash = coder::get_hash(&header);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transactions = coder::encode(&data);
        let tx_hash = coder::get_hash(&transactions[..]);

        let mut block = Block {
            header: BlockHeader {
                time: Utc::now().timestamp(),
                tx_hash,
                pre_hash,
            },
            hash: "".to_string(),
            data,
        };

        block.set_hash();
        block
    }
}