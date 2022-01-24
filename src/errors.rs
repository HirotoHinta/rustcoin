use std::fmt;
use bitcoin::OutPoint;

use crate::bitcoin::Network;
use crate::{descriptor, wallet, wallet::address_validator};

#[derive(Debug)]
pub enum Error {
    invalidU32Bytes(Vec<u8>),
    generic(String),
    scriptDoesntHaveAddress,
    noUtxosSelected,
    outputBelowDustLimit(usize),
}