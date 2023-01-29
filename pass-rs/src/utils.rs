use anyhow::Context;
use crate::Result;
use std::path::Path;

pub fn write_secret(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    println!("Writing encrypted secret to {path}", path = path.display());
    std::fs::write(path, data)?;
    Ok(())
}

pub fn read_passphrase(priv_key_path: &Path) -> Result<String> {
    let prompt = format!("Enter your passphrase for private key {path:?}: ",
                         path = priv_key_path);
    rpassword::prompt_password(&*prompt)
        .with_context(|| "While reading your passphrase")
}
