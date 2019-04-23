use kube_conf::Config;
use kube_conf::errors::*;

#[test]
pub fn it_loads_using_default() -> Result<()> {
    std::env::set_var("HOME", format!("{}/tests", env!("CARGO_MANIFEST_DIR")));
    Config::load_default()?;
    Ok(())
}

#[test]
pub fn it_loads_using_kubeconfig_env_var() -> Result<()> {
    println!("{}", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("KUBECONFIG", format!("{}/tests/config.yml", env!("CARGO_MANIFEST_DIR")));
    Config::load_default()?;
    Ok(())
}
