mod pgp_wrapper;

use std::path::Path;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let mut context =  pgp_wrapper::Context::new()?;

    decrypt_example(&mut context)?;
    encrypt_example(&mut context)?;

    Ok(())
}

fn decrypt_example(
    context: &mut pgp_wrapper::Context,
) -> Result<()> {
    println!("decrypt_example");
    let enc = std::fs::read("test_data/pass/test-pass.gpg")?;
    let priv_key = std::fs::read("test_data/priv.key")?;
    let passphrase = "foo";
    let out: Vec<u8> = context.bindings.pgp_wrapper_exports()
                              .decrypt(&mut context.store, &*enc, &*priv_key, passphrase)?
                              .map_err(|e| anyhow::Error::msg(e))?;
    write_output(Path::new("target/test_output/test-pass.decrypted.txt"), &*out)?;

    Ok(())
}

fn encrypt_example(
    context: &mut pgp_wrapper::Context,
) -> Result<()> {
    println!("encrypt_example");
    let pub_key = std::fs::read("test_data/pub.key")?;
    let msg = b"hello, world!";
    let out: Vec<u8> = context.bindings.pgp_wrapper_exports()
                              .encrypt(&mut context.store, msg, &*pub_key)?
                              .map_err(|e| anyhow::Error::msg(e))?;
    write_output(Path::new("target/test_output/encrypted.gpg"), &*out)?;

    Ok(())
}

fn write_output(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    println!("Writing output to {path}", path = path.display());
    std::fs::write(path, data)?;
    Ok(())
}
