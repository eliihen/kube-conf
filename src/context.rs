use crate::get::{get_mapping, get_string};
use serde::{Deserialize, Deserializer};
use serde_yaml::{Mapping};

/// TODO
#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub cluster: String,
    pub namespace: Option<String>,
    pub user: String,
}

/*
impl<'de> TryFrom<Mapping> for Context {
    type Error = de::Error + 'de;
    fn try_from(map: Mapping) -> Result<Self, Self::Error> {
        // let cluster = get_map(map, "cluster");
        Ok(Context {
            name: get_string(map, "name")?,
            certificate_authority: Some(String::new()),
            server: Some(String::new()),
        })
    }
}
*/

impl<'de> Deserialize<'de> for Context {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: Mapping = Deserialize::deserialize(d)?;
        let name = get_string(&map, "name")?;
        let context = get_mapping(map, "context")?;

        Ok(Context {
            name: name,
            cluster: get_string::<D::Error>(&context, "cluster")?,
            user: get_string::<D::Error>(&context, "user")?,
            namespace: get_string::<D::Error>(&context, "namespace").ok(),
        })

        // Context::try_from(map)
    }
}
