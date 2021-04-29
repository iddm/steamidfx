//! The requests feature implementation which allows making requests to different services for working with
//! steam id.

/// Steam profile from <https://steamid.co>.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Deserialize))]
pub struct SteamCoProfile {
    /// The steam id of this profile.
    #[cfg_attr(feature = "serialization", serde(rename = "steamID64"))]
    pub steam_id: crate::id::Id,
    /// The name of the profile in steam.
    #[cfg_attr(feature = "serialization", serde(rename = "steamID"))]
    pub name: String,
    /// The date the profile has been registered since.
    #[cfg_attr(feature = "serialization", serde(rename = "memberSince"))]
    pub member_since: String,
    #[cfg_attr(feature = "serialization", serde(rename = "onlineState"))]
    /// Current online status of the player.
    pub online_state: crate::id::OnlineState,
    #[cfg_attr(feature = "serialization", serde(rename = "vacBanned"))]
    #[cfg_attr(
        feature = "serialization",
        serde(deserialize_with = "serde_aux::field_attributes::deserialize_bool_from_anything")
    )]
    /// Whether this profile has been banned by VAC or not.
    pub vac_banned: bool,
    /// Current state message of the profile.
    #[cfg_attr(feature = "serialization", serde(rename = "stateMessage"))]
    pub state_message: String,
    // TODO parse more fields
}

/// Creates a URL which can be used to perform an http request for getting steam account information
/// by steam id.
///
/// # Errors
/// Throws `crate::error::Error` if it was impossible to extract the steam id 64 from the passed steam id object.
pub fn get_steamco_profile_url(id: &crate::id::Id) -> crate::error::Result<String> {
    Ok(format!(
        "http://steamid.co/php/api.php?action=steamID64&id={}",
        id.id64()?.0
    ))
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serialization")]
    #[allow(clippy::unreadable_literal)]
    #[allow(clippy::too_many_lines)]
    #[test]
    fn steamidco_profile_parse_ok() {
        let string = r#"
{
  "steamID64": "76561197992396121",
  "steamID": "Z U L U A",
  "onlineState": "offline",
  "stateMessage": "Last Online 8 hrs, 59 mins ago",
  "privacyState": "public",
  "visibilityState": "3",
  "avatarIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/a6\/a602f000f427adc5eb1a3d80eb073cac4df300a6.jpg",
  "avatarMedium": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/a6\/a602f000f427adc5eb1a3d80eb073cac4df300a6_medium.jpg",
  "avatarFull": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/a6\/a602f000f427adc5eb1a3d80eb073cac4df300a6_full.jpg",
  "vacBanned": "0",
  "tradeBanState": "None",
  "isLimitedAccount": "0",
  "customURL": "ZuluaQC",
  "memberSince": "September 7th, 2007",
  "steamRating": {

  },
  "hoursPlayed2Wk": "0.0",
  "headline": {

  },
  "location": "Montpellier, Languedoc-Roussillon, France",
  "realname": "Axel",
  "summary": "No information given.",
  "mostPlayedGames": {
    "mostPlayedGame": [
      {
        "gameName": "Quake Champions",
        "gameLink": "http:\/\/steamcommunity.com\/app\/611500",
        "gameIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/611500\/539032fb1e8b41d0a48ff11bf63bfe7918ace24c.jpg",
        "gameLogo": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/611500\/f5e57fa0fe261e947ab4935481fc66d4ff8808aa.jpg",
        "gameLogoSmall": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/611500\/f5e57fa0fe261e947ab4935481fc66d4ff8808aa.jpg",
        "hoursPlayed": "17.0",
        "hoursOnRecord": "437",
        "statsName": "611500"
      },
      {
        "gameName": "Hunt: Showdown",
        "gameLink": "http:\/\/steamcommunity.com\/app\/594650",
        "gameIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/594650\/06c70772db40f714537f4d80c11859a68560a6b3.jpg",
        "gameLogo": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/594650\/7a7f693e439e96ad3d96d67d26bb7f7b96fe3271.jpg",
        "gameLogoSmall": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/594650\/7a7f693e439e96ad3d96d67d26bb7f7b96fe3271.jpg",
        "hoursPlayed": "12.7",
        "hoursOnRecord": "12.7",
        "statsName": "594650"
      },
      {
        "gameName": "Arma 3",
        "gameLink": "http:\/\/steamcommunity.com\/app\/107410",
        "gameIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/107410\/3212af52faf994c558bd622cb0f360c1ef295a6b.jpg",
        "gameLogo": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/107410\/b49962441a01f1f80b180af1293608dddf7df6b0.jpg",
        "gameLogoSmall": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/107410\/b49962441a01f1f80b180af1293608dddf7df6b0.jpg",
        "hoursPlayed": "4.2",
        "hoursOnRecord": "50",
        "statsName": "107410"
      },
      {
        "gameName": "Quake Live",
        "gameLink": "http:\/\/steamcommunity.com\/app\/282440",
        "gameIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/282440\/bac9828d3e193c948801b14660490576fbbf9f72.jpg",
        "gameLogo": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/282440\/bba7e836bc54e709020ee4d95c08f4dff1d23537.jpg",
        "gameLogoSmall": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/282440\/bba7e836bc54e709020ee4d95c08f4dff1d23537.jpg",
        "hoursPlayed": "3.9",
        "hoursOnRecord": "1,069",
        "statsName": "282440"
      },
      {
        "gameName": "Black Squad",
        "gameLink": "http:\/\/steamcommunity.com\/app\/550650",
        "gameIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/550650\/213a736d8c5b75998b671de25e6621d1e7bc122a.jpg",
        "gameLogo": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/550650\/2a58ab257ffa058656ffc8e0d60cfbff1f54d298.jpg",
        "gameLogoSmall": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/550650\/2a58ab257ffa058656ffc8e0d60cfbff1f54d298.jpg",
        "hoursPlayed": "2.2",
        "hoursOnRecord": "2.2",
        "statsName": "550650"
      },
      {
        "gameName": "BATTALION 1944",
        "gameLink": "http:\/\/steamcommunity.com\/app\/489940",
        "gameIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/489940\/f8ead72cebd6da5f0863a9b28e0a74850a32fcef.jpg",
        "gameLogo": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/489940\/c7794d72d290c771cc1a9e9de718de4f34c5e71b.jpg",
        "gameLogoSmall": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/apps\/489940\/c7794d72d290c771cc1a9e9de718de4f34c5e71b.jpg",
        "hoursPlayed": "0.7",
        "hoursOnRecord": "2.6"
      }
    ]
  },
  "groups": {
    "group": [
      {
        "@attributes": {
          "isPrimary": "1"
        },
        "groupID64": "103582791440668750",
        "groupName": "HDQLS",
        "groupURL": "HardDonsQLS",
        "headline": "\u115a\u115a \u115a\u115a \u115a\u115a\u115aHD Quakelive Server",
        "summary": "<div class=\"bb_h1\"><br> \u115a\u115a \u115a\u115a \u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591 <br> \u115a\u115a \u115a\u115a \u2591\u2591\u2588\u2591\u2588\u2591\u2588\u2591\u2588\u2580\u2580\u2580\u2591\u2588\u2591\u2591\u2591\u2591\u2588\u2580\u2580\u2580\u2591\u2588\u2580\u2580\u2588\u2591\u2588\u2580\u2588\u2580\u2588\u2591\u2588\u2580\u2580\u2580\u2591\u2591  <br> \u115a\u115a \u115a\u115a \u2591\u2591\u2588\u2591\u2588\u2591\u2588\u2591\u2588\u2580\u2580\u2580\u2591\u2588\u2591\u2591\u2591\u2591\u2588\u2591\u2591\u2591\u2591\u2588\u2591\u2591\u2588\u2591\u2588\u2591\u2588\u2591\u2588\u2591\u2588\u2580\u2580\u2580\u2591\u2591  <br> \u115a\u115a \u115a\u115a \u2591\u2591\u2580\u2580\u2580\u2580\u2580\u2591\u2580\u2580\u2580\u2580\u2591\u2580\u2580\u2580\u2580\u2591\u2580\u2580\u2580\u2580\u2591\u2580\u2580\u2580\u2580\u2591\u2580\u2591\u2580\u2591\u2580\u2591\u2580\u2580\u2580\u2580\u2591\u2591 <br> \u115a\u115a \u115a\u115a \u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591\u2591<\/div>  <br> \u115a\u115a\u115a\u115a \u115a\u115a\u115a\u115a\u115a\u2022 Quakelive Clan Arena server hosted in London \u2022<br>____________________________________________________________________________<br><br><br><div class=\"bb_h1\"><div class=\"bb_h1\">SERVER LINKS<\/div><\/div><br>\u25b8<a class=\"bb_link\" href=\"http:\/\/steamcommunity.com\/groups\/HardDonsQLS\/discussions\/1\/487876568231147255\/\" target=\"_blank\" rel=\"noreferrer\" >Server info<\/a><\/a> [maps, rules, cmds, etc]<br><br>\u25b8<a class=\"bb_link\" href=\"https:\/\/steamcommunity.com\/linkfilter\/?url=http:\/\/qlstats.net\/server\/1393\" target=\"_blank\" rel=\"noreferrer\" >Server Stats #1<\/a><span class=\"bb_link_host\">[qlstats.net]<\/span><\/a> <br><br>\u25b8<a class=\"bb_link\" href=\"https:\/\/steamcommunity.com\/linkfilter\/?url=https:\/\/www.dropbox.com\/s\/wjm0xfosp5i1x85\/HDQLS.url?dl=1\" target=\"_blank\" rel=\"noreferrer\" >Server Shortcut<\/a><span class=\"bb_link_host\">[www.dropbox.com]<\/span><\/a> <br><br>\u25b8<a class=\"bb_link\" href=\"http:\/\/steamcommunity.com\/groups\/HardDonsQLS\/discussions\/2\/487876568231146561\/\" target=\"_blank\" rel=\"noreferrer\" >Banned?<\/a><\/a> [post your alibi]<br><br>\u25b8<a class=\"bb_link\" href=\"http:\/\/steamcommunity.com\/groups\/HardDons\/\" target=\"_blank\" rel=\"noreferrer\" >HD Clan Page<\/a><\/a> [steam group]<br><br>\u25b8<a class=\"bb_link\" href=\"https:\/\/steamcommunity.com\/linkfilter\/?url=https:\/\/www.paypal.com\/cgi-bin\/webscr?cmd=_s-xclick&amp;hosted_button_id=RAXCUP8VJZUV2\" target=\"_blank\" rel=\"noreferrer\" >Donate<\/a><span class=\"bb_link_host\">[www.paypal.com]<\/span><\/a> <br>\u25b8<a class=\"bb_link\" href=\"https:\/\/steamcommunity.com\/linkfilter\/?url=https:\/\/www.paypal.me\/HDQLS\" target=\"_blank\" rel=\"noreferrer\" >Donate<\/a><span class=\"bb_link_host\">[www.paypal.me]<\/span><\/a> <br><br><br><br><br> \u115a\u115a    \u115a\u115a  \u115a\u115a     \u115a\u115a  <b> Join us on IRC!  #HD @ <a class=\"bb_link\" href=\"https:\/\/steamcommunity.com\/linkfilter\/?url=http:\/\/irc.harddons.net\" target=\"_blank\" rel=\"noreferrer\" >irc.harddons.net<\/a><br><br><br>GLHF<br><\/b>",
        "avatarIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/3d\/3deb81220282122967a4cd4e8680767876892a64.jpg",
        "avatarMedium": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/3d\/3deb81220282122967a4cd4e8680767876892a64_medium.jpg",
        "avatarFull": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/3d\/3deb81220282122967a4cd4e8680767876892a64_full.jpg",
        "memberCount": "105",
        "membersInChat": "0",
        "membersInGame": "3",
        "membersOnline": "15"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791429523819",
        "groupName": "NoFrag",
        "groupURL": "NoFrag",
        "headline": {

        },
        "summary": "No information given.",
        "avatarIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/a0\/a0fa3e7c16c367edcd25c0d1f2d2e8cd1277d7a6.jpg",
        "avatarMedium": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/a0\/a0fa3e7c16c367edcd25c0d1f2d2e8cd1277d7a6_medium.jpg",
        "avatarFull": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/a0\/a0fa3e7c16c367edcd25c0d1f2d2e8cd1277d7a6_full.jpg",
        "memberCount": "3512",
        "membersInChat": "0",
        "membersInGame": "111",
        "membersOnline": "617"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791429526566",
        "groupName": "id Software",
        "groupURL": "idsoftware",
        "headline": {

        },
        "summary": "No information given.",
        "avatarIcon": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/6a\/6a8769fd05c9a4ba581048bbdda3ae657caec6e2.jpg",
        "avatarMedium": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/6a\/6a8769fd05c9a4ba581048bbdda3ae657caec6e2_medium.jpg",
        "avatarFull": "http:\/\/cdn.edgecast.steamstatic.com\/steamcommunity\/public\/images\/avatars\/6a\/6a8769fd05c9a4ba581048bbdda3ae657caec6e2_full.jpg",
        "memberCount": "1909",
        "membersInChat": "1",
        "membersInGame": "147",
        "membersOnline": "475"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791433589142"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791433598625"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791434006608"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791437276364"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791454939833"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791455482966"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791455704092"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791456618843"
      },
      {
        "@attributes": {
          "isPrimary": "0"
        },
        "groupID64": "103582791460329041"
      }
    ]
  }
}
"#;
        let profile = serde_json::from_str::<super::SteamCoProfile>(string).unwrap();
        assert_eq!(
            profile.steam_id,
            crate::id::Id::Id64(crate::id::Id64(76561197992396121))
        );
    }
}
