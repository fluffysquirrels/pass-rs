use crate::Result;
use rand::RngCore;

wasmtime::component::bindgen!("pgp-wrapper");

pub struct Context {
    pub store: wasmtime::Store<State>,
    pub bindings: PgpWrapperWorld,
}

pub struct State;

impl Context {
    pub fn new() -> Result<Context> {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true);
        let mut engine = wasmtime::Engine::new(&config)?;
        let component_bytes = include_bytes!("../target/wasm32-unknown-unknown/release/pgp_wrapper.component.wasm");
        let component = wasmtime::component::Component::new(&mut engine, &component_bytes)?;
        let mut linker = wasmtime::component::Linker::new(&mut engine);
        get_random::add_to_linker(&mut linker, |state: &mut State| state)?;
        utc_clock::add_to_linker(&mut linker, |state: &mut State| state)?;
        let mut store = wasmtime::Store::new(&engine, State);
        let (bindings, _instance) = PgpWrapperWorld::instantiate(&mut store, &component, &linker)?;

        Ok(Context {
            store: store,
            bindings: bindings,
        })
    }
}

impl get_random::GetRandom for State {
    fn get_random_bytes(&mut self, len: u32) -> Result<Vec<u8>> {
        let mut ret = Vec::with_capacity(len as usize);
        ret.resize(len as usize, 0);
        rand::thread_rng().fill_bytes(&mut *ret);
        Ok(ret)
    }
}

impl utc_clock::UtcClock for State {
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
