use crate::instructions::Instruction;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn assemble_file(path: &Path) -> std::io::Result<[u8; 256]> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut program = [0u8; 256];
    let mut i = 0;

    let mut labels = HashMap::new();
    let mut refs = HashMap::new();

    for res in reader.lines() {
        let mut line = &(res?)[..];
        if let Some((body, _comment)) = line.split_once("//") {
            line = body;
        }
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(label) = line.strip_prefix("'") {
            labels.insert(label.to_owned(), i as u8);
            continue;
        }

        let (name, args_str) = line.split_once(" ").unwrap_or((line, ""));

        let Ok(ins) = Instruction::from_str(name) else {
            panic!("Invalid instruction: {name}");
        };

        let args: Vec<&str> = if args_str.is_empty() {
            vec![]
        } else {
            args_str.split(',').collect()
        };

        if ins.num_args() != args.len() as u8 {
            panic!("Incorrect number of arguments: {name}");
        }

        program[i] = ins as u8;
        if i == 255 {
            println!("Reached maximum program bytes");
        }
        i += 1;
        for arg in args {
            let val = if let Some(label) = arg.strip_prefix("'") {
                refs.insert(i, label.to_owned());
                0
            } else {
                arg.parse().unwrap()
            };
            program[i] = val;
            if i == 255 {
                println!("Reached maximum program bytes");
            }
            i += 1;
        }
    }
    for (loc, label) in refs {
        let Some(val) = labels.get(&label) else {
            panic!("Unknown label {label}");
        };
        program[loc] = *val;
    }
    Ok(program)
}
