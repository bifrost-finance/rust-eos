use crate::{ParseNameError, ParseAssetError};
use hex;
use keys::error as KeyError;

pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
    FromHexError(hex::FromHexError),
    Base58(KeyError::Error),
}
