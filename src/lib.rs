//! `SteamID` FX crate.
//! Work with `SteamID`s easily and safely.
//! The specification is mostly here:
//! <https://developer.valvesoftware.com/wiki/SteamID>
//! but this crate deviates a bit from it as empirically it was
//! noticed that the valve themselves don't follow this spec fully.
//! To be in-line with valve's behaviour, this crates deviated from the spec.
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

/// An iterator over bits.
pub mod bit_iterator;
/// The errors used in this crate.
pub mod error;
/// The steam ID implementation.
pub mod id;
/// The services the crate can work with regarding the steam id information.
pub mod services;
