# To-do

## WIP

* CLI

## Must do (before publishing)

* Logging
* CLI
    * pass ls
        * Create alias `ls` for existing `list`
        * Take an optional subdir path
    * pass cp
    * pass mv
    * pass rm

## Might do

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
