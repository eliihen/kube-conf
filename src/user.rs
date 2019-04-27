//! The module holding the `User` struct

use crate::get::{get_mapping, get_string};
use serde::{Deserialize, Deserializer};
use serde_yaml::Mapping;
use std::path::PathBuf;

/// A user represents a user that can be used to log in to one of the clusters
/// given in the `Cluster` struct. The mapping of which user can log in to which
/// clusters are maintained in the `Context` set found in the `Config` struct.
///
/// Note: The user struct is flattened when compared to its representation in
/// the yaml file. There is no `user` mapping, the values of the `user`
/// mapping are directly accessible on the `User` struct.
#[derive(Debug, Clone)]
pub struct User {
    /// The name given to this user by the user
    pub name: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,

    /// A `PathBuf` representing the client certificate associated with this
    /// user. This is a path to a file on the disk.
    pub client_certificate: Option<PathBuf>,

    /// A string representing the client certificate associated with this
    /// user. This is a base64 encoded string containing the CA data.
    pub client_certificate_data: Option<String>,

    /// A `PathBuf` representing the client key associated with this
    /// user. This is a path to a file on the disk.
    pub client_key: Option<PathBuf>,

    /// A string representing the client key associated with this
    /// user. This is a base64 encoded string containing the CA data.
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
            name,
            token: get_string::<D::Error>(&user, "token").ok(),
            username: get_string::<D::Error>(&user, "username").ok(),
            password: get_string::<D::Error>(&user, "password").ok(),
            client_certificate: get_string::<D::Error>(&user, "client-certificate")
                .map(PathBuf::from)
                .ok(),
            client_certificate_data: get_string::<D::Error>(&user, "client-certificate-data").ok(),
            client_key: get_string::<D::Error>(&user, "client-key")
                .map(PathBuf::from)
                .ok(),
            client_key_data: get_string::<D::Error>(&user, "client-key-data").ok(),
        })

        // User::try_from(map)
    }
}
