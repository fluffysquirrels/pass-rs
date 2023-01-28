# To-do

## WIP

* encrypt_example is crashing on std::time::SystemTime::now(), which is unimplemented for wasm32.
    * Patch chrono to call a component-model import

## Must do (before publishing)

* CLI interface with clap
    * Read keys and encrypted data from passed file paths
    * Read passphrase from terminal
    * pass list
    * pass show
    * pass insert
    * pass rm
