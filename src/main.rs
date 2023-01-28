use rand::RngCore;

wasmtime::component::bindgen!("pgp-wrapper");

struct Context;

type Result<T> = std::result::Result<T, anyhow::Error>;

impl get_random::GetRandom for Context {
    fn get_random_bytes(&mut self, len: u32) -> Result<Vec<u8>> {
        let mut ret = Vec::with_capacity(len as usize);
        ret.resize(len as usize, 0);
        rand::thread_rng().fill_bytes(&mut *ret);
        Ok(ret)
    }
}

impl utc_clock::UtcClock for Context {
    fn now(&mut self) -> Result<utc_clock::Datetime> {
        let now_duration: std::time::Duration =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                                        .expect("system time after Unix epoch");
        Ok(utc_clock::Datetime {
            seconds: now_duration.as_secs(),
            nanoseconds: now_duration.subsec_nanos(),
        })
    }
}

fn main() -> Result<()> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let mut engine = wasmtime::Engine::new(&config)?;
    let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.component.wasm");
    let component = wasmtime::component::Component::new(&mut engine, &component_bytes)?;
    let mut linker = wasmtime::component::Linker::new(&mut engine);
    get_random::add_to_linker(&mut linker, |state: &mut Context| state)?;
    utc_clock::add_to_linker(&mut linker, |state: &mut Context| state)?;
    let mut store = wasmtime::Store::new(&engine, Context);
    let (bindings, _instance) = PgpWrapperWorld::instantiate(&mut store, &component, &linker)?;

    decrypt_example(&bindings, &mut store)?;
    encrypt_example(&bindings, &mut store)?;

    Ok(())
}

fn decrypt_example(
    bindings: &PgpWrapperWorld,
    store: &mut wasmtime::Store<Context>
) -> Result<()> {
    println!("decrypt_example");
    let enc = std::fs::read("test_data/pass/test-pass.gpg")?;
    let priv_key = std::fs::read("test_data/priv.key")?;
    let passphrase = "foo";
    let out: Vec<u8> = bindings.pgp_wrapper_exports.decrypt(store, &*enc, &*priv_key, passphrase)?
                               .map_err(|e| anyhow::Error::msg(e))?;
    write_output(Path::new("target/test_output/test-pass.decrypted.txt"), &*out)?;

    Ok(())
}

fn encrypt_example(
    bindings: &PgpWrapperWorld,
    store: &mut wasmtime::Store<Context>
) -> Result<()> {
    println!("encrypt_example");
    let pub_key = std::fs::read("test_data/pub.key")?;
    let msg = b"hello, world!";
    let out: Vec<u8> = bindings.pgp_wrapper_exports.encrypt(store, msg, &*pub_key)?
                               .map_err(|e| anyhow::Error::msg(e))?;
    write_output(Path::new("target/test_output/encrypted.gpg"), &*out)?;

    Ok(())
}

use std::path::Path;

fn write_output(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    println!("Writing output to {path}", path = path.display());
    std::fs::write(path, data)?;
    Ok(())
}
