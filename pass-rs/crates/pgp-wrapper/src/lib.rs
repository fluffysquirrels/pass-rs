// wit_bindgen_guest_rust::generate!("../../../wit/pgp-wrapper.wit");

#[path = "../../../wit/generated-guest/pgp-wrapper-world.rs"]
#[macro_use]
#[allow(dead_code)]
mod pgp_wrapper_world;

use core::num::NonZeroU32;
use getrandom::register_custom_getrandom;
use pgp::{
    Deserializable,
    Message,
    SignedPublicKey,
    SignedSecretKey,
    crypto::sym::SymmetricKeyAlgorithm,
    ser::Serialize,
};
use pgp_wrapper_world::{
    get_random,
    pgp_wrapper_exports,
    __link_section,
};
pub use pgp_wrapper_world::pgp_wrapper_exports::PgpWrapperExports;

fn getrandom_with_import(out: &mut [u8]) -> Result<(), getrandom::Error> {
    let count = out.len();
    let count_u32: u32 = count.try_into().map_err(|_e| getrandom::Error::from(NonZeroU32::new(getrandom::Error::CUSTOM_START).expect("NonZeroU32::new")))?;
    let bytes = get_random::get_random_bytes(count_u32);
    out.copy_from_slice(&*bytes);
    Ok(())
}

register_custom_getrandom!(getrandom_with_import);

struct ImportRng;

impl rand_core::RngCore for ImportRng {
    fn next_u32(&mut self) -> u32 {
        rand_core::impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        rand_core::impls::next_u64_via_fill(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let count = dest.len();
        let count_u32: u32 = count.try_into().expect("ImportRng::fill_bytes expected to convert len to u32");
        let bytes = get_random::get_random_bytes(count_u32);
        dest.copy_from_slice(&*bytes);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl rand_core::CryptoRng for ImportRng {}

pub struct RealPgpWrapperWorld;

impl PgpWrapperExports for RealPgpWrapperWorld {
    fn decrypt(
        encrypted_message: Vec<u8>,
        priv_key_bytes: Vec<u8>,
        passphrase: String
    ) -> Result<Vec<u8>, String> {
        let enc_message = Message::from_bytes(&*encrypted_message)
            .map_err(|e| e.to_string())?;
        let priv_key = SignedSecretKey::from_bytes(&*priv_key_bytes)
            .map_err(|e| e.to_string())?;
        let (decrypter, _) = enc_message.decrypt(
            /* msg_pw */ || "".to_string(), // Not used, removed in pgp crate's master.
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

    fn encrypt(
        message_bytes: Vec<u8>,
        pub_key_bytes: Vec<u8>
    ) -> Result<Vec<u8>, String> {
        let message = Message::new_literal_bytes(/* file_name */ "pass-rs", &*message_bytes);
        let pub_key = SignedPublicKey::from_bytes(&*pub_key_bytes)
                                      .map_err(|e| e.to_string())?;
        let mut rng = ImportRng;
        let enc_message =
            message.encrypt_to_keys(
                &mut rng,
                SymmetricKeyAlgorithm::AES256,
                &[&pub_key]
            ).map_err(|e| e.to_string())?;
        let enc_bytes = enc_message.to_bytes()
            .map_err(|e| e.to_string())?;
        Ok(enc_bytes)
    }
}

export_pgp_wrapper_world!(RealPgpWrapperWorld);
