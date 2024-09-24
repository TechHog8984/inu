use std::{collections::HashMap, time::Duration};

#[derive(Debug)]
pub enum LuaVersion {
    Lua51,
}

pub type LuaInt = i32; // make sure this type's size matches size_luaint (or is bigger than it)
pub type LuaNumber = f64; // make sure this type's size matches size_luanumber (or is bigger than it)
pub type LuaInstruction = u32; // make sure this type's size matches size_instruction (or is bigger than it)

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(LuaNumber),
    String(String),
}

impl Constant {
    fn format(&self) -> String {
        match self {
            Constant::Nil => "nil".to_string(),
            Constant::Boolean(bool) => bool.to_string(),
            Constant::Number(number) => number.to_string(),
            Constant::String(s) => {
                // unparse string
                let mut result: String = String::from('"');
                let mut map: HashMap<char, char> = HashMap::new();

                map.insert('\u{0007}', 'a');
                map.insert('\u{0008}', 'b');
                map.insert('\u{000C}', 'c');
                map.insert('\n', 'n');
                map.insert('\r', 'r');
                map.insert('\t', 't');
                map.insert('\u{000B}', 'v');
                map.insert('\\', '\\');
                map.insert('\'', '\'');
                map.insert('"', '"');

                for c in s.chars() {
                    if let Some(replacement) = map.get(&c) {
                        result.push('\\');
                        result.push(replacement.clone());
                    } else {
                        result.push(c);
                    }
                }
                result.push('"');

                result
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum OpMode {
    ABC(LuaInt, LuaInt, LuaInt),
    ABX(LuaInt, LuaInt),
    ASBX(LuaInt, LuaInt),
}

const SIZE_C: LuaInt = 9;
const SIZE_B: LuaInt = 9;
const SIZE_BX: LuaInt = SIZE_C + SIZE_B;
const SIZE_A: LuaInt = 8;

const SIZE_OP: LuaInt = 6;

const POS_OP: LuaInt = 0;
const POS_A: LuaInt = POS_OP + SIZE_OP;
const POS_C: LuaInt = POS_A + SIZE_A;
const POS_B: LuaInt = POS_C + SIZE_C;
const POS_BX: LuaInt = POS_C;

macro_rules! MASK1 {
    ( $n:expr, $p:expr ) => {
        (!((!(0 as LuaInstruction)) << $n)) << $p
    };
}

macro_rules! GET_OPCODE {
    ( $i:expr ) => {
        ($i >> POS_OP) & MASK1!(SIZE_OP, 0)
    };
}
macro_rules! GET_ARGA {
    ( $i:expr ) => {
        (($i >> POS_A) & MASK1!(SIZE_A, 0)) as LuaInt
    };
}
macro_rules! GET_ARGB {
    ( $i:expr ) => {
        (($i >> POS_B) & MASK1!(SIZE_B, 0)) as LuaInt
    };
}
macro_rules! GET_ARGC {
    ( $i:expr ) => {
        (($i >> POS_C) & MASK1!(SIZE_C, 0)) as LuaInt
    };
}
macro_rules! GET_ARGBX {
    ( $i:expr ) => {
        (($i >> POS_BX) & MASK1!(SIZE_BX, 0)) as LuaInt
    };
}

/*
macro_rules! MAXARG_BX {
    ( $num_bits_int:expr, $max_int:expr ) => {
        if SIZE_BX < ($num_bits_int - 1) {
            ((1 << SIZE_BX) - 1)
        } else {
            $max_int
        }
    };
}
*/
macro_rules! MAXARG_SBX {
    ( $num_bits_int:expr, $max_int:expr ) => {
        if SIZE_BX < ($num_bits_int - 1) {
            (((1 << SIZE_BX) - 1) >> 1)
        } else {
            $max_int
        }
    };
}
macro_rules! GET_ARGASBX {
    ( $i:expr, $num_bits_int:expr, $max_int:expr ) => {
        GET_ARGBX!($i) - MAXARG_SBX!($num_bits_int, $max_int)
    };
}

const BITRK: LuaInt = 1 << (SIZE_B - 1);

macro_rules! IS_RK {
    ( $x:expr ) => {
        ($x & BITRK) != 0
    };
}
macro_rules! INDEXK {
    ( $r:expr ) => {
        $r & !BITRK
    };
}

#[derive(Debug, Clone)]
pub enum OpCode {
    OpMove(OpMode),
    OpLoadK(OpMode),
    OpLoadBool(OpMode),
    OpLoadNil(OpMode),
    OpGetUpval(OpMode),

    OpGetGlobal(OpMode),
    OpGetTable(OpMode),

    OpSetGlobal(OpMode),
    OpSetUpval(OpMode),
    OpSetTable(OpMode),

    OpNewTable(OpMode),

    OpSelf(OpMode),

    OpAdd(OpMode),
    OpSub(OpMode),
    OpMul(OpMode),
    OpDiv(OpMode),
    OpMod(OpMode),
    OpPow(OpMode),
    OpUnm(OpMode),
    OpNot(OpMode),
    OpLen(OpMode),

    OpConcat(OpMode),

    OpJmp(OpMode),

    OpEq(OpMode),
    OpLt(OpMode),
    OpLe(OpMode),

    OpTest(OpMode),
    OpTestset(OpMode),

    OpCall(OpMode),
    OpTailCall(OpMode),
    OpReturn(OpMode),

    OpForLoop(OpMode),
    OpForPrep(OpMode),

    OpTForLoop(OpMode),
    OpSetList(OpMode),

    OpClose(OpMode),
    OpClosure(OpMode),

    OpVararg(OpMode),

    OpUnknown(u32),
}

impl OpCode {
    fn describe(&self, constants: &Vec<Constant>) -> String {
        return if let OpCode::OpMove(OpMode::ABC(a, b, _c)) = self {
            format!("r_{} = r_{}", a, b)
        } else if let OpCode::OpLoadK(OpMode::ABX(a, bx)) = self {
            format!("r_{} = {}", a, constants[*bx as usize].format())
        } else if let OpCode::OpLoadBool(OpMode::ABC(a, b, c)) = self {
            format!(
                "r_{} = {}{}",
                a,
                *b == 1,
                if *c == 1 { "; pc++" } else { "" }
            )
        } else if let OpCode::OpLoadNil(OpMode::ABC(a, b, _c)) = self {
            format!("r_{}..r_{} = nil", a, b)
        } else if let OpCode::OpGetUpval(OpMode::ABC(a, b, _c)) = self {
            format!("r_{} = upvalue_{}", a, b)
        } else if let OpCode::OpGetGlobal(OpMode::ABX(a, bx)) = self {
            format!(
                "r_{} = {}",
                a,
                if let Constant::String(s) = constants[*bx as usize].clone() {
                    s
                } else {
                    panic!("Failed to format GETGLOBAL instruction: constant type was not String");
                }
            )
        } else if let OpCode::OpGetTable(OpMode::ABC(a, b, c)) = self {
            format!(
                "r_{} = r_{}[{}]",
                a,
                b,
                if IS_RK!(*c) {
                    constants[INDEXK!(*c) as usize].format()
                } else {
                    format!("r_{}", *c)
                }
            )
        } else if let OpCode::OpSetGlobal(OpMode::ABX(a, bx)) = self {
            format!(
                "{} = r_{}",
                if let Constant::String(s) = constants[*bx as usize].clone() {
                    s
                } else {
                    panic!("Failed to format SETGLOBAL instruction: constant type was not String");
                },
                a
            )
        } else {
            format!("TODO: DESCRIBE {:?}", self)
        };
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    // raw: LuaInstruction,
    op: OpCode,
}

pub fn build_instruction(
    raw: LuaInstruction,
    num_bits_int: LuaInt,
    max_int: LuaInt,
) -> Instruction {
    let a: LuaInt = GET_ARGA!(raw);
    let b: LuaInt = GET_ARGB!(raw);
    let c: LuaInt = GET_ARGC!(raw);
    let bx: LuaInt = GET_ARGBX!(raw);
    let asbx: LuaInt = GET_ARGASBX!(raw, num_bits_int, max_int);
    let op: OpCode = match GET_OPCODE!(raw) {
        0 => OpCode::OpMove(OpMode::ABC(a, b, c)),
        1 => OpCode::OpLoadK(OpMode::ABX(a, bx)),
        2 => OpCode::OpLoadBool(OpMode::ABC(a, b, c)),
        3 => OpCode::OpLoadNil(OpMode::ABC(a, b, c)),
        4 => OpCode::OpGetUpval(OpMode::ABC(a, b, c)),

        5 => OpCode::OpGetGlobal(OpMode::ABX(a, bx)),
        6 => OpCode::OpGetTable(OpMode::ABC(a, b, c)),

        7 => OpCode::OpSetGlobal(OpMode::ABX(a, bx)),

        36 => OpCode::OpClosure(OpMode::ABX(a, bx)),

        op => {
            // panic!("Failed to get opcode: {}", op);
            OpCode::OpUnknown(op)
        }
    };
    Instruction { /* raw, */ op }
}

#[derive(Debug, Clone)]
pub struct Proto {
    pub is_main: bool,
    pub id: LuaInt,

    pub source: Option<String>,
    pub line_defined: LuaInt,
    pub last_line_defined: LuaInt,
    pub upvalues_count: u8,
    pub param_count: u8,
    pub is_vararg: bool,
    pub max_stack_size: u8,

    pub code: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub protos: Vec<Proto>,
}

pub struct Bytecode {
    // header
    pub version: LuaVersion,
    pub format: u8,
    pub endianness: bool, // true means little
    pub size_int: u8,
    pub size_sizet: u8,
    pub size_instruction: u8,
    pub size_luanumber: u8,
    pub luanumber_integral: bool,

    // function
    pub main_proto: Proto,

    // printing
    pub time_taken: Duration,
    indent: u8,
}

pub fn build_bytecode(
    version: LuaVersion,
    format: u8,
    endianness: bool,
    size_int: u8,
    size_sizet: u8,
    size_instruction: u8,
    size_luanumber: u8,
    luanumber_integral: bool,
    main_proto: Proto,
    time_taken: Duration,
) -> Bytecode {
    Bytecode {
        version,
        format,
        endianness,
        size_int,
        size_sizet,
        size_instruction,
        size_luanumber,
        luanumber_integral,

        main_proto,

        time_taken,
        indent: 0,
    }
}

impl Bytecode {
    pub fn print(&mut self) {
        let time_taken = self.time_taken;
        println!("-- disassembled by inu in {}", {
            let nanoseconds: u128 = time_taken.as_nanos();
            let seconds: u128 = nanoseconds / 1e9 as u128;
            if seconds == 0 {
                let milliseconds: u128 = nanoseconds / 1e6 as u128;
                if milliseconds == 0 {
                    format!("{} microseconds", time_taken.as_micros())
                } else {
                    format!("{} milliseconds", milliseconds)
                }
            } else {
                format!("{} seconds", seconds)
            }
        });
        println!();

        self.print_proto(self.main_proto.clone());
    }

    fn print_text(&self, text: String) {
        println!("{}{}", ("    ").repeat(self.indent.into()), text);
    }

    fn print_proto(&mut self, proto: Proto) {
        if !proto.is_main {
            self.print_text(format!("local function proto_{}()", proto.id));
            self.indent += 1;
        }

        let mut was_proto_printed_map: Vec<bool> = vec![false; proto.code.len()];
        for i in 0..proto.code.len() {
            let inst: &Instruction = &proto.code[i];
            if let OpCode::OpClosure(OpMode::ABX(_, bx)) = inst.op.clone() {
                let index: usize = bx as usize;
                if !was_proto_printed_map[index] {
                    was_proto_printed_map[index] = true;
                    self.print_proto(proto.protos[index].clone());
                }
            }
            // TODO: proper formatting (certain # of spaces)
            self.print_text(format!(
                "{}    {:?}    -- {}",
                i,
                inst.op,
                inst.op.describe(&proto.constants)
            ));
        }

        for i in 0..proto.protos.len() {
            let proto: Proto = proto.protos[i].clone();
            if !was_proto_printed_map[i] {
                self.print_proto(proto);
            }
        }

        if !proto.is_main {
            self.indent -= 1;
            self.print_text("end".to_string())
        }
    }
}
