//! Welcome to the `kube-conf` crate.
//!
//! This crate is a convenient way of fetching the local user's kubernetes
//! config file and reading the values.
//!
//! # Examples
//!
//! ## Fetching current context
//!
//! ```
//! use kube_conf::Config;
//! let config = Config::load("tests/config.yml")?;
//! let current_context = config.get_current_context().unwrap();
//!
//! assert_eq!("dev-frontend", current_context.name);
//! # Ok::<(), kube_conf::errors::Error>(())
//! ```
//!
//! ## Fetching the default kubeconfig file
//!
//! This typically means the file located at `$HOME/.kube/config`
//!
//! ```
//! use kube_conf::Config;
//! # use std::env::{set_var, current_dir};
//! # set_var("HOME", format!("{}/tests", current_dir().unwrap().to_str().unwrap()));
//! let config = Config::load_default()?;
//! let current_context = config.current_context.unwrap();
//!
//! assert_eq!("dev-frontend", current_context);
//! # Ok::<(), kube_conf::errors::Error>(())
//! ```

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod cluster;
pub mod context;
mod get;
pub mod user;

/// A module that exposes errors thrown by the crate.
///
/// These errors are a chain generated from the the
/// [error-chain](https://crates.io/crates/error-chain) crate. This is so that
/// the underlying errors are not lost. See the
/// [ErrorKind](errors/enum.ErrorKind.html) enum for the various errors that can
/// be returned.
#[allow(deprecated)]
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
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

use cluster::Cluster;
use context::Context;
use errors::*;
use serde_yaml::Mapping;
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use user::User;

/// The main struct that holds the entire config map.
/// See the methods on this struct for ways to parse a config.
#[serde(rename_all = "kebab-case")]
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The name of the current active context.
    /// The actual context can be retrieved by finding the context in the
    /// context set based on this name.
    pub current_context: Option<String>,

    /// Preferences provided in the config.yml file.
    pub preferences: Option<Mapping>,

    /// The clusters as defined by the "clusters" key
    pub clusters: Vec<Cluster>,

    /// The contexts as defined by the "contexts" key
    pub contexts: Vec<Context>,

    /// The users as defined by the "users" key
    pub users: Vec<User>,

    /// Will typically be "v1", generally not needed
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,

    /// Will typically be "Config", generally not needed
    pub kind: Option<String>,
}

impl Config {
    /// Fetches the current config based on the user's configured environment.
    ///
    /// This includes `$KUBECONFIG` when set, or simply `$HOME/.kube/config`
    /// otherwise.
    ///
    /// TODO: Support multiple kubeconfig files in the KUBECONFIG env var
    /// separated by colons, i.e. `KUBECONFIG=file1:file2`. Merge the result.
    pub fn load_default() -> Result<Config> {
        if let Ok(conf_path) = env::var("KUBECONFIG") {
            return Config::load(&conf_path);
        }

        if let Ok(home) = env::var("HOME") {
            return Config::load(&format!("{}/.kube/config", &home));
        }

        bail!("Neither $KUBECONFIG nor $HOME are defined");
    }

    /// Fetches the config from the provided path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
        let conf = read_to_string(&path)
            .chain_err(|| ErrorKind::MissingConfigFile(format!("{}", path.as_ref().display())))?;
        let conf = serde_yaml::from_str(&conf)?;
        Ok(conf)
    }

    /// Gets the currently active context based on the `current-context` key in
    /// the config file.
    pub fn get_current_context(&self) -> Option<&Context> {
        if let Some(current_context) = &self.current_context {
            for context in self.contexts.iter() {
                if &context.name == current_context {
                    return Some(context);
                }
            }
        }

        None
    }
}
