use crate::{
    Result,
    types::{ SecretName, StoreDir },
};
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct CommonArgs {
    #[arg(long, env = "PASS_RS_STORE_DIR")]
    store_dir: Option<PathBuf>,

    #[arg(long, env = "PASS_RS_PUB_KEY_FILE")]
    pub pub_key_file: PathBuf,

    #[arg(long, env = "PASS_RS_PRIV_KEY_FILE")]
    pub priv_key_file: PathBuf,
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

    pub fn read_pub_key(&self) -> Result<Vec<u8>> {
        Ok(std::fs::read(&*self.pub_key_file)?)
    }

    pub fn read_priv_key(&self) -> Result<Vec<u8>> {
        Ok(std::fs::read(&*self.priv_key_file)?)
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
