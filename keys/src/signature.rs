#![allow(dead_code)]
use std::fmt;

/// An secp256k1 signature.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Signature(pub secp256k1::Signature);

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sig = self.0.serialize_der();
        for v in sig.iter() {
            write!(f, "{:02x}", v)?;
        }

        Ok(())
    }
}
