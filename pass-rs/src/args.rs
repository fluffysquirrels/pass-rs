use anyhow::Context;
use crate::{
    Result,
    types::{ SecretName, StoreDir },
};
use std::ffi::OsStr;
use std::path::{ Path, PathBuf };

#[derive(clap::Args, Clone, Debug)]
pub struct CommonArgs {
    /// Root directory of the password store.
    ///
    /// If this argument is missing read from the environment variable
    /// `PASS_RS_STORE_DIR`, and if that is missing the default value
    /// "$HOME/.password-store" will be used.
    #[arg(long, env = "PASS_RS_STORE_DIR")]
    store_dir: Option<PathBuf>,

    /// File to read as a public key for encryption operations.
    ///
    /// If this argument is missing try to read a value from the
    /// environment variable `PASS_RS_PUB_KEY_FILE`.
    #[arg(long, env = "PASS_RS_PUB_KEY_FILE")]
    pub_key_file: Option<PathBuf>,

    /// File to read as a private key for encryption operations.
    ///
    /// If this argument is missing try to read a value from the
    /// environment variable `PASS_RS_PRIV_KEY_FILE`.
    #[arg(long, env = "PASS_RS_PRIV_KEY_FILE")]
    priv_key_file: Option<PathBuf>,
}

impl CommonArgs {
    pub fn get_store_dir(&self) -> StoreDir {
        let path = self.store_dir.clone()
                       .unwrap_or_else(|| std::env::home_dir()
                                              .expect("env::home_dir()")
                                              .join(".password-store")
                                              .to_path_buf());
        StoreDir::new(path)
    }

    pub fn new_pgp_wrapper_context(&self) -> Result<crate::pgp_wrapper::Context> {
        crate::pgp_wrapper::Context::new()
    }

    pub fn pub_key_path(&self) -> Result<&Path> {
        let buf =
            self.pub_key_file
                .as_ref()
                .ok_or_else(|| anyhow::Error::msg("You must specify a public key file"))?;
        Ok(&*buf)
    }

    pub fn priv_key_path(&self) -> Result<&Path> {
        let buf =
            self.priv_key_file
                .as_ref()
                .ok_or_else(|| anyhow::Error::msg("You must specify a private key file"))?;
        Ok(&*buf)
    }

    pub fn read_pub_key(&self) -> Result<Vec<u8>> {
        let path = self.pub_key_path()?;
        Ok(std::fs::read(path)
           .with_context(|| format!("Reading public key from '{path:?}'"))?)
    }

    pub fn read_priv_key(&self) -> Result<Vec<u8>> {
        let path = self.priv_key_path()?;
        Ok(std::fs::read(path)
           .with_context(|| format!("Reading private key from '{path:?}'"))?)
    }
}

#[derive(Clone)]
pub struct SecretNameParser;

impl clap::builder::TypedValueParser for SecretNameParser {
    type Value = SecretName;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr
    ) -> std::result::Result<Self::Value, clap::Error> {
        SecretName::try_from(value)
            .map_err(|e| clap::Error::raw(clap::error::ErrorKind::InvalidValue, e.to_string()))
    }
}
