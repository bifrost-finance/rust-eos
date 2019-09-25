use bitcoin_hashes;
use core::fmt;
use crate::base58;
use secp256k1;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /// Base58 encoding error
    Base58(base58::Error),
    /// secp-related error
    Secp256k1(secp256k1::Error),
    /// hash error
    Hash(bitcoin_hashes::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Base58(ref e) => fmt::Display::fmt(e, f),
            Error::Secp256k1(ref e) => fmt::Display::fmt(e, f),
            Error::Hash(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<base58::Error> for Error {
    fn from(e: base58::Error) -> Error {
        Error::Base58(e)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(e: secp256k1::Error) -> Error {
        Error::Secp256k1(e)
    }
}

impl From<bitcoin_hashes::error::Error> for Error {
    fn from(e: bitcoin_hashes::error::Error) -> Error {
        Error::Hash(e)
    }
}
