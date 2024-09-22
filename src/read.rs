use std::process::exit;

use crate::bytecode::{Bytecode, Constant, LuaInt, LuaNumber, LuaVersion, Proto};

pub struct Reader<'a> {
    pub bytes: &'a Vec<u8>,
    pub bytes_size: usize,

    position: usize,
    endianness: bool,
    size_int: u8,
    size_sizet: u8,
    size_instruction: u8,
    size_luanumber: u8,
}

impl<'a> Reader<'a> {
    fn validate_read(&self, size: usize) {
        assert!(
            self.position + size <= self.bytes_size,
            "Attempt to read past bytecode length (position: {}, size: {}, bytes_size: {})",
            self.position,
            size,
            self.bytes_size
        );
    }
    fn read_u8(&mut self) -> u8 {
        self.validate_read(size_of::<u8>());

        let result: u8 = self.bytes[self.position];
        self.position += 1;
        return result;
    }
    fn read_u8s(&mut self, length: usize) -> Vec<u8> {
        self.validate_read(length * size_of::<u8>());

        let result: &[u8] = &self.bytes[self.position..self.position + length];
        self.position += length * size_of::<u8>();
        return result.into();
    }

    fn read_u64(&mut self) -> u64 {
        let bytes: Vec<u8> = self.read_u8s(size_of::<u64>());
        if self.endianness {
            return bytes
                .try_into()
                .map(u64::from_le_bytes)
                .expect("Failed to read u64: could not map bytes");
        }
        return bytes
            .try_into()
            .map(u64::from_be_bytes)
            .expect("Failed to read u64: could not map bytes");
    }
    fn read_i32(&mut self) -> i32 {
        let bytes: Vec<u8> = self.read_u8s(size_of::<i32>());
        if self.endianness {
            return bytes
                .try_into()
                .map(i32::from_le_bytes)
                .expect("Failed to read i32: could not map bytes");
        }
        return bytes
            .try_into()
            .map(i32::from_be_bytes)
            .expect("Failed to read i32: could not map bytes");
    }
    fn read_f64(&mut self) -> f64 {
        let bytes: Vec<u8> = self.read_u8s(size_of::<f64>());
        if self.endianness {
            return bytes
                .try_into()
                .map(f64::from_le_bytes)
                .expect("Failed to read f64: could not map bytes");
        }
        return bytes
            .try_into()
            .map(f64::from_be_bytes)
            .expect("Failed to read f64: could not map bytes");
    }

    // return type should be the biggest of all possible types
    // make sure to reflect changes here to LuaInt as well
    fn read_int(&mut self) -> LuaInt {
        return if self.size_int == size_of::<i32>() as u8 {
            self.read_i32()
        } else {
            eprintln!("Failed to read int: unhandled size {}", self.size_int);
            exit(1);
        };
    }
    // return type should be the biggest of all possible types
    fn read_sizet(&mut self) -> u64 {
        return if self.size_sizet == size_of::<u64>() as u8 {
            self.read_u64()
        } else {
            eprintln!("Failed to read sizet: unhandled size {}", self.size_sizet);
            exit(1);
        };
    }
    // return type should be the biggest of all possible types
    // make sure to reflect changes here to LuaNumber as well
    fn read_number(&mut self) -> LuaNumber {
        return if self.size_luanumber == size_of::<f64>() as u8 {
            self.read_f64()
        } else {
            eprintln!(
                "Failed to read number: unhandled size {}",
                self.size_luanumber
            );
            exit(1);
        };
    }

    fn read_string(&mut self) -> Option<String> {
        let size = self.read_sizet();
        if size == 0 {
            return None;
        }

        let mut value: String = String::from_utf8(self.read_u8s(size as usize))
            .expect("Failed to read string: could not convert bytes to string");

        assert_eq!(
            value.chars().last().unwrap_or(1 as char),
            0 as char,
            "Expected last character in read string to be a 0"
        );
        value.remove(value.len() - 1);

        return Some(value);
    }

    // return type should be the biggest of all possible types
    fn read_instruction(&mut self) -> u32 {
        return if self.size_instruction == size_of::<u32>() as u8 {
            self.read_i32() as u32
        } else {
            eprintln!(
                "Failed to read instruction: unhandled size {}",
                self.size_instruction
            );
            exit(1);
        };
    }

    pub fn read(&mut self) -> Bytecode {
        assert_eq!(
            self.read_u8(),
            0o33,
            "Failed to read bytecode: expected first byte to equal octal 33"
        );
        assert_eq!(
            self.read_u8s(3),
            ['L', 'u', 'a']
                .iter()
                .map(|&c| c as u8)
                .collect::<Vec<u8>>(),
            "Failed to read bytecode: expected the next 3 bytes to equal 'Lua'"
        );
        let version_number: u8 = self.read_u8();

        let version: LuaVersion = match version_number {
            0x51 => LuaVersion::Lua51,
            _ => {
                eprintln!(
                    "Failed to read bytecode: unsupported version number ({})",
                    version_number
                );
                exit(1);
            }
        };

        let format: u8 = self.read_u8();
        let endianness: bool = self.read_u8() == 1;
        let size_int: u8 = self.read_u8();
        let size_sizet: u8 = self.read_u8();
        let size_instruction: u8 = self.read_u8();
        let size_luanumber: u8 = self.read_u8();
        let luanumber_integral: bool = self.read_u8() == 1;

        self.endianness = endianness;
        self.size_int = size_int;
        self.size_sizet = size_sizet;
        self.size_instruction = size_instruction;
        self.size_luanumber = size_luanumber;

        let main_proto: Proto = self.read_proto();

        let bytecode: Bytecode = Bytecode {
            version,
            format,
            endianness,
            size_int,
            size_sizet,
            size_instruction,
            size_luanumber,
            luanumber_integral,
            main_proto,
        };

        return bytecode;
    }

    fn read_proto(&mut self) -> Proto {
        let source: Option<String> = self.read_string();
        let line_defined: LuaInt = self.read_int();
        let last_line_defined: LuaInt = self.read_int();
        let upvalues_count: u8 = self.read_u8();
        let param_count: u8 = self.read_u8();
        let is_vararg: bool = self.read_u8() == 1;
        let max_stack_size: u8 = self.read_u8();

        let code: Vec<u32> = self.read_code();
        let constants: Vec<Constant> = self.read_constants();
        let protos: Vec<Proto> = self.read_protos();
        // eventually we might read instead of skip, depending on if someone requests such functionality
        self.skip_debug();

        let result: Proto = Proto {
            source,
            line_defined,
            last_line_defined,
            upvalues_count,
            param_count,
            is_vararg,
            max_stack_size,
            code,
            constants,
            protos,
        };

        return result;
    }

    fn read_code(&mut self) -> Vec<u32> {
        let size_code = self.read_int() as u32;
        let mut result: Vec<u32> = Vec::with_capacity(size_code as usize);

        for _ in 0..size_code {
            result.push(self.read_instruction());
        }

        return result;
    }
    fn read_constants(&mut self) -> Vec<Constant> {
        let size_constants: LuaInt = self.read_int();
        let mut result: Vec<Constant> = Vec::with_capacity(size_constants as usize);

        for _ in 0..size_constants {
            let constant_type: u8 = self.read_u8();
            result.push(match constant_type {
                0 => Constant::Nil,
                1 => Constant::Boolean(self.read_u8() == 1),
                3 => Constant::Number(self.read_number()),
                4 => Constant::String(self.read_string().unwrap_or("".to_string())),
                _ => {
                    eprintln!("Failed to read constant: invalid type {}", constant_type);
                    exit(1);
                }
            });
        }

        return result;
    }
    fn read_protos(&mut self) -> Vec<Proto> {
        let size_proto: LuaInt = self.read_int();
        let mut result: Vec<Proto> = Vec::with_capacity(size_proto as usize);

        for _ in 0..size_proto {
            result.push(self.read_proto());
        }

        return result;
    }

    fn skip_debug(&mut self) {
        let size_lineinfo: LuaInt = self.read_int();
        for _ in 0..size_lineinfo {
            self.read_int();
        }

        let size_localvars: LuaInt = self.read_int();
        for _ in 0..size_localvars {
            self.read_string();
            self.read_int();
            self.read_int();
        }

        let size_upvalues: LuaInt = self.read_int();
        for _ in 0..size_upvalues {
            self.read_string();
        }
    }
}

pub fn build_reader(bytes: &Vec<u8>) -> Reader {
    Reader {
        bytes,
        bytes_size: bytes.len(),
        position: 0,
        endianness: false,
        size_int: 4,
        size_sizet: 8,
        size_instruction: 4,
        size_luanumber: 8,
    }
}
