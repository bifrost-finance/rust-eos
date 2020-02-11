use crate::{
    ParseAssetError, ParseNameError,
    ParseSymbolError, ReadError, WriteError
};
#[cfg(feature = "std")]
use keys::error as KeyError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    BytesReadError(ReadError),
    BytesWriteError(WriteError),
    FromHexError(hex::FromHexError),
    #[cfg(feature = "std")]
    Keys(KeyError::Error),
    ParseAssetErr(ParseAssetError),
    ParseNameErr(ParseNameError),
    ParseSymbolError(ParseSymbolError),
    FromTrxKindsError,
    IncreMerkleError,
    InvalidLength,
    NoNewProducersList,
    #[cfg(feature = "std")]
    VerificationError(KeyError::Error),
}
