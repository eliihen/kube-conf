//! The module holding the `Context` struct

use crate::get::{get_mapping, get_string};
use crate::{Cluster, Config, User};
use serde::{Deserialize, Deserializer};
use serde_yaml::Mapping;

/// A context represents a mapping between known users in the `users` set and
/// servers in the `clusters` set. By looking them up here
///
/// Note: The context struct is flattened when compared to its representation in
/// the yaml file. There is no `context` mapping, the values of the `context`
/// mapping are directly accessible on the `Context` struct.
#[derive(Debug, Clone)]
pub struct Context {
    /// The name given to this context by the user
    pub name: String,

    /// The cluster `name` this context refers to
    pub cluster: String,

    /// The default namespace to use with this context
    pub namespace: Option<String>,

    /// The user `name` this cluster refers to
    pub user: String,
}

impl Context {
    /// Gets the `Cluster` object associated with this context if it exists in
    /// the provided config object.
    pub fn get_cluster(&self, config: &Config) -> Option<Cluster> {
        for cluster in config.clusters.iter() {
            if &cluster.name == &self.cluster {
                return Some(cluster.clone());
            }
        }

        None
    }

    /// Gets the `User` object associated with this context if it exists in the
    /// provided config object.
    pub fn get_user(&self, config: &Config) -> Option<User> {
        for user in config.users.iter() {
            if user.name == self.user {
                return Some(user.clone());
            }
        }

        None
    }
}

impl<'de> Deserialize<'de> for Context {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: Mapping = Deserialize::deserialize(d)?;
        let name = get_string(&map, "name")?;
        let context = get_mapping(map, "context")?;

        Ok(Context {
            name,
            cluster: get_string::<D::Error>(&context, "cluster")?,
            user: get_string::<D::Error>(&context, "user")?,
            namespace: get_string::<D::Error>(&context, "namespace").ok(),
        })

        // Context::try_from(map)
    }
}

/*
TODO Write a working TryFrom impl

impl TryFrom<Mapping> for Context {
    type Error = de::Error;
    fn try_from(map: Mapping) -> Result<Self, Self::Error> {
        let name = get_string(&map, "name")?;
        let context = get_mapping(map, "context")?;

        Ok(Context {
            name,
            cluster: get_string::<D::Error>(&context, "cluster")?,
            user: get_string::<D::Error>(&context, "user")?,
            namespace: get_string::<D::Error>(&context, "namespace").ok(),
        })
    }
}
*/
