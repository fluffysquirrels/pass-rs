use anyhow::anyhow;

wasmtime::component::bindgen!("pgp-wrapper");

struct Context;

impl pgp_wrapper_imports::PgpWrapperImports for Context {}

fn main() -> Result<(), anyhow::Error> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let mut engine = wasmtime::Engine::new(&config)?;
    let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.component.wasm");
    let component = wasmtime::component::Component::new(&mut engine, &component_bytes)?;
    let mut linker = wasmtime::component::Linker::new(&mut engine);
    pgp_wrapper_imports::add_to_linker(&mut linker, |state: &mut Context| state)?;
    let mut store = wasmtime::Store::new(&engine, Context);
    let (bindings, _instance) = PgpWrapperWorld::instantiate(&mut store, &component, &linker)?;

    let enc = std::fs::read("test_data/pass/test-pass.gpg")?;
    let key = std::fs::read_to_string("test_data/priv.key")?;
    let passphrase = "foo";
    let out: Vec<u8> = bindings.pgp_wrapper_exports.decrypt(&mut store, &*enc, &*key, passphrase)?
                               .map_err(|e| anyhow::Error::msg(e))?;
    println!("out = '{out:?}'");
    Ok(())
}
