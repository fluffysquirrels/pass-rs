use anyhow::Context;
use crate::{
    args::CommonArgs,
    Result,
    types::SecretName,
};

/// Insert a secret into the password store
#[derive(clap::Args, Clone, Debug)]
pub struct Args {
    #[clap(flatten)]
    common: CommonArgs,

    /// Name of the secret to insert
    #[arg(value_parser(crate::args::SecretNameParser))]
    secret_name: SecretName,

    /// Replace an existing secret. By default this will fail with an error.
    #[arg(default_value_t = false, long)]
    replace: bool,
}

pub fn main(args: Args) -> Result<()> {
    let mut pgp_wrapper_context =  args.common.new_pgp_wrapper_context()?;
    let pub_key = args.common.read_pub_key()?;
    let secret_plaintext = rpassword::prompt_password("Enter your secret: ")
        .with_context(|| "While reading your secret")?;
    let out: Vec<u8> = pgp_wrapper_context.bindings.pgp_wrapper_exports()
                           .encrypt(
                               &mut pgp_wrapper_context.store,
                               secret_plaintext.as_bytes(),
                               &*pub_key)?
                           .map_err(|e| anyhow::Error::msg(e))?;
    let out_path = args.common.get_store_dir().secret_file_path(&args.secret_name);
    crate::utils::write_secret(&out_path, &*out, args.replace)?;

    Ok(())
}
