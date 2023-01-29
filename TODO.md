# To-do

## WIP

* CLI
    * pass show
    * pass insert
        * Read secret from terminal

## Must do (before publishing)

* Logging
* CLI
    * pass insert
        * Don't overwrite secrets by default
    * pass list
    * pass mv
    * pass rm

## Might do

* Encrypt to multiple keys
* Access keys in the GPG key store
* Key generation

* chrono
    * Implement DateTime<Local>::now(); use an import to access the local timezone UTC offset.

* Upstream patches
    * chrono
    * pgp crate
    * iana-time-zone
    * [wasi-clocks](https://github.com/WebAssembly/wasi-clocks) specification / impl
