//    let component = Component::new(engine, &component)?;
//    let mut store = Store::new(engine, String::new());
//    let mut linker = Linker::new(engine);
//    linker.root().func_wrap(
//        "host",
//        |store: StoreContextMut<String>, (arg,): (String,)| {
//            assert_eq!(*store.data(), arg);
//            Ok((arg,))
//        },
//    )?;
//    let instance = linker.instantiate(&mut store, &component)?;
//    let func = instance.get_typed_func::<(String,), (String,), _>(&mut store, "echo")?;
//
//    for string in STRINGS {
//        println!("testing string {string:?}");
//        *store.data_mut() = string.to_string();
//        let (ret,) = func.call(&mut store, (string.to_string(),))?;
//        assert_eq!(ret, *string);
//        func.post_return(&mut store)?;

struct Context;

fn main() -> Result<(), anyhow::Error> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let mut engine = wasmtime::Engine::new(&config)?;
//    let component_bytes = include_bytes!("../target/wasm32-wasi/release/pgp_wrapper.wasm");
//    let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.wasm");
    let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.component.wasm");
    let component = wasmtime::component::Component::new(&mut engine, &component_bytes)?;
    let mut store = wasmtime::Store::new(&mut engine, Context {});
    let mut linker = wasmtime::component::Linker::new(&mut engine);
//    linker.root().func_wrap(
//        "host",
//        |store: StoreContextMut<String>, (arg,): (String,)| {
//            assert_eq!(*store.data(), arg);
//            Ok((arg,))
//        },
//    )?;
    let instance = linker.instantiate(&mut store, &component)?;
    let func = { // Scope exports
        let mut exports = instance.exports(&mut store);
        let mut mod_instance = exports.instance("pgp-wrapper-exports").expect("to find instance");
        let func = mod_instance.typed_func::<(String, String, String,), (Result<String,String>,)>("decrypt")?;
        func
    };
    // let func = instance.get_typed_func::<(String, String, String,), (Result<String,String>,), _>(&mut store, "pgp-wrapper-exports#decrypt")?;
    let (res,) = func.call(&mut store, ("encrypted".to_string(), "key".to_string(), "passphrase".to_string()))?;
    let out = res.unwrap();
    println!("out = '{out}'");
    func.post_return(&mut store)?;
    Ok(())
}
