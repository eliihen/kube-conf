use kube_conf::errors::*;
use kube_conf::Config;
use serde_yaml::Value;

#[test]
pub fn it_loads_using_default() -> Result<()> {
    std::env::set_var("HOME", format!("{}/tests", env!("CARGO_MANIFEST_DIR")));
    Config::load_default()?;

    Ok(())
}

#[test]
pub fn it_loads_using_kubeconfig_env_var() -> Result<()> {
    std::env::set_var(
        "KUBECONFIG",
        format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR")),
    );
    Config::load_default()?;

    Ok(())
}

#[test]
pub fn it_returns_error_when_no_file_is_found() {
    let path = "/path/that/does/not/exist";
    let conf_result = Config::load(&path);
    assert!(
        conf_result.is_err(),
        "Expected return to be an error, but was success"
    );
    assert_eq!(
        conf_result.unwrap_err().to_string(),
        format!("missing config file: '{}'", &path)
    );

    std::env::set_var("KUBECONFIG", path);
    let conf_result = Config::load_default();
    assert!(
        conf_result.is_err(),
        "Expected return to be an error, but was success"
    );
    assert_eq!(
        conf_result.unwrap_err().to_string(),
        format!("missing config file: '{}'", &path)
    );
}

#[test]
pub fn it_returns_the_preferences_map() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;
    assert!(config.preferences.is_some(), "Preferences were not defined");

    let preferences = config.preferences.unwrap();
    let value = preferences.get(&Value::String("fooValue".to_string()));
    assert!(value.is_some(), "fooValue not found in preferences mapping");
    assert_eq!(
        value.unwrap().as_str().unwrap(),
        "abc",
        "Preferences were not defined"
    );

    Ok(())
}

#[test]
pub fn it_returns_the_cluster_set() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;
    assert_eq!(
        config.clusters.len(),
        2,
        "Expected 2 clusters in the cluster set"
    );

    let cluster1 = config.clusters.get(0).unwrap();
    println!("{:#?}", cluster1);
    assert!(
        cluster1.certificate_authority.is_some(),
        "Expected the first cluster to have a CA configured"
    );
    assert_eq!(
        cluster1.certificate_authority.as_ref().unwrap(),
        "fake-ca-file",
        "Expected the first cluster to have a CA configured"
    );
    assert_eq!(
        cluster1.server, "https://1.2.3.4",
        "Expected the first cluster to have a server configured"
    );

    let cluster2 = config.clusters.get(1).unwrap();
    assert!(
        cluster2.insecure_skip_tls_verify.is_some(),
        "Expected the second cluster to have a skip_tls flag set"
    );
    assert_eq!(
        cluster2.insecure_skip_tls_verify.unwrap(),
        true,
        "Expected the first second to have a skip_tls flag set"
    );
    assert_eq!(
        cluster2.server, "https://5.6.7.8",
        "Expected the second cluster to have a server configured"
    );

    Ok(())
}

#[test]
pub fn it_returns_the_context_set() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;
    assert_eq!(
        config.contexts.len(),
        3,
        "Expected 3 contexts in the cluster set"
    );

    let context1 = config.contexts.get(0).unwrap();
    assert_eq!(
        context1.cluster, "development",
        "Expected the first context to have the correct cluster ID configured"
    );
    assert_eq!(
        context1.user, "developer",
        "Expected the first context to have the correct user configured"
    );
    assert!(
        context1.namespace.is_some(),
        "Expected the first context to have a namespace set"
    );
    assert_eq!(
        context1.namespace.as_ref().unwrap(),
        "frontend",
        "Expected the first context to have the correct namespace set"
    );

    Ok(())
}

#[test]
pub fn it_returns_the_users_set() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;
    assert_eq!(config.users.len(), 2, "Expected 2 users in the cluster set");

    let user1 = config.users.get(0).unwrap();
    assert!(
        user1.client_certificate.is_some(),
        "Expected the first user to have a client-certificate set"
    );
    assert_eq!(
        user1.client_certificate.as_ref().unwrap(),
        "fake-cert-file",
        "Expected the first user to have the correct client-certificate configured"
    );

    assert!(
        user1.client_key.is_some(),
        "Expected the first user to have a client-key set"
    );
    assert_eq!(
        user1.client_key.as_ref().unwrap(),
        "fake-key-file",
        "Expected the first user to have the correct client-key configured"
    );

    let user2 = config.users.get(1).unwrap();
    assert!(
        user2.password.is_some(),
        "Expected the second user to have a password set"
    );
    assert_eq!(
        user2.password.as_ref().unwrap(),
        "some-password",
        "Expected the second user to have the correct password set"
    );

    assert!(
        user2.username.is_some(),
        "Expected the second user to have a password set"
    );
    assert_eq!(
        user2.username.as_ref().unwrap(),
        "exp",
        "Expected the second user to have the correct password set"
    );

    Ok(())
}
