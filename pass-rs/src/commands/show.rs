use anyhow::Context;
use crate::{
    args::CommonArgs,
    Result,
    types::SecretName,
};

/// Show a secret in the password store
#[derive(clap::Args, Debug)]
pub struct Args {
    #[clap(flatten)]
    common: CommonArgs,

    /// Name of the secret to show
    #[arg(value_parser(crate::args::SecretNameParser))]
    secret_name: SecretName,
}

pub fn main(args: Args) -> Result<()> {
    let mut pgp_wrapper_context =  args.common.new_pgp_wrapper_context()?;
    let enc_path = args.common.get_store_dir().secret_file_path(&args.secret_name);
    let enc = std::fs::read(&enc_path)
        .with_context(|| format!("Failed to read encrypted secret from '{enc_path:?}'"))?;
    let priv_key = args.common.read_priv_key()?;
    let passphrase = crate::utils::read_passphrase(&*args.common.priv_key_file)?;
    let out: Vec<u8> = pgp_wrapper_context.bindings.pgp_wrapper_exports()
                           .decrypt(
                               &mut pgp_wrapper_context.store,
                               &*enc,
                               &*priv_key,
                               &*passphrase)?
                           .map_err(|e| anyhow::Error::msg(e))?;
    let out_str = std::str::from_utf8(&*out);
    match out_str {
        Ok(v) => {
            print!("{v}");
            if !v.ends_with("\n") && !v.ends_with("\r\n") {
                println!("");
            }
        }
        Err(_e) => {
            eprintln!("Secret data is not valid UTF8, printing byte array");
            println!("{out:?}");
        }
    }

    Ok(())
}
