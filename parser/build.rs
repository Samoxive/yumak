#![feature(plugin,main)]

extern crate lalrpop;

use std::path::Path;

fn main() {
    lalrpop::process_root().unwrap();
}