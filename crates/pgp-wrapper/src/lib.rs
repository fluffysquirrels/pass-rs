// wit_bindgen_guest_rust::generate!("../../../wit/pgp-wrapper.wit");

#[path = "../../../wit/generated-guest/pgp-wrapper-world.rs"]
#[macro_use]
mod pgp_wrapper_world;

use pgp_wrapper_world::{
    pgp_wrapper_exports,
    __link_section,
};
pub use pgp_wrapper_world::pgp_wrapper_exports::PgpWrapperExports;

pub fn getrandom_always_fail(_: &mut [u8]) -> Result<(), getrandom::Error> {
    unimplemented!()
}
use getrandom::register_custom_getrandom;
register_custom_getrandom!(getrandom_always_fail);

use pgp::{ Deserializable, Message, SignedSecretKey };

pub struct RealPgpWrapperWorld;

impl PgpWrapperExports for RealPgpWrapperWorld {
    fn decrypt(
        encrypted_message: Vec<u8>,
        key: String,
        passphrase: String
    ) -> Result<Vec<u8>, String> {
        let enc_message = Message::from_bytes(&*encrypted_message)
            .map_err(|e| e.to_string())?;
        let (priv_key, _) = SignedSecretKey::from_string(&*key)
            .map_err(|e| e.to_string())?;
        let (decrypter, _) = enc_message.decrypt(
            /* key_pw */ || passphrase,
            &[&priv_key])
            .map_err(|e| e.to_string())?;
        let dec_results: Vec<Result<Message, pgp::errors::Error>> = decrypter.collect();
        match dec_results.len() {
            0 => return Err("No messages in encrypted message".to_string()),
            1 => (),
            n => return Err(format!("{n} messages in encrypted message")),
        };
        let dec_result = &dec_results[0];
        let decrypted_message = dec_result.as_ref().map_err(|e| e.to_string())?;
        let literal_data =
            decrypted_message.get_literal()
            .ok_or("No literal in decrypted message".to_string())?;
        // let str = literal_data.to_string()
        //                       .ok_or("Non UTF-8 string in LiteralData".to_string())?;
        let dec_bytes = literal_data.data().to_vec();
        Ok(dec_bytes)
    }
}

export_pgp_wrapper_world!(RealPgpWrapperWorld);
