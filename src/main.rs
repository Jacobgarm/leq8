#![allow(dead_code)]

use std::{env, path::Path};
mod assembly;
mod display;
pub mod instructions;
pub mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let Some(path_str) = args.get(1) else {
        println!("No program specified");
        return;
    };
    let program = assembly::assemble_file(Path::new(path_str)).unwrap();
    display::run(program);
}
