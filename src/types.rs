use std::convert::AsRef;
use std::ops::Sub;

use bitcoin::blockdata::transaction::{OutPoint, Transaction, TxOut};
use bitcoin::{hash_types::Txid, util::psbt};
use serde::{Deserialize, Serialize};

#[derive(Serialze, Deserialize, Debug)]
pub enum KeychainKind {
    External = 0,
    Internal = 1,
}

impl KeychainKind {
    fn as_byte(&self) -> u8 {
        match self {
            KeychainKind::External => b'e',
            KeychainKind::Internal => b'i',
        }
    }
}

impl AsRef<[u8]> for KeychainKind {
    fn as_ref(&self) -> &[u8] {
        match self {
            KeychainKind::External => b"e",
            KeychainKind::Internal => b"i",
        }
    }
}