use crate::bit_iterator::BitIterator;
use regex::Regex;
#[cfg(feature = "serialization")]
use serde::de::{self, Visitor};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;

const DEFAULT_STEAM_ACCOUNT_TYPE: u8 = 1;
// The steam id community page just try to set it to `1` if you don't know the value.
const DEFAULT_STEAM_ACCOUNT_INSTANCE: u8 = 1;

/// Steam online state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialization", derive(serde::Deserialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "lowercase"))]
pub enum OnlineState {
    /// When a user is offline.
    Offline,
    /// When a user is online.
    Online,
    /// When a user is playing or in game.
    #[cfg_attr(feature = "serialization", serde(rename = "in-game"))]
    InGame,
    /// Any other status.
    #[cfg_attr(feature = "serialization", serde(other))]
    Other,
}
impl std::fmt::Display for OnlineState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match *self {
            OnlineState::Offline => "Offline",
            OnlineState::Online => "Online",
            OnlineState::InGame => "In game",
            OnlineState::Other => "Other",
        })
    }
}

/// Steam Id Universe.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SteamIdUniverse {
    /// An individual account or unspecified.
    IndividualOrUnspecified = 0,
    /// A public account.
    Public = 1,
    /// A beta account.
    Beta = 2,
    /// Internal (related to Valve?) account.
    Internal = 3,
    /// A developer account (a game developer's?).
    Developer = 4,
    /// Can't remember already. Fix this :-)
    Rc = 5,
}

impl std::fmt::Display for SteamIdUniverse {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            SteamIdUniverse::IndividualOrUnspecified => "Individual or unspecified",
            SteamIdUniverse::Public => "Public",
            SteamIdUniverse::Beta => "Beta",
            SteamIdUniverse::Internal => "Internal",
            SteamIdUniverse::Developer => "Developer",
            SteamIdUniverse::Rc => "RC",
        })
    }
}

impl std::convert::TryFrom<u64> for SteamIdUniverse {
    type Error = crate::error::Error;

    fn try_from(value: u64) -> crate::error::Result<Self> {
        Ok(match value {
            0 => SteamIdUniverse::IndividualOrUnspecified,
            1 => SteamIdUniverse::Public,
            2 => SteamIdUniverse::Beta,
            3 => SteamIdUniverse::Internal,
            4 => SteamIdUniverse::Developer,
            5 => SteamIdUniverse::Rc,
            _ => return Err("The number doesn't represent a correct steam id universe.".into()),
        })
    }
}

impl std::convert::TryFrom<u32> for SteamIdUniverse {
    type Error = crate::error::Error;

    fn try_from(value: u32) -> crate::error::Result<Self> {
        Self::try_from(value as u64)
    }
}

impl std::convert::TryFrom<u16> for SteamIdUniverse {
    type Error = crate::error::Error;

    fn try_from(value: u16) -> crate::error::Result<Self> {
        Self::try_from(value as u64)
    }
}

impl std::convert::TryFrom<u8> for SteamIdUniverse {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        Self::try_from(value as u64)
    }
}

/// Steam Id Account type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SteamIdAccountType {
    /// The account is of an individual.
    Individual = 1,
    /// The account is of a multiseat type.
    Multiseat = 2,
    /// The account is of a game server (registered in steam).
    GameServer = 3,
    /// The account is of a game server (unregistered in steam).
    AnonymousGameServer = 4,
    /// The account is pending approval from Valve.
    Pending = 5,
    /// The account is of a content server.
    ContentServer = 6,
    /// The account is of a clan.
    Clan = 7,
    /// The chat account.
    Chat = 8,
    /// The P2P seeder account.
    PeerToPeerSuperSeeder = 9,
    /// The anonymous user account.
    AnonymousUser = 10,
    /// An invalid account type.
    #[cfg_attr(feature = "serialization", serde(other))]
    Invalid = 0,
}

impl std::fmt::Display for SteamIdAccountType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            SteamIdAccountType::Individual => "Individual",
            SteamIdAccountType::Multiseat => "Multiseat",
            SteamIdAccountType::GameServer => "Game server",
            SteamIdAccountType::AnonymousGameServer => "Anonymous game server",
            SteamIdAccountType::Pending => "Pending",
            SteamIdAccountType::ContentServer => "Content server",
            SteamIdAccountType::Clan => "Clan",
            SteamIdAccountType::Chat => "Chat",
            SteamIdAccountType::PeerToPeerSuperSeeder => "Peer to peer superseeder",
            SteamIdAccountType::AnonymousUser => "Anonymous user",
            SteamIdAccountType::Invalid => "Invalid",
        })
    }
}

lazy_static::lazy_static! {
    static ref STEAM_ID_ACCOUNT_TYPE_MAP: HashMap<char, SteamIdAccountType> = vec![
        ('I', SteamIdAccountType::Invalid),
        ('U', SteamIdAccountType::Individual),
        ('M', SteamIdAccountType::Multiseat),
        ('G', SteamIdAccountType::GameServer),
        ('A', SteamIdAccountType::AnonymousGameServer),
        ('P', SteamIdAccountType::Pending),
        ('C', SteamIdAccountType::ContentServer),
        ('g', SteamIdAccountType::Clan),
        ('T', SteamIdAccountType::Chat),
        ('L', SteamIdAccountType::Chat),
        ('c', SteamIdAccountType::Chat),
        ('a', SteamIdAccountType::AnonymousUser),
    ].into_iter().collect();
}

impl std::str::FromStr for SteamIdAccountType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> crate::error::Result<Self> {
        if s.len() > 1 {
            Err("The passed string is too long to represent a steam account type.".into())
        } else {
            Self::try_from(s.chars().next().unwrap()) // safe cuz we have checked the string length.
        }
    }
}

impl std::convert::TryFrom<char> for SteamIdAccountType {
    type Error = crate::error::Error;

    fn try_from(value: char) -> crate::error::Result<Self> {
        Ok(*STEAM_ID_ACCOUNT_TYPE_MAP
            .get(&value)
            .ok_or_else(|| "The string doesn't contain a correct id account type.".to_owned())?)
    }
}

impl std::convert::TryFrom<u64> for SteamIdAccountType {
    type Error = crate::error::Error;

    fn try_from(value: u64) -> crate::error::Result<Self> {
        Ok(match value {
            0 => SteamIdAccountType::Invalid,
            1 => SteamIdAccountType::Individual,
            2 => SteamIdAccountType::Multiseat,
            3 => SteamIdAccountType::GameServer,
            4 => SteamIdAccountType::AnonymousGameServer,
            5 => SteamIdAccountType::Pending,
            6 => SteamIdAccountType::ContentServer,
            7 => SteamIdAccountType::Clan,
            8 => SteamIdAccountType::Chat,
            9 => SteamIdAccountType::PeerToPeerSuperSeeder,
            10 => SteamIdAccountType::AnonymousUser,
            _ => return Err("The number doesn't represent a correct steam id universe.".into()),
        })
    }
}

impl std::convert::TryFrom<u32> for SteamIdAccountType {
    type Error = crate::error::Error;

    fn try_from(value: u32) -> crate::error::Result<Self> {
        Self::try_from(value as u64)
    }
}

impl std::convert::TryFrom<u16> for SteamIdAccountType {
    type Error = crate::error::Error;

    fn try_from(value: u16) -> crate::error::Result<Self> {
        Self::try_from(value as u64)
    }
}

impl std::convert::TryFrom<u8> for SteamIdAccountType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        Self::try_from(value as u64)
    }
}

/// Steam Id information.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct SteamIdInfo {
    /// The universe this id belongs to.
    pub universe: SteamIdUniverse,
    /// The type of the account.
    pub account_type: SteamIdAccountType,
    /// Account instance.
    pub instance: u32,
    /// Account number.
    pub account: u32,
    /// The authentication server used by the account, either `1` or `0`.
    pub authentication_server: u8,
}

/// Steam Id 64.
/// Example: `7656119xxxxxxxxxx`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct SteamId64(pub u64);
impl SteamId64 {
    /// Get a detailed information about the steam account from the steam id.
    pub fn info(&self) -> crate::error::Result<SteamIdInfo> {
        let mut iter = BitIterator::new(self.0, 8);
        Ok(SteamIdInfo {
            universe: iter.next().unwrap().try_into()?,
            account_type: iter.next_bits::<u8>(4).unwrap().try_into()?,
            instance: iter.next_bits::<u32>(20).unwrap(),
            account: iter.next_bits::<u32>(31).unwrap(),
            authentication_server: iter.next_bits::<u8>(1).unwrap(),
        })
    }
}

/// Steam Id 32.
/// Example: `STEAM_0:X:XXXXXXXX`.
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct SteamId32(pub String);

/// Steam Id 3.
/// Example: `U:1:xxxxxxxx`.
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct SteamId3(pub String);
impl SteamId3 {
    /// Get a detailed information about the steam account from the steam id.
    /// This information may not contain all the fields correct as to how `SteamId64` can due to unavailable
    /// documentation of this format.
    pub fn info(&self) -> crate::error::Result<SteamIdInfo> {
        let split: Vec<&str> = self.0.split(':').collect();
        let authentication_server: u8 = split[1].parse()?;
        let account: u32 = split[2].parse()?;
        Ok(SteamIdInfo {
            /// The universe is hard to know for sure, as from `SteamId3` format it is unknown how to
            /// parse it.
            universe: SteamIdUniverse::IndividualOrUnspecified,
            account_type: SteamIdAccountType::try_from(self.0.chars().next().unwrap())?,
            instance: DEFAULT_STEAM_ACCOUNT_INSTANCE as u32,
            account,
            authentication_server,
        })
    }
}

/// <https://developer.valvesoftware.com/wiki/SteamID>
/// Holds a steam id in various formats.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SteamId {
    /// Steam ID in a single integer format (SteamID64).
    /// Example: `7656119xxxxxxxxxx`.
    Id64(SteamId64),
    /// Steam ID 32 in the default format, starting with `STEAM_0`.
    /// Example: `STEAM_0:X:XXXXXXXX`.
    Id32(SteamId32),
    /// Steam ID in the format called "Steam ID 3".
    /// Example: `U:1:xxxxxxxx`.
    Id3(SteamId3),
}

impl std::convert::TryFrom<SteamId32> for SteamId3 {
    type Error = crate::error::Error;

    fn try_from(id: SteamId32) -> crate::error::Result<SteamId3> {
        let re = Regex::new(r"^STEAM_\d:\d:\d+$").unwrap();
        if re.is_match(&id.0) {
            let split: Vec<&str> = id.0.split(':').collect();
            let first: u64 = split[1].parse()?;
            let second: u64 = split[2].parse()?;
            let id = second * 2 + first;
            return Ok(SteamId3(format!("U:1:{}", id)));
        }
        Err("The steam id provided is not in the SteamID32 format.".into())
    }
}

impl std::convert::TryFrom<SteamId3> for SteamId32 {
    type Error = crate::error::Error;

    fn try_from(id: SteamId3) -> crate::error::Result<SteamId32> {
        let re = Regex::new(r"^(\w):(\d):(\d+)$").unwrap();
        if let Some(captures) = re.captures(&id.0) {
            if captures.len() < 4 {
                return Err("The steam id provided is not in the SteamID3 format.".into());
            }
            let _account_type = SteamIdAccountType::from_str(captures.get(1).unwrap().as_str())?;
            // Probably this is not an authentication server, I don't know then what it can be.
            let _authentication_server: u8 = captures.get(2).unwrap().as_str().parse()?;
            let account: u32 = captures.get(3).unwrap().as_str().parse()?;
            if account % 2 == 0 {
                return Ok(SteamId32(format!("STEAM_0:0:{}", account / 2)));
            } else {
                return Ok(SteamId32(format!("STEAM_0:1:{}", (account - 1) / 2)));
            }
        }
        Err("The steam id provided is not in the SteamID3 format.".into())
    }
}

impl TryFrom<SteamId64> for SteamId32 {
    type Error = crate::error::Error;

    fn try_from(id: SteamId64) -> crate::error::Result<SteamId32> {
        // Here we go off-spec as it seems they have implemented it wrong.
        // The first digit after the `"STEAM_"` should be the universe number, but it
        // is just either always zero or is not a universe number.
        // Hence it is hardcoded to be 0 when we convert the `SteamId64` to `SteamId32`.
        // It works, but off-spec.
        let info = id.info()?;
        Ok(SteamId32(format!(
            "STEAM_0:{}:{}",
            info.authentication_server, info.account
        )))
    }
}

impl TryFrom<SteamId32> for SteamId64 {
    type Error = crate::error::Error;

    fn try_from(id: SteamId32) -> crate::error::Result<SteamId64> {
        let re = Regex::new(r"^STEAM_(\d):(\d):(\d+)$").unwrap();
        if let Some(captures) = re.captures(&id.0) {
            if captures.len() < 4 {
                return Err("The steam id provided is not in the SteamID32 format.".into());
            }
            let mut universe: u64 = captures.get(1).unwrap().as_str().parse()?;
            let authentication_server: u64 = captures.get(2).unwrap().as_str().parse()?;
            let account: u32 = captures.get(3).unwrap().as_str().parse()?;
            if universe == 0 {
                universe = 1;
            }
            let num = u64::from_str_radix(
                &format!(
                    "{:08b}{:04b}{:020b}{:031b}{:b}",
                    universe,
                    DEFAULT_STEAM_ACCOUNT_TYPE,
                    DEFAULT_STEAM_ACCOUNT_INSTANCE,
                    account,
                    authentication_server
                ),
                2,
            )?;
            return Ok(SteamId64(num));
        }
        Err("The steam id provided is not in the SteamID32 format.".into())
    }
}

impl TryFrom<SteamId3> for SteamId64 {
    type Error = crate::error::Error;

    fn try_from(id: SteamId3) -> crate::error::Result<SteamId64> {
        SteamId64::try_from(SteamId32::try_from(id)?)
    }
}

impl SteamId {
    /// Converts (if needed) the current id format into id64.
    pub fn id64(&self) -> crate::error::Result<SteamId64> {
        match self {
            SteamId::Id64(num) => Ok(*num),
            SteamId::Id32(id) => SteamId64::try_from(id.clone()),
            SteamId::Id3(id) => SteamId64::try_from(id.clone()),
        }
    }

    /// Converts (if needed) the current id format into id32.
    pub fn id32(&self) -> crate::error::Result<SteamId32> {
        match self {
            SteamId::Id64(num) => SteamId32::try_from(*num),
            SteamId::Id32(id) => Ok(id.clone()),
            SteamId::Id3(id) => SteamId32::try_from(id.clone()),
        }
    }

    /// Consumes the object and converts it into a steam id in the id64 format.
    pub fn into_id64(self) -> crate::error::Result<SteamId> {
        Ok(SteamId::Id64(self.id64()?))
    }

    /// Consumes the object and converts it into a steam id in the id32 format.
    pub fn into_id32(self) -> crate::error::Result<SteamId> {
        Ok(SteamId::Id32(self.id32()?))
    }
}

impl std::str::FromStr for SteamId {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(id64) = value.parse::<u64>() {
            return Ok(SteamId::Id64(SteamId64(id64)));
        }

        let re = Regex::new(r"^STEAM_\d:\d:\d+$").unwrap();
        if re.is_match(value) {
            return Ok(SteamId::Id32(SteamId32(value.to_owned())));
        }

        let re = Regex::new(r"^\w:\d:\d+$").unwrap();
        if re.is_match(value) {
            return Ok(SteamId::Id3(SteamId3(value.to_owned())));
        }

        Err(format!("Not a valid steam id value: {}", value))
    }
}

impl std::fmt::Display for SteamId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SteamId::Id64(num) => fmt.write_str(&num.0.to_string()),
            SteamId::Id32(s) => fmt.write_str(&s.0),
            SteamId::Id3(s) => fmt.write_str(&s.0),
        }
    }
}

// As we need to be able to serialize the id, we should come to a least common denominator and the thing we
// can use the best. Here the integer kind of the id is simply the best: less memory usage compared to strings,
// provides more information, easier to work with.
#[cfg(feature = "serialization")]
impl serde::Serialize for SteamId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(
            self.id64()
                .map_err(|e| serde::ser::Error::custom(e.description()))?
                .0,
        )
    }
}

#[cfg(feature = "serialization")]
struct SteamIdVisitor;

#[cfg(feature = "serialization")]
impl<'de> Visitor<'de> for SteamIdVisitor {
    type Value = SteamId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a steam id in the format of Steam ID64, Steam ID, or Steam ID 3.")
    }

    fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(SteamId::Id64(SteamId64(value)))
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        SteamId::from_str(value).map_err(E::custom)
    }
}
#[cfg(feature = "serialization")]
impl<'de> serde::Deserialize<'de> for SteamId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<SteamId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SteamIdVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam_id_conversions() {
        let steam_id_64 = SteamId64(76561197983318796);
        let steam_id_3 = SteamId3("U:1:23053068".to_owned());
        assert_eq!(
            SteamId32::try_from(steam_id_3.clone()).unwrap(),
            SteamId32("STEAM_0:0:11526534".to_owned())
        );
        assert_eq!(
            SteamId64::try_from(steam_id_3).unwrap(),
            SteamId64(76561197983318796)
        );
        assert_eq!(
            SteamId32::try_from(steam_id_64).unwrap(),
            SteamId32("STEAM_0:0:11526534".to_owned())
        );
    }

    #[cfg(feature = "serialization")]
    #[test]
    fn steam_id_enum_serialize() {
        assert_eq!(
            serde_json::to_string(&SteamId::Id64(SteamId64(76561197983318796))).unwrap(),
            "76561197983318796"
        );
        assert_eq!(
            serde_json::to_string(&SteamId::Id32(SteamId32("STEAM_0:0:11526534".to_owned())))
                .unwrap(),
            "76561197983318796"
        );
        assert_eq!(
            serde_json::to_string(&SteamId::Id3(SteamId3("U:1:23053068".to_owned()))).unwrap(),
            "76561197983318796"
        );
    }

    #[cfg(feature = "serialization")]
    #[test]
    fn steam_id_enum_deserialize() {
        let s = "\"STEAM_0:0:11526534\"";
        let id = serde_json::from_str::<SteamId>(s).unwrap();
        assert_eq!(
            id,
            SteamId::Id32(SteamId32("STEAM_0:0:11526534".to_owned()))
        );
        let s = "76561197983318796";
        let id = serde_json::from_str::<SteamId>(s).unwrap();
        assert_eq!(id, SteamId::Id64(SteamId64(76561197983318796)));
        let s = "\"76561197983318796\"";
        let id = serde_json::from_str::<SteamId>(s).unwrap();
        assert_eq!(id, SteamId::Id64(SteamId64(76561197983318796)));
        let s = "\"U:1:23053068\"";
        let id = serde_json::from_str::<SteamId>(s).unwrap();
        assert_eq!(id, SteamId::Id3(SteamId3("U:1:23053068".to_owned())));
    }
}
