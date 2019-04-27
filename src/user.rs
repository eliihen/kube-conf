use crate::get::{get_bool, get_mapping, get_string};
use serde::{de, Deserialize, Deserializer};
use serde_yaml::{Error as YamlError, Mapping};
use std::convert::TryFrom;

/// TODO
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_certificate: Option<String>,
    pub client_certificate_data: Option<String>,
    pub client_key: Option<String>,
    pub client_key_data: Option<String>,
}

/*
impl<'de> TryFrom<Mapping> for User {
    type Error = de::Error + 'de;
    fn try_from(map: Mapping) -> Result<Self, Self::Error> {
        // let user = get_map(map, "user");
        Ok(User {
            name: get_string(map, "name")?,
            certificate_authority: Some(String::new()),
            server: Some(String::new()),
        })
    }
}
*/

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: Mapping = Deserialize::deserialize(d)?;
        let name = get_string(&map, "name")?;
        let user = get_mapping(map, "user")?;

        Ok(User {
            name: name,
            token: get_string::<D::Error>(&user, "token").ok(),
            username: get_string::<D::Error>(&user, "username").ok(),
            password: get_string::<D::Error>(&user, "password").ok(),
            client_certificate: get_string::<D::Error>(&user, "client-certificate").ok(),
            client_certificate_data: get_string::<D::Error>(&user, "client-certificate-data").ok(),
            client_key: get_string::<D::Error>(&user, "client-key").ok(),
            client_key_data: get_string::<D::Error>(&user, "client-key-data").ok(),
        })

        // User::try_from(map)
    }
}
