use bytecode::Bytecode;
use read::{build_reader, Reader};
use std::{env, fs, process::exit};

pub mod bytecode;
pub mod read;
pub mod util;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc < 2 {
        display_help();
    }

    let mut do_decompile: bool = false;
    let mut do_psuedo_code: bool = false;
    let mut input_path: Option<&String> = None;
    for i in 1..argv.len() {
        let value: &String = &argv[i];
        if value.starts_with("--") {
            let option: &str = &value[2..value.len()];
            match option {
                "dec" => {
                    do_decompile = true;
                }
                "psuedo" => {
                    do_psuedo_code = true;
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
        eprintln!("decompile is currently disabled");
        return;
    }
    bytecode.print_disassembly(do_psuedo_code);
}
fn display_help() {
    println!("inu - A disassembler for lua\n");
    println!("Usage: inu [options] file");
    println!();
    println!("options:");
    println!("    --dec : decompile");
    println!("    --psuedo: omit disassembly");
    exit(0);
}
