use anyhow::anyhow;

#[cfg(target_arch = "ignore")] // Disable this for now.
mod manual_wrapper {
    struct Context;

    struct PgpWrapper {
        engine: wasmtime::Engine,
        store: wasmtime::Store<Context>,
        instance: wasmtime::component::Instance,
        decrypt_func: wasmtime::component::TypedFunc<(Vec<u8>, String, String),
        (Result<Vec<u8>, String>,)>,
    }

    impl PgpWrapper {
        pub fn new() -> Result<PgpWrapper, anyhow::Error> {
            let mut config = wasmtime::Config::new();
            config.wasm_component_model(true);
            let mut engine = wasmtime::Engine::new(&config)?;
            //    let component_bytes = include_bytes!("../target/wasm32-wasi/release/pgp_wrapper.wasm");
            //    let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.wasm");
            let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.component.wasm");
            let component = wasmtime::component::Component::new(&mut engine, &component_bytes)?;
            let mut store = wasmtime::Store::new(&mut engine, Context {});
            #[allow(unused_mut)] // No imports added yet, but we will need them.
            let mut linker = wasmtime::component::Linker::new(&mut engine);
            //    linker.root().func_wrap(
            //        "host",
            //        |store: StoreContextMut<String>, (arg,): (String,)| {
            //            assert_eq!(*store.data(), arg);
            //            Ok((arg,))
            //        },
            //    )?;
            let instance = linker.instantiate(&mut store, &component)?;
            let mut exports = instance.exports(&mut store);
            let mut pgp_wrapper_exports =
                exports.instance("pgp-wrapper-exports")
                .ok_or(anyhow!("Unable to find export in instance"))?;
            let decrypt_func =
                pgp_wrapper_exports.typed_func::<(Vec<u8>, String, String,),
            (Result<Vec<u8>,String>,)>("decrypt")?;
            drop(pgp_wrapper_exports);
            drop(exports);
            let pgp_wrapper = PgpWrapper {
                engine: engine,
                store: store,
                instance: instance,
                decrypt_func: decrypt_func,
            };
            Ok(pgp_wrapper)
        }

        pub fn decrypt(&mut self, enc: Vec<u8>, key: String, passphrase: String)
                       -> Result<Vec<u8>, anyhow::Error> {
                           let wasmtime_res = self.decrypt_func.call(&mut self.store, (enc, key, passphrase));
                           let func_res = wasmtime_res?.0
                               .map_err(|e| anyhow::Error::msg(e));
                           self.decrypt_func.post_return(&mut self.store)?;
                           func_res
                       }
    }

    fn main() -> Result<(), anyhow::Error> {
        let mut pgp_wrapper = PgpWrapper::new()?;
        let enc: Vec<u8> = include_bytes!("../test_data/pass/test-pass.gpg").to_vec();
        let key = include_str!("../test_data/priv.key").to_string();
        let passphrase = "foo".to_string();
        let out: Vec<u8> = pgp_wrapper.decrypt(enc, key, passphrase)?;
        println!("out = '{out:?}'");
        Ok(())
    }
}

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

    let enc = include_bytes!("../test_data/pass/test-pass.gpg");
    let key = include_str!("../test_data/priv.key");
    let passphrase = "foo";
    let out: Vec<u8> = bindings.pgp_wrapper_exports.decrypt(&mut store, enc, key, passphrase)?
                               .map_err(|e| anyhow::Error::msg(e))?;
    println!("out = '{out:?}'");
    Ok(())
}
