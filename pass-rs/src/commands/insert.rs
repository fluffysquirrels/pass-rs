use anyhow::Context;
use crate::{
    args::CommonArgs,
    Result,
    types::SecretName,
};
use std::io::Read;

/// Insert a secret into the password store
#[derive(clap::Args, Clone, Debug)]
pub struct Args {
    #[clap(flatten)]
    common: CommonArgs,

    /// Name of the secret to insert
    #[arg(value_parser(crate::args::SecretNameParser))]
    secret_name: SecretName,

    /// Replace an existing secret. By default this will fail with an error.
    #[arg(long, default_value_t = false)]
    replace: bool,

    /// Read the secret value from stdin rather than a password prompt.
    #[arg(long, default_value_t = false)]
    from_stdin: bool,
}

pub fn main(args: Args) -> Result<()> {
    let mut pgp_wrapper_context =  args.common.new_pgp_wrapper_context()?;
    let pub_key = args.common.read_pub_key()?;

    let secret_plaintext = if args.from_stdin {
        let mut  = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        s
    } else {
        rpassword::prompt_password("Enter your secret: ")
                  .with_context(|| "While reading your secret")?
    };

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
