//! Steam ID and all it needs and may represent.
//!
//! Please check out the structures for more information.
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
pub enum Universe {
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

impl std::fmt::Display for Universe {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            Universe::IndividualOrUnspecified => "Individual or unspecified",
            Universe::Public => "Public",
            Universe::Beta => "Beta",
            Universe::Internal => "Internal",
            Universe::Developer => "Developer",
            Universe::Rc => "RC",
        })
    }
}

impl std::convert::TryFrom<u64> for Universe {
    type Error = crate::error::Error;

    fn try_from(value: u64) -> crate::error::Result<Self> {
        Ok(match value {
            0 => Universe::IndividualOrUnspecified,
            1 => Universe::Public,
            2 => Universe::Beta,
            3 => Universe::Internal,
            4 => Universe::Developer,
            5 => Universe::Rc,
            _ => return Err("The number doesn't represent a correct steam id universe.".into()),
        })
    }
}

impl std::convert::TryFrom<u32> for Universe {
    type Error = crate::error::Error;

    fn try_from(value: u32) -> crate::error::Result<Self> {
        Self::try_from(u64::from(value))
    }
}

impl std::convert::TryFrom<u16> for Universe {
    type Error = crate::error::Error;

    fn try_from(value: u16) -> crate::error::Result<Self> {
        Self::try_from(u64::from(value))
    }
}

impl std::convert::TryFrom<u8> for Universe {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        Self::try_from(u64::from(value))
    }
}

/// Steam Id Account type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum AccountType {
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

impl std::fmt::Display for AccountType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            AccountType::Individual => "Individual",
            AccountType::Multiseat => "Multiseat",
            AccountType::GameServer => "Game server",
            AccountType::AnonymousGameServer => "Anonymous game server",
            AccountType::Pending => "Pending",
            AccountType::ContentServer => "Content server",
            AccountType::Clan => "Clan",
            AccountType::Chat => "Chat",
            AccountType::PeerToPeerSuperSeeder => "Peer to peer superseeder",
            AccountType::AnonymousUser => "Anonymous user",
            AccountType::Invalid => "Invalid",
        })
    }
}

lazy_static::lazy_static! {
    static ref ACCOUNT_TYPE_MAP: HashMap<char, AccountType> = vec![
        ('I', AccountType::Invalid),
        ('U', AccountType::Individual),
        ('M', AccountType::Multiseat),
        ('G', AccountType::GameServer),
        ('A', AccountType::AnonymousGameServer),
        ('P', AccountType::Pending),
        ('C', AccountType::ContentServer),
        ('g', AccountType::Clan),
        ('T', AccountType::Chat),
        ('L', AccountType::Chat),
        ('c', AccountType::Chat),
        ('a', AccountType::AnonymousUser),
    ].into_iter().collect();

    static ref ID32_REGEXP: Regex = {
        Regex::new(r"^STEAM_(\d):(\d):(\d+)$").unwrap()
    };

    static ref ID3_REGEXP: Regex = {
        Regex::new(r"^(\w):(\d):(\d+)$").unwrap()
    };
}

impl std::str::FromStr for AccountType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> crate::error::Result<Self> {
        if s.len() > 1 {
            Err("The passed string is too long to represent a steam account type.".into())
        } else {
            Self::try_from(s.chars().next().unwrap()) // safe cuz we have checked the string length.
        }
    }
}

impl std::convert::TryFrom<char> for AccountType {
    type Error = crate::error::Error;

    fn try_from(value: char) -> crate::error::Result<Self> {
        Ok(*ACCOUNT_TYPE_MAP
            .get(&value)
            .ok_or_else(|| "The string doesn't contain a correct id account type.".to_owned())?)
    }
}

impl std::convert::TryFrom<u64> for AccountType {
    type Error = crate::error::Error;

    fn try_from(value: u64) -> crate::error::Result<Self> {
        Ok(match value {
            0 => AccountType::Invalid,
            1 => AccountType::Individual,
            2 => AccountType::Multiseat,
            3 => AccountType::GameServer,
            4 => AccountType::AnonymousGameServer,
            5 => AccountType::Pending,
            6 => AccountType::ContentServer,
            7 => AccountType::Clan,
            8 => AccountType::Chat,
            9 => AccountType::PeerToPeerSuperSeeder,
            10 => AccountType::AnonymousUser,
            _ => return Err("The number doesn't represent a correct steam id universe.".into()),
        })
    }
}

impl std::convert::TryFrom<u32> for AccountType {
    type Error = crate::error::Error;

    fn try_from(value: u32) -> crate::error::Result<Self> {
        Self::try_from(u64::from(value))
    }
}

impl std::convert::TryFrom<u16> for AccountType {
    type Error = crate::error::Error;

    fn try_from(value: u16) -> crate::error::Result<Self> {
        Self::try_from(u64::from(value))
    }
}

impl std::convert::TryFrom<u8> for AccountType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        Self::try_from(u64::from(value))
    }
}

/// Steam Id information.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Info {
    /// The universe this id belongs to.
    pub universe: Universe,
    /// The type of the account.
    pub account_type: AccountType,
    /// Account instance.
    pub instance: u32,
    /// Account number.
    pub account: u32,
    /// The authentication server used by the account, either `1` or `0`.
    pub authentication_server: u8,
}

/// Steam Id 64.
/// Example: `7656119xxxxxxxxxx`.
#[allow(clippy::clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Id64(pub u64);
impl Id64 {
    /// Get a detailed information about the steam account from the steam id.
    ///
    /// # Errors
    /// Returns an error if the account type or universe are incorrect.
    ///
    /// # Panics
    /// Panics when it suddenly becomes impossible to iterate over the bits in the steam id, what in fact can't happen ever.
    pub fn info(self) -> crate::error::Result<Info> {
        let mut iter = BitIterator::new(self.0, 8);
        Ok(Info {
            universe: iter.next().unwrap().try_into()?,
            account_type: iter.next_bits::<u8>(4).unwrap().try_into()?,
            instance: iter.next_bits::<u32>(20).unwrap(),
            account: iter.next_bits::<u32>(31).unwrap(),
            authentication_server: iter.next_bits::<u8>(1).unwrap(),
        })
    }

    /// Create a new Id64 with only three parameters passed, all others will be constructed using the default,
    /// most commonly used values.
    ///
    /// # Errors
    /// Throws `crate::error::Error` if it is impossible to create such an id due to the width of the values passed.
    /// To avoid this, the values passed must be used according to the specification of the steam id 64.
    /// Also may throw a error if the `account_type` passed is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// let old_id = steamidfx::id::Id64(76561197983318796);
    /// let info = old_id.info().unwrap();
    /// let new_id = steamidfx::id::Id64::new_simple(info.universe, info.authentication_server, info.account).unwrap();
    /// assert_eq!(old_id, new_id);
    /// ```
    pub fn new_simple(
        universe: Universe,
        authentication_server: u8,
        account: u32,
    ) -> crate::error::Result<Id64> {
        Id64::new_full(
            universe,
            AccountType::try_from(DEFAULT_STEAM_ACCOUNT_TYPE)?,
            DEFAULT_STEAM_ACCOUNT_INSTANCE.into(),
            authentication_server,
            account,
        )
    }

    /// Create a new Id64 with all the values specified explicitly.
    ///
    /// # Errors
    /// Throws `crate::error::Error` if it is impossible to create such an id due to the width of the values passed.
    /// To avoid this, the values passed must be used according to the specification of the steam id 64.
    /// Also may throw a error if the `account_type` passed is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// let old_id = steamidfx::id::Id64(76561197983318796);
    /// let info = old_id.info().unwrap();
    /// let new_id = steamidfx::id::Id64::new_full(info.universe, info.account_type, info.instance, info.authentication_server, info.account).unwrap();
    /// assert_eq!(old_id, new_id);
    /// ```
    pub fn new_full(
        universe: Universe,
        account_type: AccountType,
        account_instance: u32,
        authentication_server: u8,
        account: u32,
    ) -> crate::error::Result<Id64> {
        let num = u64::from_str_radix(
            &format!(
                "{:08b}{:04b}{:020b}{:031b}{:b}",
                universe as u8,
                account_type as u8,
                account_instance,
                account,
                authentication_server
            ),
            2,
        )?;
        Ok(Id64(num))
    }
}

/// Steam Id 32.
/// Example: `STEAM_0:X:XXXXXXXX`.
#[allow(clippy::clippy::module_name_repetitions)]
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Id32(pub String);

/// Steam Id 3.
/// Example: `U:1:xxxxxxxx`.
#[allow(clippy::clippy::module_name_repetitions)]
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Id3(pub String);
impl Id3 {
    /// Get a detailed information about the steam account from the steam id.
    /// This information may not contain all the fields correct as to how `SteamId64` can due to unavailable
    /// documentation of this format.
    ///
    /// # Errors
    /// Returns an error if the id is of an incorrect format.
    pub fn info(&self) -> crate::error::Result<Info> {
        let split: Vec<&str> = self.0.split(':').collect();
        if split.len() < 3 {
            return Err(crate::error::ErrorKind::InvalidSteamId(self.0.clone()).into());
        }
        let authentication_server: u8 = split[1].parse()?;
        let account: u32 = split[2].parse()?;
        Ok(Info {
            /// The universe is hard to know for sure, as from `SteamId3` format it is unknown how to
            /// parse it.
            universe: Universe::IndividualOrUnspecified,
            account_type: AccountType::from_str(split[0])?,
            instance: u32::from(DEFAULT_STEAM_ACCOUNT_INSTANCE),
            account,
            authentication_server,
        })
    }
}

/// <https://developer.valvesoftware.com/wiki/SteamID>
/// Holds a steam id in various formats.
///
/// # Example
///
/// ```rust
/// use std::convert::TryFrom;
///
/// let steam_id_64 = steamidfx::id::Id64(76561197983318796);
/// let steam_id_3 = steamidfx::id::Id3("U:1:23053068".to_owned());
/// assert_eq!(
///     steamidfx::id::Id32::try_from(steam_id_3.clone()).unwrap(),
///     steamidfx::id::Id32("STEAM_0:0:11526534".to_owned())
/// );
/// assert_eq!(
///     steamidfx::id::Id64::try_from(steam_id_3).unwrap(),
///     steamidfx::id::Id64(76561197983318796)
/// );
/// assert_eq!(
///     steamidfx::id::Id32::try_from(steam_id_64).unwrap(),
///     steamidfx::id::Id32("STEAM_0:0:11526534".to_owned())
/// );
///
/// // The most preferred way to construct Ids is using the fallible `TryFrom`.
/// // This will make sure that the constructed `ID`s are correct as some of
/// // the fields of it are required to be of some certain ranges of values.
/// use std::str::FromStr;
///
/// let id_64 = steamidfx::id::Id::from_str("76561197983318796").unwrap();
/// let id_64_2 = steamidfx::id::Id::try_from(76561197983318796).unwrap();
/// let id_32 = steamidfx::id::Id::from_str("STEAM_0:0:11526534").unwrap();
/// let id_3 = steamidfx::id::Id::from_str("U:1:23053068").unwrap();
/// // This way you'll make sure after unpacking the `Result` that the value is correct
/// // at least, according to the specification.
/// ```

#[allow(clippy::clippy::module_name_repetitions)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Id {
    /// Steam ID in a single integer format (SteamID64).
    /// Example: `7656119xxxxxxxxxx`.
    Id64(Id64),
    /// Steam ID 32 in the default format, starting with `STEAM_0`.
    /// Example: `STEAM_0:X:XXXXXXXX`.
    Id32(Id32),
    /// Steam ID in the format called "Steam ID 3".
    /// Example: `U:1:xxxxxxxx`.
    Id3(Id3),
}

impl std::convert::TryFrom<u64> for Id {
    type Error = crate::error::Error;

    fn try_from(id: u64) -> crate::error::Result<Id> {
        let id = Id64(id);
        if let Err(e) = id.info() {
            return Err(e);
        }
        Ok(Id::Id64(id))
    }
}

impl std::convert::TryFrom<Id32> for Id3 {
    type Error = crate::error::Error;

    fn try_from(id: Id32) -> crate::error::Result<Id3> {
        if ID32_REGEXP.is_match(&id.0) {
            let split: Vec<&str> = id.0.split(':').collect();
            let first: u64 = split[1].parse()?;
            let second: u64 = split[2].parse()?;
            let num = second * 2 + first;
            return Ok(Id3(format!("U:1:{}", num)));
        }
        Err("The steam id provided is not in the SteamID32 format.".into())
    }
}

impl std::convert::TryFrom<Id3> for Id32 {
    type Error = crate::error::Error;

    fn try_from(id: Id3) -> crate::error::Result<Id32> {
        if let Some(captures) = ID3_REGEXP.captures(&id.0) {
            if captures.len() < 4 {
                return Err("The steam id provided is not in the SteamID3 format.".into());
            }
            let _account_type = AccountType::from_str(captures.get(1).unwrap().as_str())?;
            // Probably this is not an authentication server, but I don't know then what it can be.
            let _authentication_server: u8 = captures.get(2).unwrap().as_str().parse()?;
            let account: u32 = captures.get(3).unwrap().as_str().parse()?;
            if account % 2 == 0 {
                return Ok(Id32(format!("STEAM_0:0:{}", account / 2)));
            }

            return Ok(Id32(format!("STEAM_0:1:{}", (account - 1) / 2)));
        }
        Err("The steam id provided is not in the SteamID3 format.".into())
    }
}

impl TryFrom<Id64> for Id32 {
    type Error = crate::error::Error;

    fn try_from(id: Id64) -> crate::error::Result<Id32> {
        // Here we go off-spec as it seems they have implemented it wrong.
        // The first digit after the `"STEAM_"` should be the universe number, but it
        // is just either always zero or is not a universe number.
        // Hence it is hardcoded to be 0 when we convert the `SteamId64` to `SteamId32`.
        // It works, but off-spec.
        let info = id.info()?;
        Ok(Id32(format!(
            "STEAM_0:{}:{}",
            info.authentication_server, info.account
        )))
    }
}

impl TryFrom<Id32> for Id64 {
    type Error = crate::error::Error;

    fn try_from(id: Id32) -> crate::error::Result<Id64> {
        if let Some(captures) = ID32_REGEXP.captures(&id.0) {
            if captures.len() < 4 {
                return Err("The steam id provided is not in the SteamID32 format.".into());
            }
            let mut universe: u8 = captures.get(1).unwrap().as_str().parse()?;
            let authentication_server: u8 = captures.get(2).unwrap().as_str().parse()?;
            let account: u32 = captures.get(3).unwrap().as_str().parse()?;
            if universe == 0 {
                universe = 1;
            }
            return Id64::new_simple(
                Universe::try_from(universe)?,
                authentication_server,
                account,
            );
        }
        Err("The steam id provided is not in the SteamID32 format.".into())
    }
}

impl TryFrom<Id3> for Id64 {
    type Error = crate::error::Error;

    fn try_from(id: Id3) -> crate::error::Result<Id64> {
        Id64::try_from(Id32::try_from(id)?)
    }
}

impl Id {
    /// Converts (if needed) the current id format into id64.
    ///
    /// # Errors
    /// Throws `crate::error::Error` if it was impossible to extract the steam id 64.
    pub fn id64(&self) -> crate::error::Result<Id64> {
        match self {
            Id::Id64(num) => Ok(*num),
            Id::Id32(id) => Id64::try_from(id.clone()),
            Id::Id3(id) => Id64::try_from(id.clone()),
        }
    }

    /// Converts (if needed) the current id format into id32.
    ///
    /// # Errors
    /// Throws `crate::error::Error` if it was impossible to extract the steam id 32.
    pub fn id32(&self) -> crate::error::Result<Id32> {
        match self {
            Id::Id64(num) => Id32::try_from(*num),
            Id::Id32(id) => Ok(id.clone()),
            Id::Id3(id) => Id32::try_from(id.clone()),
        }
    }

    /// Consumes the object and converts it into a steam id in the id64 format.
    ///
    /// # Errors
    /// Throws `crate::error::Error` if it was impossible to extract the steam id 64.
    pub fn into_id64(self) -> crate::error::Result<Id> {
        Ok(Id::Id64(self.id64()?))
    }

    /// Consumes the object and converts it into a steam id in the id32 format.
    ///
    /// # Errors
    /// Throws `crate::error::Error` if it was impossible to extract the steam id 32.
    pub fn into_id32(self) -> crate::error::Result<Id> {
        Ok(Id::Id32(self.id32()?))
    }

    /// Attempts to compare two ids. Returns `true` when they are representing
    /// the same values, even using different formats.
    /// The conventional `Eq` and `PartialEq` traits derived will be checking
    /// the enum variants and strings inside, which is sometimes not what we want.
    /// Sometimes we just want to say that these two `Id` objects represent the
    /// same person, or an entity of valve. For that, we aren't interested in the
    /// format of the ID, but rather in the data it provides.
    ///
    /// # Errors
    /// Returns an error when it is impossible to convert both ids to a single
    /// format: `id64`. This format is used as it is the most commonly used one,
    /// and provides the most information possible. Hence, all the ID formats
    /// should ideally be convertible to it just fine, but we are still performing
    /// all the checks to be a bit more cautious about it.
    ///
    /// # Examples
    /// ```rust
    /// use std::str::FromStr;
    /// use std::convert::TryFrom;
    ///
    /// let id_64 = steamidfx::id::Id::try_from(76561197983318796).unwrap();
    /// let id_32 = steamidfx::id::Id::from_str("STEAM_0:0:11526534").unwrap();
    /// let id_3 = steamidfx::id::Id::from_str("U:1:23053068").unwrap();
    /// assert!(id_3.is_same(&id_32).unwrap());
    /// assert!(id_32.is_same(&id_64).unwrap());
    /// ```
    pub fn is_same(&self, other: &Id) -> crate::error::Result<bool> {
        Ok(self.id64()? == other.id64()?)
    }
}

impl std::str::FromStr for Id {
    type Err = crate::error::Error;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(id64) = value.parse::<u64>() {
            return Ok(Id::Id64(Id64(id64)));
        }

        if ID32_REGEXP.is_match(value) {
            return Ok(Id::Id32(Id32(value.to_owned())));
        }

        if ID3_REGEXP.is_match(value) {
            return Ok(Id::Id3(Id3(value.to_owned())));
        }

        Err(crate::error::ErrorKind::InvalidSteamId(format!(
            "Not a valid steam id value: {}",
            value
        ))
        .into())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Id::Id64(num) => fmt.write_str(&num.0.to_string()),
            Id::Id32(s) => fmt.write_str(&s.0),
            Id::Id3(s) => fmt.write_str(&s.0),
        }
    }
}

// As we need to be able to serialize the id, we should come to a least common denominator and the thing we
// can use the best. Here the integer kind of the id is simply the best: less memory usage compared to strings,
// provides more information, easier to work with.
#[cfg(feature = "serialization")]
impl serde::Serialize for Id {
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
struct IdVisitor;

#[cfg(feature = "serialization")]
impl<'de> Visitor<'de> for IdVisitor {
    type Value = Id;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a steam id in the format of  ID64,  ID, or  ID 3.")
    }

    fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Id::Id64(Id64(value)))
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Id::from_str(value).map_err(E::custom)
    }
}
#[cfg(feature = "serialization")]
impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Id, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(IdVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialization")]
    #[allow(clippy::unreadable_literal)]
    #[test]
    fn steam_id_enum_serialize() {
        assert_eq!(
            serde_json::to_string(&Id::Id64(Id64(76561197983318796))).unwrap(),
            "76561197983318796"
        );
        assert_eq!(
            serde_json::to_string(&Id::Id32(Id32("STEAM_0:0:11526534".to_owned()))).unwrap(),
            "76561197983318796"
        );
        assert_eq!(
            serde_json::to_string(&Id::Id3(Id3("U:1:23053068".to_owned()))).unwrap(),
            "76561197983318796"
        );
    }

    #[cfg(feature = "serialization")]
    #[allow(clippy::unreadable_literal)]
    #[test]
    fn steam_id_enum_deserialize() {
        let strid32 = "\"STEAM_0:0:11526534\"";
        let id32 = serde_json::from_str::<Id>(strid32).unwrap();
        assert_eq!(id32, Id::Id32(Id32("STEAM_0:0:11526534".to_owned())));
        let strid64 = "76561197983318796";
        let id64 = serde_json::from_str::<Id>(strid64).unwrap();
        assert_eq!(id64, Id::Id64(Id64(76561197983318796)));
        let strid64s = "\"76561197983318796\"";
        let id64s = serde_json::from_str::<Id>(strid64s).unwrap();
        assert_eq!(id64s, Id::Id64(Id64(76561197983318796)));
        let strid3 = "\"U:1:23053068\"";
        let id3 = serde_json::from_str::<Id>(strid3).unwrap();
        assert_eq!(id3, Id::Id3(Id3("U:1:23053068".to_owned())));
    }
}
