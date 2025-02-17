use std::time::Instant;

use crate::bytecode::{
    build_bytecode, build_instruction, Bytecode, Constant, Instruction, LuaInstruction, LuaInt,
    LuaNumber, LuaUint, LuaVersion, Proto,
};

pub struct Reader<'a> {
    pub bytes: &'a Vec<u8>,
    pub bytes_size: usize,

    position: usize,
    endianness: bool,
    size_int: u8,
    max_int: LuaInt,
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

    fn read_u32(&mut self) -> u32 {
        let bytes: Vec<u8> = self.read_u8s(size_of::<u32>());
        if self.endianness {
            return bytes
                .try_into()
                .map(u32::from_le_bytes)
                .expect("Failed to read u32: could not map bytes");
        }
        return bytes
            .try_into()
            .map(u32::from_be_bytes)
            .expect("Failed to read u32: could not map bytes");
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
            panic!("Failed to read int: unhandled size {}", self.size_int);
        };
    }
    // return type should be the biggest of all possible types
    fn read_sizet(&mut self) -> u64 {
        return if self.size_sizet == size_of::<u64>() as u8 {
            self.read_u64()
        } else if self.size_sizet == size_of::<u32>() as u8 {
            self.read_u32() as u64
        } else {
            panic!("Failed to read sizet: unhandled size {}", self.size_sizet);
        };
    }
    // return type should be the biggest of all possible types
    // make sure to reflect changes here to LuaNumber as well
    fn read_number(&mut self) -> LuaNumber {
        return if self.size_luanumber == size_of::<f64>() as u8 {
            self.read_f64()
        } else {
            panic!(
                "Failed to read number: unhandled size {}",
                self.size_luanumber
            );
        };
    }

    fn read_string(&mut self) -> Vec<u8> {
        let size = self.read_sizet();
        if size == 0 {
            return vec![];
        }

        let mut bytes = self.read_u8s(size as usize);
        assert_eq!(bytes.last(), Some(&0), "Expected last character in string to be a 0");

        bytes.remove(bytes.len() - 1);

        bytes
    }

    // return type should be the biggest of all possible types
    fn read_instruction(&mut self) -> LuaInstruction {
        return if self.size_instruction == size_of::<u32>() as u8 {
            self.read_i32() as u32
        } else {
            panic!(
                "Failed to read instruction: unhandled size {}",
                self.size_instruction
            );
        };
    }

    pub fn read(&mut self) -> Bytecode {
        let start_instant: Instant = Instant::now();

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
                panic!(
                    "Failed to read bytecode: unsupported version number ({})",
                    version_number
                );
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
        self.max_int = if size_int == size_of::<i32>() as u8 {
            i32::MAX
        } else {
            panic!(
                "Failed to set max_int: unhandled int size {}",
                self.size_int
            );
        };
        self.size_sizet = size_sizet;
        self.size_instruction = size_instruction;
        self.size_luanumber = size_luanumber;

        let main_proto: Proto = self.read_proto(0, true);

        let bytecode: Bytecode = build_bytecode(
            version,
            format,
            endianness,
            size_int,
            size_sizet,
            size_instruction,
            size_luanumber,
            luanumber_integral,
            main_proto,
            start_instant.elapsed(),
        );

        return bytecode;
    }

    fn read_proto(&mut self, id: LuaInt, is_main: bool) -> Proto {
        let source: Vec<u8> = self.read_string();
        let line_defined: LuaInt = self.read_int();
        let last_line_defined: LuaInt = self.read_int();
        let upvalues_count: u8 = self.read_u8();
        let param_count: u8 = self.read_u8();
        let is_vararg: bool = self.read_u8() != 0;
        let max_stack_size: u8 = self.read_u8();

        let code: Vec<Instruction> = self.read_code();
        let constants: Vec<Constant> = self.read_constants();
        let protos: Vec<Proto> = self.read_protos();
        // eventually we might read instead of skip, depending on if someone requests such functionality
        self.skip_debug();

        let result: Proto = Proto {
            is_main,
            id,

            source,
            line_defined,
            last_line_defined,
            upvalue_count: upvalues_count,
            param_count,
            is_vararg,
            max_stack_size,
            code,
            constants,
            protos,
        };

        return result;
    }

    fn read_code(&mut self) -> Vec<Instruction> {
        let size_code: LuaUint = self.read_int() as LuaUint;
        let mut result: Vec<Instruction> = Vec::with_capacity(size_code as usize);

        let mut raw_instructions: Vec<LuaInstruction> = Vec::with_capacity(size_code as usize);
        for _ in 0..size_code {
            raw_instructions.push(self.read_instruction());
        }

        let mut i: LuaUint = 0;
        while i < size_code {
            let mut inst = build_instruction(
                raw_instructions[i as usize],
                match self.size_int {
                    4 => 32,
                    _ => {
                        panic!(
                            "Failed to get bitsint: unhandled size_int {}",
                            self.size_int
                        )
                    }
                } as LuaInt,
                self.max_int,
            );
            if inst.handle_aux(raw_instructions.get(i as usize + 1)) {
                i += 1;
            }

            result.push(inst);
            i += 1;
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
                4 => Constant::String(self.read_string()),
                _ => {
                    panic!("Failed to read constant: invalid type {}", constant_type);
                }
            });
        }

        return result;
    }
    fn read_protos(&mut self) -> Vec<Proto> {
        let size_proto: LuaInt = self.read_int();
        let mut result: Vec<Proto> = Vec::with_capacity(size_proto as usize);

        for id in 0..size_proto {
            result.push(self.read_proto(id, false));
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
        max_int: LuaInt::MAX,
        size_sizet: 8,
        size_instruction: 4,
        size_luanumber: 8,
    }
}
