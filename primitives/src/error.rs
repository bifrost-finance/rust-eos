use hex;

use keys::error as KeyError;

use crate::{ParseAssetError, ParseNameError, ReadError, WriteError};

#[derive(Clone, Debug)]
pub enum Error {
    BytesReadError(ReadError),
    BytesWriteError(WriteError),
    FromHexError(hex::FromHexError),
    Keys(KeyError::Error),
    ParseAssetErr(ParseAssetError),
    ParseNameErr(ParseNameError),
}

pub type Result<T> = std::result::Result<T, Error>;
