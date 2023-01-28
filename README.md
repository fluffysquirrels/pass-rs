# pass-rs

A password manager like [`pass`](https://www.passwordstore.org/) implemented in Rust.

Currently this is just a prototype that encrypts and decrypts data in
the required format.

## Getting started

This crate uses submodules for external crate dependencies that have
patches that are not yet merged upstream. To initialise the submodules
after cloning run:

```
git submodule update --init --recursive
```

Ensure you have a recent version of the rust toolchain installed,
for example using [`rustup`](https://rustup.rs/).

If necessary, install the `wasm32-unknown-unknown` target:

```
rustup target add wasm32-unknown-unknown
```

In a Unix environment to build the whole project simply run the script
`${REPO}/pass-rs/bin/build`
and to run it run the script `${REPO}/pass-rs/bin/run`

## Sandboxing

The implementation sandboxes the cryptographic code in WebAssembly
(Wasm) using the Wasmtime runtime. The original purpose of the project
was to experiment with this sandboxing approach with Rust. The goal is
to prevent malicious or buggy code within the sandbox from
compromising the host system or leaking the secrets stored in the
password store (e.g. over the network). This could be done with a
supply chain attack on one of the dependencies, for example.

The crate `pgp-wrapper` (in `${REPO}/pass-rs/crates/pgp-wrapper/`)
contains all the cryptographic code (implemented using the pure-Rust
[`pgp`](https://crates.io/crates/pgp) crate) and is compiled to a Wasm
component. The top-level crate `pass-rs` (in `${REPO}/pass-rs/`) is
compiled to native code and runs `pgp-wrapper` in a sandbox using the [Wasmtime](https://wasmtime.dev/) runtime.

Part of the Wasm sandbox design is that Wasm guest modules can only
influence the host system by returning data to the host, or running
code to via the functions the module imports. Functions are imported
either from the host runtime or other Wasm modules through a
dependency tree. Wasmtime lets us configure explicitly which functions
the host provides to the guest, and only functions provided by the
host can perform host syscalls. This can strongly limit the
capability of buggy or malicious code to influence the host
system. For example `pgp-wrapper` does not need to read or write to
the host file system or access the network, so this is simply
forbidden.

When evaluating the security of `pass-rs` we do not need to trust the
code compiled into `pgp-wrapper` not to leak secrets over the network,
for example, so long as the host binary (including Wasmtime) functions
correctly in sandboxing `pgp-wrapper`. This could reduce the effort
needed to audit the security of the application compared to a version
that did not use sandboxing (although it does introduce Wasmtime as a
dependency to consider).
