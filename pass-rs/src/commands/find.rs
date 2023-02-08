use crate::{
    args::CommonArgs,
    Result,
    types::SecretName,
};

/// List the names of all secrets in the password store.
#[derive(clap::Args, Clone, Debug)]
pub struct Args {
    #[clap(flatten)]
    common: CommonArgs,
}

pub fn main(args: Args) -> Result<()> {
    let secrets: Vec<SecretName> = args.common.get_store_dir().find_secrets()?;
    for secret in secrets.iter() {
        println!("{}", secret.as_str())
    }

    Ok(())
}
