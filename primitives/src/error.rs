use crate::{ParseNameError, ParseAssetError};
use hex;

pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
    FromHexError(hex::FromHexError),
}
