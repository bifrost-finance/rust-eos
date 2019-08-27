#![allow(dead_code)]

/// The size (in bytes) of a message
pub const MESSAGE_SIZE: usize = 32;

/// The size (in bytes) of a secret key
pub const SECRET_KEY_SIZE: usize = 32;

/// The size (in bytes) of a serialized public key.
pub const PUBLIC_KEY_SIZE: usize = 33;

/// The size (in bytes) of a serialized public key's checksum
pub const PUBLIC_KEY_CHECKSUM_SIZE: usize = 4;

/// The size (in bytes) of a serialized public key and checksum
pub const PUBLIC_KEY_WITH_CHECKSUM_SIZE: usize = PUBLIC_KEY_SIZE + PUBLIC_KEY_CHECKSUM_SIZE;

/// The size (in bytes) of an serialized uncompressed public key
pub const UNCOMPRESSED_PUBLIC_KEY_SIZE: usize = 65;

/// The size (in bytes) of a serialized keypair
pub const KEYPAIR_LENGTH: usize = SECRET_KEY_SIZE + PUBLIC_KEY_SIZE;

/// The maximum size of a signature
pub const MAX_SIGNATURE_SIZE: usize = 72;

/// The maximum size of a compact signature
pub const COMPACT_SIGNATURE_SIZE: usize = 64;
