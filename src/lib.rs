//! SteamID FX crate.
//! Work with SteamIDs easily and safely.
//! <https://steamid.io/lookup/>
//! <https://developer.valvesoftware.com/wiki/SteamID>
#![deny(warnings)]
#![deny(missing_docs)]

mod bit_iterator;
/// The errors used in this crate.
pub mod error;
/// The steam ID implementation.
pub mod id;
/// The services the crate can work with regarding the steam id information.
pub mod services;
