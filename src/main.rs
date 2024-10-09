use bytecode::Bytecode;
use read::{build_reader, Reader};
use std::{env, fs, process::exit};

pub mod bytecode;
pub mod read;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc < 2 {
        display_help();
    }

    let input_path: &String = &argv[1];

    let input_bytes: Vec<u8> =
        fs::read(input_path).expect(format!("Failed to read file at {}", input_path).as_str());

    let mut reader: Reader = build_reader(&input_bytes);
    let mut bytecode: Bytecode = reader.read();

    /*
    println!(
        "version: {:?}, format: {}, endianness: {}, size_int: {}, size_sizet: {}, size_instruction: {}, size_luanumber: {}, luanumber_integral: {}",
        bytecode.version, bytecode.format, bytecode.endianness, bytecode.size_int, bytecode.size_sizet,
        bytecode.size_instruction, bytecode.size_luanumber, bytecode.luanumber_integral
    );
    println!("main_proto: {:?}", bytecode.main_proto);
    */

    bytecode.print_disassembly();
}
fn display_help() {
    println!("inu - A disassembler for lua\n");
    println!("Usage: inu file");
    exit(0);
}
