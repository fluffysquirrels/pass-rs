use anyhow::Context;
use crate::Result;
use std::fs::{ File, OpenOptions };
use std::io::Write;
use std::path::Path;

pub fn write_secret(path: &Path, data: &[u8], replace: bool) -> Result<()> {
    tracing::info!(path = path.display().to_string(), "Writing encrypted secret");
    (|| -> Result<()> { // Wrap any file operation errors with anyhow.
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file: File = OpenOptions::new().write(true)
                                               .create_new(!replace)
                                               .open(path)?;
        file.write_all(data)?;
        Ok(())
    })().with_context(|| format!("Failed to write encrypted secret to {path}",
                                 path = path.display()))?;

    Ok(())
}

pub fn read_passphrase(priv_key_path: &Path) -> Result<String> {
    let prompt = format!("Enter your passphrase for private key {path:?}: ",
                         path = priv_key_path);
    rpassword::prompt_password(&*prompt)
        .with_context(|| "While reading your passphrase")
}
