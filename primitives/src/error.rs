use hex;

use keys::error as KeyError;

use crate::{ParseAssetError, ParseNameError, ReadError, WriteError};

#[derive(Clone, Debug)]
pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
    FromHexError(hex::FromHexError),
    Keys(KeyError::Error),
    BytesReadError(ReadError),
    BytesWriteError(WriteError),
}
