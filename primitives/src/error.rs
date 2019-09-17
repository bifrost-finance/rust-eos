use crate::{ParseNameError, ParseAssetError};
use keys;

pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
    SignErr(keys::error::Error),
}
