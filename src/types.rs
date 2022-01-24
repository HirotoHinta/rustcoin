use std::convert::AsRef;
use std::ops::Sub;

use bitcoin::blockdata::transaction::{OutPoint, Transaction, TxOut};
use bitcoin::{hash_types::Txid, util::psbt};
use serde::{Deserialize, Serialize};
use sled::LogKind::Free;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub struct FreeRate(f32);

impl std::default::Default for FreeRate {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Sub for FreeRate {
    type Output = Self;

    fn sub(self, other: FreeRate) -> Self::Output {
        FreeRate(self.0 - other.0)
    }
}

pub trait vBytes {
    fn vbytes(self) -> usize;
}