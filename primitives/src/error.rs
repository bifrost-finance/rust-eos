use crate::{ParseNameError, ParseAssetError};
use keys;
use hex;

pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
    SignErr(keys::error::Error),
    FromHexError(hex::FromHexError),
}
