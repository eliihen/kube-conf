use kube_conf::Config;
use kube_conf::errors::*;
use serde_yaml::Value;

#[test]
pub fn it_loads_using_default() -> Result<()> {
    std::env::set_var("HOME", format!("{}/tests", env!("CARGO_MANIFEST_DIR")));
    Config::load_default()?;

    Ok(())
}

#[test]
pub fn it_loads_using_kubeconfig_env_var() -> Result<()> {
    std::env::set_var("KUBECONFIG", format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR")));
    Config::load_default()?;

    Ok(())
}

#[test]
pub fn it_returns_error_when_no_file_is_found() {
    let path = "/path/that/does/not/exist";
    let conf_result = Config::load(&path);
    assert!(conf_result.is_err(), "Expected return to be an error, but was success");
    assert_eq!(conf_result.unwrap_err().to_string(), format!("missing config file: '{}'", &path));

    std::env::set_var("KUBECONFIG", path);
    let conf_result = Config::load_default();
    assert!(conf_result.is_err(), "Expected return to be an error, but was success");
    assert_eq!(conf_result.unwrap_err().to_string(), format!("missing config file: '{}'", &path));
}

#[test]
pub fn it_returns_the_preferences_map() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;
    assert!(config.preferences.is_some(), "Preferences were not defined");

    let preferences = config.preferences.unwrap();
    let value = preferences.get(&Value::String("fooValue".to_string()));
    assert!(value.is_some(), "fooValue not found in preferences mapping");
    assert_eq!(value.unwrap().as_str().unwrap(), "abc", "Preferences were not defined");

    Ok(())
}

