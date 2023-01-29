use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SecretName {
    name: String,
}

impl std::string::ToString for SecretName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl TryFrom<&OsStr> for SecretName {
    type Error = anyhow::Error;

    fn try_from(s: &OsStr) -> std::result::Result<Self, Self::Error> {
        let str = s.to_str()
                   .ok_or(anyhow::Error::msg("SecretName must be valid UTF8"))?;
        let path = PathBuf::from_str(str).expect("infallible");
        if !path.is_relative() {
            return Err(anyhow::Error::msg("SecretName must be a relative path"));
        }
        for comp in path.components() {
            let comp_str = comp.as_os_str().to_str().expect("already validated UTF8");
            let comp_valid = comp_str.chars().all(valid_secret_name_char);
            if !comp_valid {
                return Err(anyhow::Error::msg("SecretName must be a relative path where each component contains only alphanumeric characters, hypens, and underscores"));
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

pub struct StoreDir {
    path: PathBuf,
}

impl StoreDir {
    pub fn new(path: PathBuf) -> Self {
        StoreDir {
            path: path,
        }
    }

    pub fn secret_file_path(&self, secret_name: &SecretName) -> PathBuf {
        self.path.join(secret_name.to_string())
                 .with_extension("gpg")
    }
}
