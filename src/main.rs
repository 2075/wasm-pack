extern crate wasm_pack;

extern crate indicatif;
#[macro_use]
extern crate quicli;

use quicli::prelude::*;
use wasm_pack::Cli;
use wasm_pack::command::run_wasm_pack;

main!(|args: Cli, log_level: verbosity| {
    run_wasm_pack(args.cmd)?;
});
