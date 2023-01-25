// wit_bindgen_guest_rust::generate!("../../../wit/pgp-wrapper.wit");

#[path = "../../../wit/generated/pgp-wrapper-world.rs"]
#[macro_use]
mod generated;

use generated::{
    pgp_wrapper_exports,
    __link_section,
};
pub use pgp_wrapper_exports::PgpWrapperExports;

pub struct DummyPgpWrapperWorld;

impl PgpWrapperExports for DummyPgpWrapperWorld {
    fn decrypt(
        _encrypted_message: String,
        _key: String,
        _passphrase: String
    ) -> Result<String, String> {
        Ok("foo".to_string())
    }
}

export_pgp_wrapper_world!(DummyPgpWrapperWorld);
