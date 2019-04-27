use crate::get::{get_bool, get_mapping, get_string};
use serde::{de, Deserialize, Deserializer};
use serde_yaml::{Error as YamlError, Mapping};
use std::convert::TryFrom;

/// A cluster that
#[derive(Debug)]
pub struct Cluster {
    pub name: String,
    pub server: String,
    pub certificate_authority: Option<String>,
    pub certificate_authority_data: Option<String>,
    pub insecure_skip_tls_verify: Option<bool>,
}

/*
impl<'de> TryFrom<Mapping> for Cluster {
    type Error = de::Error + 'de;
    fn try_from(map: Mapping) -> Result<Self, Self::Error> {
        // let cluster = get_map(map, "cluster");
        Ok(Cluster {
            name: get_string(map, "name")?,
            certificate_authority: Some(String::new()),
            server: Some(String::new()),
        })
    }
}
*/

impl<'de> Deserialize<'de> for Cluster {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: Mapping = Deserialize::deserialize(d)?;
        let name = get_string(&map, "name")?;
        let cluster = get_mapping(map, "cluster")?;

        Ok(Cluster {
            name: name,
            certificate_authority: get_string::<D::Error>(&cluster, "certificate-authority").ok(),
            certificate_authority_data: get_string::<D::Error>(
                &cluster,
                "certificate-authority-data",
            )
            .ok(),
            insecure_skip_tls_verify: get_bool::<D::Error>(&cluster, "insecure-skip-tls-verify")
                .ok(),
            server: get_string::<D::Error>(&cluster, "server")?,
        })

        // Cluster::try_from(map)
    }
}
