#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod keypair;
pub mod public;
pub mod secret;
pub mod signature;
pub mod error;

mod constant;
mod hash;
mod base58;
mod network;

use error::Result;
