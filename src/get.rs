use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

pub fn get_string<E>(map: &Mapping, key: &str) -> Result<String, E>
where
    E: Error,
{
    let key_val = Value::String(String::from(key));

    map.get(&key_val)
        .ok_or_else(|| Error::missing_field("Field was not found"))?
        .as_str()
        .map(String::from)
        .ok_or_else(|| E::missing_field("Field could not be parsed as string"))
}

pub fn get_bool<E>(map: &Mapping, key: &str) -> Result<bool, E>
where
    E: Error,
{
    let key_val = Value::String(String::from(key));

    map.get(&key_val)
        .ok_or_else(|| Error::missing_field("Field was not found"))?
        .as_bool()
        .ok_or_else(|| E::missing_field("Field could not be parsed as bool"))
}

pub fn get_mapping<E>(map: Mapping, key: &str) -> Result<Mapping, E>
where
    E: Error,
{
    let key_val = Value::String(String::from(key));

    map.get(&key_val)
        .ok_or_else(|| Error::missing_field("Field was not found"))?
        .as_mapping()
        // Clone to avoid returning reference to discarded local variable
        .cloned()
        .ok_or_else(|| E::missing_field("Field could not be parsed as mapping"))
}
