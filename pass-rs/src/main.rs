mod pgp_wrapper;

use anyhow::{ anyhow, Context };
use clap::Parser;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::path::{ Path, PathBuf };
use std::str::FromStr;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(clap::Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Show(ShowArgs),
    Insert(InsertArgs),
}

#[derive(clap::Args, Debug)]
struct ShowArgs {
    #[clap(flatten)]
    common: CommonArgs,

    #[arg(value_parser(SecretNameParser))]
    secret_name: SecretName,
}

#[derive(clap::Args, Debug)]
struct InsertArgs {
    #[clap(flatten)]
    common: CommonArgs,

    #[arg(value_parser(SecretNameParser))]
    secret_name: SecretName,
}

#[derive(clap::Args, Debug)]
struct CommonArgs {
    #[arg(long, env = "PASS_RS_STORE_DIR")]
    store_dir: Option<PathBuf>,

    #[arg(long, env = "PASS_RS_PUB_KEY_FILE")]
    pub_key_file: PathBuf,

    #[arg(long, env = "PASS_RS_PRIV_KEY_FILE")]
    priv_key_file: PathBuf,
}

#[derive(Clone, Debug)]
struct SecretName {
    name: String,
}

impl std::string::ToString for SecretName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl TryFrom<&OsStr> for SecretName {
    type Error = clap::Error;

    fn try_from(s: &OsStr) -> std::result::Result<Self, Self::Error> {
        let str = s.to_str()
                   .ok_or(clap::Error::raw(clap::error::ErrorKind::InvalidUtf8,
                                           "SecretName must be valid UTF8".to_string()))?;
        let path = PathBuf::from_str(str).expect("infallible");
        if !path.is_relative() {
            return Err(clap::Error::raw(clap::error::ErrorKind::InvalidValue,
                                        "SecretName must be a relative path".to_string()));
        }
        for comp in path.components() {
            let comp_str = comp.as_os_str().to_str().expect("already validated UTF8");
            let comp_valid = comp_str.chars().all(valid_secret_name_char);
            if !comp_valid {
                return Err(clap::Error::raw(clap::error::ErrorKind::InvalidValue,
                                            "SecretName must be a relative path where each component contains only alphanumeric characters, hypens, and underscores".to_string()));
            }
        }

        Ok(SecretName {
            name: str.to_string(),
        })
    }
}

fn valid_secret_name_char(ch: char) -> bool {
    ch.is_alphanumeric()
        || ch == '-'
        || ch == '_'
}

#[derive(Clone)]
struct SecretNameParser;

impl clap::builder::TypedValueParser for SecretNameParser {
    type Value = SecretName;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr
    ) -> std::result::Result<Self::Value, clap::Error> {
        SecretName::try_from(value)
    }
}

impl CommonArgs {
    fn get_store_dir(&self) -> StoreDir {
        let path = self.store_dir.clone()
                       .unwrap_or_else(|| std::env::home_dir()
                                              .expect("env::home_dir()")
                                              .join(".password-store")
                                              .to_path_buf());
        StoreDir::new(path)
    }

    fn new_pgp_wrapper_context(&self) -> Result<pgp_wrapper::Context> {
        pgp_wrapper::Context::new()
    }

    fn read_pub_key(&self) -> Result<Vec<u8>> {
        Ok(std::fs::read(&*self.pub_key_file)?)
    }

    fn read_priv_key(&self) -> Result<Vec<u8>> {
        Ok(std::fs::read(&*self.priv_key_file)?)
    }
}

struct StoreDir {
    path: PathBuf,
}

impl StoreDir {
    fn new(path: PathBuf) -> Self {
        StoreDir {
            path: path,
        }
    }

    fn secret_file_path(&self, secret_name: &SecretName) -> PathBuf {
        self.path.join(secret_name.to_string())
                 .with_extension("gpg")
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    eprintln!("Args = {args:#?}");

    match args.command {
        Command::Show(cmd_args) => show_main(cmd_args)?,
        Command::Insert(cmd_args) => insert_main(cmd_args)?,
    };

    Ok(())
}

fn show_main(args: ShowArgs) -> Result<()> {
    let mut pgp_wrapper_context =  args.common.new_pgp_wrapper_context()?;
    let enc_path = args.common.get_store_dir().secret_file_path(&args.secret_name);
    let enc = std::fs::read(&enc_path)
        .with_context(|| format!("Failed to read encrypted secret from '{enc_path:?}'"))?;
    let priv_key = args.common.read_priv_key()?;
    let passphrase = "foo";
    let out: Vec<u8> = pgp_wrapper_context.bindings.pgp_wrapper_exports()
                           .decrypt(&mut pgp_wrapper_context.store, &*enc, &*priv_key, passphrase)?
                           .map_err(|e| anyhow::Error::msg(e))?;
    write_output(Path::new("target/test_output/test-pass.decrypted.txt"), &*out)?;

    Ok(())
}

fn insert_main(args: InsertArgs) -> Result<()> {
    let mut pgp_wrapper_context =  args.common.new_pgp_wrapper_context()?;
    let pub_key = args.common.read_pub_key()?;
    let msg = b"hello, world!";
    let out: Vec<u8> = pgp_wrapper_context.bindings.pgp_wrapper_exports()
                           .encrypt(&mut pgp_wrapper_context.store, msg, &*pub_key)?
                           .map_err(|e| anyhow::Error::msg(e))?;
    let out_path = args.common.get_store_dir().secret_file_path(&args.secret_name);
    write_secret(&out_path, &*out)?;

    Ok(())
}

fn write_secret(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    println!("Writing encrypted secret to {path}", path = path.display());
    std::fs::write(path, data)?;
    Ok(())
}
