use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prevBlockHash: Hash,
    pub nonce: u64,
    pub difficulty: u128,
}
