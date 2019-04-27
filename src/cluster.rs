//! The module holding the `Cluster` struct

use crate::get::{get_bool, get_mapping, get_string};
use serde::{Deserialize, Deserializer};
use serde_yaml::Mapping;
use std::path::PathBuf;

/// A cluster represents a cluster that the user knows how to connect to.
///
/// Note: The cluster struct is flattened when compared to its representation in
/// the yaml file. There is no `cluster` mapping, the values of the `cluster`
/// mapping are directly accessible on the `Cluster` struct.
#[derive(Debug, Clone)]
pub struct Cluster {
    /// The name given to the cluster by the user
    pub name: String,

    /// The HTTP address to the server, including protocol
    pub server: String,

    /// A `PathBuf` representing the certificate authority associated with this
    /// cluster. This is a path to a file on the disk.
    pub certificate_authority: Option<PathBuf>,

    /// A string representing the certificate authority associated with this
    /// cluster. This is a base64 encoded string containing the CA data.
    pub certificate_authority_data: Option<String>,

    /// When set to true this is a signal that any certificate checking should
    /// be bypassed by the user agent.
    pub insecure_skip_tls_verify: bool,
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
            name,
            server: get_string::<D::Error>(&cluster, "server")?,
            certificate_authority: get_string::<D::Error>(&cluster, "certificate-authority")
                .map(PathBuf::from)
                .ok(),
            certificate_authority_data: get_string::<D::Error>(
                &cluster,
                "certificate-authority-data",
            )
            .ok(),
            insecure_skip_tls_verify: get_bool::<D::Error>(&cluster, "insecure-skip-tls-verify")
                .unwrap_or_default(),
        })

        // Cluster::try_from(map)
    }
}
