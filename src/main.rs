use bytecode::Bytecode;
use decompiler::{build_decompiler, Decompiler};
use read::{build_reader, Reader};
use std::{env, fs, process::exit};

pub mod bytecode;
pub mod decompiler;
pub mod read;
pub mod util;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc < 2 {
        display_help();
    }

    let mut do_decompile: bool = false;
    let mut input_path: Option<&String> = None;
    for i in 1..argv.len() {
        let value: &String = &argv[i];
        if value.starts_with("-") {
            let option: &str = &value[1..value.len()];
            match option {
                "dec" => {
                    do_decompile = true;
                }
                _ => {
                    panic!("unexpected argument {} found", value);
                }
            }
        } else {
            input_path = Some(value);
        }
    }

    if input_path.is_none() {
        panic!("expected argument file not found");
    }

    let input_path: &String = input_path.unwrap();

    let input_bytes: Vec<u8> =
        fs::read(input_path).expect(format!("Failed to read file at {}", input_path).as_str());

    let mut reader: Reader = build_reader(&input_bytes);
    let mut bytecode: Bytecode = reader.read();

    if do_decompile {
        let mut decompiler: Decompiler = build_decompiler(bytecode);

        println!("{}", decompiler.decompile());

        return;
    }
    bytecode.print_disassembly();
}
fn display_help() {
    println!("inu - A disassembler for lua\n");
    println!("Usage: inu [options] file");
    println!();
    println!("options:");
    println!("    -dec : decompile");
    exit(0);
}
