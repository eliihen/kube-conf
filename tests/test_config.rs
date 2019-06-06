use kube_conf::errors::*;
use kube_conf::Config;
use serde_yaml::Value;
use std::path::PathBuf;

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
        &PathBuf::from("fake-ca-file"),
        "Expected the first cluster to have a CA configured"
    );
    assert_eq!(
        cluster1.server, "https://1.2.3.4",
        "Expected the first cluster to have a server configured"
    );
    assert_eq!(
        cluster1.insecure_skip_tls_verify, false,
        "Expected the first cluster to have the default value of skip_tls (false)"
    );

    let cluster2 = config.clusters.get(1).unwrap();
    assert_eq!(
        cluster2.insecure_skip_tls_verify, true,
        "Expected the second cluster to have a skip_tls flag set"
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
        &PathBuf::from("fake-cert-file"),
        "Expected the first user to have the correct client-certificate configured"
    );

    assert!(
        user1.client_key.is_some(),
        "Expected the first user to have a client-key set"
    );
    assert_eq!(
        user1.client_key.as_ref().unwrap(),
        &PathBuf::from("fake-key-file"),
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

#[test]
pub fn it_gets_the_current_context() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;

    let context = config.get_current_context();
    assert!(context.is_some(), "Current context not found");
    assert_eq!(
        context.unwrap().name,
        "dev-frontend",
        "Current context did not have the expected name"
    );
    assert_eq!(
        context.unwrap().cluster,
        "development",
        "Current context did not have the expected cluster"
    );
    assert_eq!(
        context.unwrap().user,
        "developer",
        "Current context did not have the expected user"
    );

    Ok(())
}

#[test]
pub fn it_gets_the_cluster_from_a_context() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;

    let context = config.get_current_context().unwrap();
    let cluster = context.get_cluster(&config);

    assert!(cluster.is_some(), "Cluster of context not found");
    let cluster = cluster.unwrap();

    assert_eq!(
        cluster.name, "development",
        "Cluster of context did not have the expected name"
    );
    assert!(
        cluster.certificate_authority.is_some(),
        "Expected the cluster of context to have a certificate-authority set"
    );
    assert_eq!(
        cluster.certificate_authority.as_ref().unwrap(),
        &PathBuf::from("fake-ca-file"),
        "Cluster of context did not have the expected certificate-authority"
    );
    assert_eq!(
        cluster.server, "https://1.2.3.4",
        "Cluster of context did not have the expected server"
    );

    Ok(())
}

#[test]
pub fn it_gets_the_user_from_a_context() -> Result<()> {
    let path = format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&path)?;

    let context = config.get_current_context().unwrap();
    let user = context.get_user(&config);

    assert!(user.is_some(), "User of context not found");
    let user = user.unwrap();

    assert_eq!(
        user.name, "developer",
        "User of context did not have the expected name"
    );
    assert!(
        user.client_certificate.is_some(),
        "Expected the User of context to have a client-certificate set"
    );
    assert_eq!(
        user.client_certificate.as_ref().unwrap(),
        &PathBuf::from("fake-cert-file"),
        "User of context did not have the expected client-certificate"
    );
    assert!(
        user.client_key.is_some(),
        "Expected the User of context to have a client-key set"
    );
    assert_eq!(
        user.client_key.as_ref().unwrap(),
        &PathBuf::from("fake-key-file"),
        "User of context did not have the expected client-key"
    );

    Ok(())
}
