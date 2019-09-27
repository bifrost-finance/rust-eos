#![warn(
    clippy::all,
    clippy::complexity,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod chain;
pub mod history;
pub mod net;
pub mod producer;

mod client;
mod clients;
mod error;

pub use self::client::*;
pub use self::clients::*;
pub use self::error::*;
pub use crate::chain::*;
