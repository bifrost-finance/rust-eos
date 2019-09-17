use crate::{ParseNameError, ParseAssetError};

pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
}
