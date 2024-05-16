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
    * Probably don't try too hard to be compatible with `pass`
    * `.public_keys/` in each directory? Would need to duplicate keys
      or use symlinks, doesn't sound like a nice user experience
    * `.public_keys/` in the root and `.gpg-id` in each directory?
      Can't share a subdirectory without further work, e.g. in a git repo.
      Might not do that anyway.
    * `.gpg-id` in each directory and public keys somewhere else,
      e.g. `~/.config/pass-rs/public_keys/${ID}.pub` or GPG's key store
* git integration inspired by `pass`
    * Commit after every mutation
        * `insert`
        * `init` / set keys
        * `cp`
        * `rm`
        * `mv`
    * git command, e.g. `pass-rs git push`, that forwards commands to git
    * Multiple repos under subdirectories? No.
      Doesn't really work for the git root without .gitignore or submodules or something.
* Check the key is correct (based on `${STORE}/.gpg-id`) before operating
* Access keys in the GPG key store?
* Key generation

* chrono
    * Implement DateTime<Local>::now(); use an import to access the local timezone UTC offset.

* Upstream patches
    * chrono
    * pgp crate
    * iana-time-zone
    * [wasi-clocks](https://github.com/WebAssembly/wasi-clocks) specification / impl
