// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

/// A module that exposes errors thrown by the crate.
///
/// These errors are a chain generated from the the
/// [error-chain](https://crates.io/crates/error-chain) crate. This is so that
/// the underlying errors are not lost. See the
/// [ErrorKind](errors/enum.ErrorKind.html) enum for the various errors that can
/// be returned.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
            Yaml(serde_yaml::Error) #[doc = "Error when parsing a yaml file"];
        }
        errors {
            /// If a config file does not exist (at the given path) this error
            /// will be returned
            MissingConfigFile(f: String) {
                description("config file was not found at the specified path"),
                display("missing config file: '{}'", f),
            }
        }
    }
}

use std::fs::read_to_string;
use std::env;
use std::collections::HashMap;
use serde_yaml::Mapping;
use errors::*;

/// The main struct that holds the entire config map.
/// See the methods on this struct for ways to parse a config.
///
/// # Examples
///
/// ## Fetching current context
///
/// ```
/// use kube_conf::Config;
/// let config = Config::load("tests/config.yml")?;
/// let current_context = config.current_context.unwrap();
///
/// assert_eq!("default/dev-m01-example-com:8443/user", current_context);
/// # Ok::<(), kube_conf::errors::Error>(())
/// ```
///
/// ## Fetching the default kubeconfig file
///
/// This typically means the file located at `$HOME/.kube/config`
///
/// ```
/// use kube_conf::Config;
/// # use std::env::{set_var, current_dir};
/// # set_var("HOME", format!("{}/tests", current_dir().unwrap().to_str().unwrap()));
/// let config = Config::load_default()?;
/// let current_context = config.current_context.unwrap();
///
/// assert_eq!("default/dev-m01-example-com:8443/user", current_context);
/// # Ok::<(), kube_conf::errors::Error>(())
/// ```
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Will typically be "v1"
    #[serde(rename="apiVersion")]
    pub api_version: Option<String>,

    /// Will typically be "Config"
    pub kind: Option<String>,

    #[serde(rename="current-context")]
    pub current_context: Option<String>,

    pub preferences: Option<Mapping>,

    //pub clusters: Vec<Cluster>,

    //pub contexts: Vec<Context>,

    //pub users: Vec<User>,
}

impl Config {
    /// Fetches the current config based on the user's configured environment.
    ///
    /// This includes `$KUBECONFIG` when set, or simply `$HOME/.kube/config`
    /// otherwise.
    ///
    /// TODO: Support multiple kubeconfig files in the KUBECONFIG env var
    /// separated by colons, i.e. `KUBECONFIG=file1:file2`.
    pub fn load_default() -> Result<Config> {
        if let Ok(conf_path) = env::var("KUBECONFIG") {
            return Config::load(&conf_path);
        }

        if let Ok(home) = env::var("HOME") {
            return Config::load(&format!("{}/.kube/config", &home));
        }

        bail!("Neither $KUBECONFIG nor $HOME are defined");
    }

    pub fn load(path: &str) -> Result<Config> {
        let conf = read_to_string(&path)
            .chain_err(|| ErrorKind::MissingConfigFile(path.to_string()))?;
        let conf = serde_yaml::from_str(&conf)?;
        Ok(conf)
    }
}

