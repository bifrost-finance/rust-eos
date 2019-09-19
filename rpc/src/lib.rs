#![warn(
    clippy::all,
    clippy::complexity,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

pub mod chain;
pub mod history;
pub mod net;
pub mod producer;

mod client;
mod clients;
mod error;
mod eosio {
    pub use eosio_core::*;
    pub use eosio_core_macros::*;
}

pub use self::client::*;
pub use self::clients::*;
pub use self::error::*;
pub use crate::chain::*;
