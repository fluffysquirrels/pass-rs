# To-do

## WIP

* CLI

## Must do (before publishing)

* Logging
* CLI
    * pass-rs find
        * Take an optional subdir path
    * pass-rs ls ?
    * pass-rs cp
    * pass-rs mv
    * pass-rs rm

## Might do

* Split `args::CommonArgs` into separate use-based structs. Goal is to
  see more accurate prompts, e.g. no prompt for keys in `find`, and no
  prompt for private key in `insert`. Another benefit: automatic clap
  validation for required values like public key in `insert`, rather
  than returning an `Err` later dynamically.
    * `DecryptArgs`
    * `EncryptArgs`
    * `StoreDir`
* Encrypt to multiple keys
* Check the key is correct (based on `${STORE}/.gpg-id`) before operating
* Access keys in the GPG key store
* Key generation

* chrono
    * Implement DateTime<Local>::now(); use an import to access the local timezone UTC offset.

* Upstream patches
    * chrono
    * pgp crate
    * iana-time-zone
    * [wasi-clocks](https://github.com/WebAssembly/wasi-clocks) specification / impl
