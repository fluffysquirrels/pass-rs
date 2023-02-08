# Test data

This directory contains a sample GPG key pair and `pass` password store.

* `./priv.key` is a GPG private key with passphrase `foo`.
* `./pub.key` is the GPG public key counterpart to `./priv.key`.
* `./pass` is a password store encrypted for `./priv.key`.

Run `source ./set-envs` to set environment variables so pass-rs uses
this key pair and password store for its commands.
