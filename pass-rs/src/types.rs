use anyhow::Context;
use crate::Result;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::{ Path, PathBuf };
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

impl SecretName {
    pub fn as_str(&self) -> &str {
        &*self.name
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

    /// Return an alphabetically sorted Vec of SecretName's in this password store.
    pub fn find_secrets(&self) -> Result<Vec<SecretName>> {
        let mut ret = Vec::new();

        self.visit_secrets(&mut |secret: &SecretName| {
            ret.push(secret.clone());
        })?;

        ret.sort_by(|a: &SecretName, b: &SecretName| -> Ordering { a.name.cmp(&b.name) });

        Ok(ret)
    }

    fn visit_secrets<F>(&self, cb: &mut F) -> Result<()>
    where F: FnMut(&SecretName) {
        fn visit_dir<F>(root: &Path, curr: &Path, cb: &mut F) -> Result<()>
        where F: FnMut(&SecretName) {
            for entry in curr.read_dir()? {
                let entry = entry?;
                let meta = entry.metadata()?;
                if meta.is_file()
                        && entry.path().extension() == Some(OsStr::new("gpg")) {
                    let secret_rel_path = entry.path().strip_prefix(root)?
                                               .with_extension("");
                    let secret_name = SecretName::try_from(secret_rel_path.as_os_str())?;
                    cb(&secret_name);
                } else if meta.is_dir() {
                    visit_dir(root, &*entry.path(), cb)?;
                }
            }
            Ok(())
        }

        (|| { // Wrap all errors in anyhow context.
            let root = &*self.path.canonicalize()?;
            visit_dir(root, root, cb)
        })().with_context(|| format!("Visiting secrets in the password store {path}",
                                     path = self.path.display()))
    }
}
