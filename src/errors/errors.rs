use std::fmt;
use bitcoin::OutPoint;
use bitcoin::Network;

#[derive(Debug)]
pub enum Error {
    invalidU32Bytes(Vec<u8>),
    generic(String),
    scriptDoesntHaveAddress,
    noUtxosSelected,
    outputBelowDustLimit(usize),

    bnbTotalTriesExceeded,
    bnbNoExactMatch,
    unKnownUtxo,
    transactionNotFound,
    transactionConfrimed,

    insufficientFunds {
        needed: u64,
        available: u64,
    },

    freeRateTooLow {
        required: u64,
    },

    feeTooLow {
        required: u64,
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
