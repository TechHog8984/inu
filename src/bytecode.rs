use std::{collections::HashMap, str::from_utf8, time::Duration};

use crate::util::util::format_time_taken;

#[derive(Debug)]
pub enum LuaVersion {
    Lua51,
}

pub type LuaInt = i32; // make sure this type's size matches size_luaint (or is bigger than it)
pub type LuaUint = u32;
pub type LuaNumber = f64; // make sure this type's size matches size_luanumber (or is bigger than it)
pub type LuaInstruction = u32; // make sure this type's size matches size_instruction (or is bigger than it)

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(LuaNumber),
    String(Vec<u8>),
}

impl Constant {
    pub fn format(&self) -> String {
        match self {
            Constant::Nil => "nil".to_string(),
            Constant::Boolean(bool) => bool.to_string(),
            Constant::Number(number) => number.to_string(),
            Constant::String(bytes) => {
                // unparse string
                let mut result: String = String::from('"');
                let mut map: HashMap<u8, char> = HashMap::new();

                map.insert('\u{0007}' as u8, 'a');
                map.insert('\u{0008}' as u8, 'b');
                map.insert('\u{000C}' as u8, 'c');
                map.insert('\n' as u8, 'n');
                map.insert('\r' as u8, 'r');
                map.insert('\t' as u8, 't');
                map.insert('\u{000B}' as u8, 'v');
                map.insert('\\' as u8, '\\');
                map.insert('\'' as u8, '\'');
                map.insert('"' as u8, '"');
                map.insert('\0' as u8, '0');
                // map.insert('\1', '1');

                let mut i = 0;
                while i < bytes.len() {
                    let b = bytes[i];
                    if let Some(replacement) = map.get(&b) {
                        result.push('\\');
                        result.push(replacement.clone());
                    } else if let Ok(c) = from_utf8(&bytes[i..i + 1]) {
                        result.push_str(c);
                    } else {
                        result.push_str(&format!("\\x{:02x}", b));
                    }
                    i += 1;
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
    OpTestSet(OpMode),

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

macro_rules! GET_RAW_CONSTANT_AND_EXPECT_STRING {
    ( $constants:expr, $index:expr, $instruction_name:expr ) => {
        if let Constant::String(s) = $constants[$index].clone() {
            s
        } else {
            panic!(
                "Failed to format {} instruction: constant type was not String",
                $instruction_name
            );
        }
    };
}

macro_rules! REG_FMT {
    ( $reg:expr, $proto:expr ) => {
        match $reg < $proto.param_count as i32 {
            true => format!("arg_{}", $reg),
            false => format!("r_{}", $reg),
        }
    };
}

macro_rules! FORMAT_CONSTANT_RK {
    ( $constants:expr, $value:expr, $proto:expr ) => {
        if IS_RK!($value) {
            $constants[INDEXK!($value) as usize].format()
        } else {
            REG_FMT!($value, $proto)
        }
    };
}
macro_rules! FORMAT_BINARY {
    ( $op:expr, $constants:expr, $proto:expr, $a:expr, $b:expr, $c:expr ) => {
        format!(
            "{} = {} {} {}",
            REG_FMT!($a, $proto),
            FORMAT_CONSTANT_RK!($constants, $b, $proto),
            $op,
            FORMAT_CONSTANT_RK!($constants, $c, $proto)
        )
    };
}
macro_rules! FORMAT_UNARY {
    ( $op:expr, $proto:expr, $a:expr, $b:expr ) => {
        format!("{} = {}{}", REG_FMT!($a, $proto), $op, REG_FMT!($b, $proto))
    };
}

macro_rules! FORMAT_CONDITION {
    ( $yes:expr, $no:expr, $constants:expr, $proto:expr, $a:expr, $b:expr, $c:expr, $pc:expr ) => {
        format!(
            "if {} {} {} then goto {}",
            FORMAT_CONSTANT_RK!($constants, $b, $proto),
            if $a == 1 { $no } else { $yes },
            FORMAT_CONSTANT_RK!($constants, $c, $proto),
            $pc + 2
        )
    };
}

macro_rules! SIMPLE_REG_LIST {
    ( $from:expr, $to:expr, $proto:expr ) => {
        match $from == $to {
            true => REG_FMT!($from, $proto),
            false => format!("r_{} ... r_{}", $from, $to),
        }
    };
}

impl OpCode {
    fn describe(
        &self,
        constants: &Vec<Constant>,
        proto: &Proto,
        protos: &Vec<Proto>,
        pc: isize,
    ) -> String {
        return if let OpCode::OpMove(OpMode::ABC(a, b, _c)) = self {
            format!("{} = {}", REG_FMT!(*a, proto), REG_FMT!(*b, proto))
        } else if let OpCode::OpLoadK(OpMode::ABX(a, bx)) = self {
            format!(
                "{} = {}",
                REG_FMT!(*a, proto),
                constants[*bx as usize].format()
            )
        } else if let OpCode::OpLoadBool(OpMode::ABC(a, b, c)) = self {
            format!(
                "{} = {}{}",
                REG_FMT!(*a, proto),
                *b == 1,
                if *c == 1 {
                    format!("; goto {}", pc + 2)
                } else {
                    "".to_string()
                }
            )
        } else if let OpCode::OpLoadNil(OpMode::ABC(a, b, _c)) = self {
            format!("{} = nil", SIMPLE_REG_LIST!(*a, *b, proto))
        } else if let OpCode::OpGetUpval(OpMode::ABC(a, b, _c)) = self {
            format!("{} = upvalue_{}", REG_FMT!(*a, proto), b)
        } else if let OpCode::OpGetGlobal(OpMode::ABX(a, bx)) = self {
            format!(
                "{} = {}",
                REG_FMT!(*a, proto),
                String::from_utf8(GET_RAW_CONSTANT_AND_EXPECT_STRING!(
                    constants,
                    *bx as usize,
                    "GETGLOBAL"
                ))
                .unwrap_or("[INVALID STRING]".to_string())
            )
        } else if let OpCode::OpGetTable(OpMode::ABC(a, b, c)) = self {
            format!(
                "{} = {}[{}]",
                REG_FMT!(*a, proto),
                REG_FMT!(*b, proto),
                FORMAT_CONSTANT_RK!(constants, *c, proto)
            )
        } else if let OpCode::OpSetGlobal(OpMode::ABX(a, bx)) = self {
            format!(
                "{} = {}",
                String::from_utf8(GET_RAW_CONSTANT_AND_EXPECT_STRING!(
                    constants,
                    *bx as usize,
                    "SETGLOBAL"
                ))
                .unwrap_or("[INVALID STRING]".to_string()),
                REG_FMT!(*a, proto)
            )
        } else if let OpCode::OpSetUpval(OpMode::ABC(a, b, _c)) = self {
            format!("upvalue_{} = {}", b, REG_FMT!(*a, proto))
        } else if let OpCode::OpSetTable(OpMode::ABC(a, b, c)) = self {
            format!(
                "{}[{}] = {}",
                REG_FMT!(*a, proto),
                FORMAT_CONSTANT_RK!(constants, *b, proto),
                FORMAT_CONSTANT_RK!(constants, *c, proto)
            )
        } else if let OpCode::OpNewTable(OpMode::ABC(a, b, c)) = self {
            format!("{} = {{}} -- {} list, {} record", REG_FMT!(*a, proto), b, c)
        } else if let OpCode::OpSelf(OpMode::ABC(a, b, c)) = self {
            format!(
                "{} = {}; {} = {}[{}]",
                REG_FMT!(a + 1, proto),
                REG_FMT!(*b, proto),
                REG_FMT!(*a, proto),
                REG_FMT!(*b, proto),
                FORMAT_CONSTANT_RK!(constants, *c, proto)
            )
        } else if let OpCode::OpAdd(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('+', constants, proto, *a, *b, *c)
        } else if let OpCode::OpSub(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('-', constants, proto, *a, *b, *c)
        } else if let OpCode::OpMul(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('*', constants, proto, *a, *b, *c)
        } else if let OpCode::OpDiv(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('/', constants, proto, *a, *b, *c)
        } else if let OpCode::OpMod(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('%', constants, proto, *a, *b, *c)
        } else if let OpCode::OpPow(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('^', constants, proto, *a, *b, *c)
        } else if let OpCode::OpUnm(OpMode::ABC(a, b, _c)) = self {
            FORMAT_UNARY!('-', proto, *a, *b)
        } else if let OpCode::OpNot(OpMode::ABC(a, b, _c)) = self {
            FORMAT_UNARY!("not ", proto, *a, *b)
        } else if let OpCode::OpLen(OpMode::ABC(a, b, _c)) = self {
            FORMAT_UNARY!('#', proto, *a, *b)
        } else if let OpCode::OpConcat(OpMode::ABC(a, b, c)) = self {
            format!(
                "{} = {} .. ... .. {}",
                REG_FMT!(*a, proto),
                REG_FMT!(*b, proto),
                REG_FMT!(*c, proto)
            )
        } else if let OpCode::OpJmp(OpMode::ASBX(_a, sbx)) = self {
            format!("goto {}", pc + *sbx as isize + 1)
        } else if let OpCode::OpEq(OpMode::ABC(a, b, c)) = self {
            FORMAT_CONDITION!("==", "~=", constants, proto, *a, *b, *c, pc)
        } else if let OpCode::OpLt(OpMode::ABC(a, b, c)) = self {
            FORMAT_CONDITION!("<", ">=", constants, proto, *a, *b, *c, pc)
        } else if let OpCode::OpLe(OpMode::ABC(a, b, c)) = self {
            FORMAT_CONDITION!("<=", ">", constants, proto, *a, *b, *c, pc)
        } else if let OpCode::OpTest(OpMode::ABC(a, _b, c)) = self {
            format!(
                "if {}{} then goto {}",
                if *c == 0 { "" } else { "not " },
                REG_FMT!(*a, proto),
                pc + 2
            )
        } else if let OpCode::OpTestSet(OpMode::ABC(a, b, c)) = self {
            format!(
                "if {}{} then goto {} else {} = {}",
                if *c == 0 { "" } else { "not " },
                b,
                REG_FMT!(pc as i32 + 2, proto),
                REG_FMT!(*a, proto),
                REG_FMT!(*b, proto)
            )
        } else if let OpCode::OpCall(OpMode::ABC(a, b, c)) = self {
            format!(
                "{}{}({})",
                if *c == 0 {
                    String::from("top ... ??? = ")
                } else if *c == 1 {
                    String::new()
                } else {
                    format!("{} = ", SIMPLE_REG_LIST!(*a, *a + *c - 2, proto))
                },
                REG_FMT!(*a, proto),
                if *b == 0 {
                    String::from("top ... ???")
                } else if *b == 1 {
                    String::new()
                } else {
                    SIMPLE_REG_LIST!(*a + 1, *a + *b - 1, proto)
                }
            )
        } else if let OpCode::OpTailCall(OpMode::ABC(a, b, _c)) = self {
            format!(
                "return {}({})",
                REG_FMT!(*a, proto),
                if *b == 0 {
                    String::from("top ... ???")
                } else if *b == 1 {
                    String::new()
                } else {
                    SIMPLE_REG_LIST!(*a + 1, *a + *b - 1, proto)
                }
            )
        } else if let OpCode::OpReturn(OpMode::ABC(a, b, _c)) = self {
            format!(
                "return{}",
                if *b == 0 {
                    String::from(" top ... ???")
                } else if *b == 1 {
                    String::new()
                } else {
                    format!(" {}", SIMPLE_REG_LIST!(*a, *a + *b - 2, proto))
                }
            )
        } else if let OpCode::OpForLoop(OpMode::ASBX(a, sbx)) = self {
            format!(
                "{} += {}; if {} <?= {} then {{ goto {}; {} = {} }}",
                REG_FMT!(*a, proto),
                REG_FMT!(*a + 2, proto),
                REG_FMT!(*a, proto),
                REG_FMT!(*a + 1, proto),
                pc + *sbx as isize + 2,
                REG_FMT!(*a + 3, proto),
                REG_FMT!(*a, proto)
            )
        } else if let OpCode::OpForPrep(OpMode::ASBX(a, sbx)) = self {
            format!(
                "{} -= {}; goto {}",
                REG_FMT!(*a, proto),
                REG_FMT!(*a + 2, proto),
                pc + *sbx as isize + 2
            )
        } else if let OpCode::OpTForLoop(OpMode::ABC(a, _b, c)) = self {
            format!(
                "{} = {}({}, {}); if {} ~= nil {{{} = {}}} else goto {}",
                SIMPLE_REG_LIST!(*a + 3, *a + 2 + *c, proto),
                *a,
                REG_FMT!(*a + 1, proto),
                REG_FMT!(*a + 2, proto),
                REG_FMT!(*a + 3, proto),
                REG_FMT!(*a + 2, proto),
                REG_FMT!(*a + 3, proto),
                pc + 2
            )
        } else if let OpCode::OpSetList(OpMode::ABC(a, b, c)) = self {
            let offset = (*c - 1) * 50;
            format!(
                "{}[{} ... {}] = {} ... {}",
                REG_FMT!(*a, proto),
                offset + 1,
                offset + *b,
                REG_FMT!(*a + 1, proto),
                REG_FMT!(*a + *b, proto)
            )
        } else if let OpCode::OpClose(OpMode::ABX(a, _bx)) = self {
            format!("close all variables in the stack up to r_{}", a)
        } else if let OpCode::OpClosure(OpMode::ABX(a, bx)) = self {
            let upvalue_count: u8 = protos[*bx as usize].upvalue_count;
            let multiple: bool = upvalue_count > 1;
            format!(
                "{} = proto_{}{}",
                REG_FMT!(*a, proto),
                bx,
                if upvalue_count > 0 {
                    format!(
                        " -- the {} proceeding getupval or move instruction{} upvalue{}",
                        upvalue_count,
                        match multiple {
                            true => "s are",
                            false => " is an",
                        },
                        match multiple {
                            true => "s",
                            false => "",
                        }
                    )
                } else {
                    String::new()
                }
            )
        } else if let OpCode::OpVararg(OpMode::ABC(a, b, _c)) = self {
            format!(
                "{} = vararg",
                if *b == 0 {
                    format!("{}, top ... ???", REG_FMT!(*a, proto))
                } else if *b == 1 {
                    format!("{}", REG_FMT!(*a, proto))
                } else {
                    format!("{}", SIMPLE_REG_LIST!(*a, *a + *b - 2, proto))
                }
            )
        } else {
            format!("TODO: DESCRIBE {:?}", self)
        };
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    // raw: LuaInstruction,
    pub op: OpCode,
}

impl Instruction {
    pub fn handle_aux(&mut self, aux: Option<&LuaInstruction>) -> bool {
        if let OpCode::OpSetList(OpMode::ABC(a, b, c)) = &self.op {
            if *c != 0 {
                return false;
            }
            self.op = OpCode::OpSetList(OpMode::ABC(
                *a,
                *b,
                *aux.expect("failed to get aux for SetList with C of 0") as i32,
            ));
            return true;
        }
        return false;
    }
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
    let sbx: LuaInt = GET_ARGASBX!(raw, num_bits_int, max_int);
    let op: OpCode = match GET_OPCODE!(raw) {
        0 => OpCode::OpMove(OpMode::ABC(a, b, c)),
        1 => OpCode::OpLoadK(OpMode::ABX(a, bx)),
        2 => OpCode::OpLoadBool(OpMode::ABC(a, b, c)),
        3 => OpCode::OpLoadNil(OpMode::ABC(a, b, c)),
        4 => OpCode::OpGetUpval(OpMode::ABC(a, b, c)),

        5 => OpCode::OpGetGlobal(OpMode::ABX(a, bx)),
        6 => OpCode::OpGetTable(OpMode::ABC(a, b, c)),

        7 => OpCode::OpSetGlobal(OpMode::ABX(a, bx)),
        8 => OpCode::OpSetUpval(OpMode::ABC(a, b, c)),
        9 => OpCode::OpSetTable(OpMode::ABC(a, b, c)),

        10 => OpCode::OpNewTable(OpMode::ABC(a, b, c)),

        11 => OpCode::OpSelf(OpMode::ABC(a, b, c)),

        12 => OpCode::OpAdd(OpMode::ABC(a, b, c)),
        13 => OpCode::OpSub(OpMode::ABC(a, b, c)),
        14 => OpCode::OpMul(OpMode::ABC(a, b, c)),
        15 => OpCode::OpDiv(OpMode::ABC(a, b, c)),
        16 => OpCode::OpMod(OpMode::ABC(a, b, c)),
        17 => OpCode::OpPow(OpMode::ABC(a, b, c)),
        18 => OpCode::OpUnm(OpMode::ABC(a, b, c)),
        19 => OpCode::OpNot(OpMode::ABC(a, b, c)),
        20 => OpCode::OpLen(OpMode::ABC(a, b, c)),

        21 => OpCode::OpConcat(OpMode::ABC(a, b, c)),

        22 => OpCode::OpJmp(OpMode::ASBX(0, sbx)),

        23 => OpCode::OpEq(OpMode::ABC(a, b, c)),
        24 => OpCode::OpLt(OpMode::ABC(a, b, c)),
        25 => OpCode::OpLe(OpMode::ABC(a, b, c)),

        26 => OpCode::OpTest(OpMode::ABC(a, b, c)),
        27 => OpCode::OpTestSet(OpMode::ABC(a, b, c)),

        28 => OpCode::OpCall(OpMode::ABC(a, b, c)),
        29 => OpCode::OpTailCall(OpMode::ABC(a, b, c)),
        30 => OpCode::OpReturn(OpMode::ABC(a, b, c)),

        31 => OpCode::OpForLoop(OpMode::ASBX(a, sbx)),
        32 => OpCode::OpForPrep(OpMode::ASBX(a, sbx)),

        33 => OpCode::OpTForLoop(OpMode::ABC(a, b, c)),
        34 => OpCode::OpSetList(OpMode::ABC(a, b, c)),

        35 => OpCode::OpClose(OpMode::ABX(a, bx)),
        36 => OpCode::OpClosure(OpMode::ABX(a, bx)),

        37 => OpCode::OpVararg(OpMode::ABC(a, b, c)),

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

    pub source: Vec<u8>,
    pub line_defined: LuaInt,
    pub last_line_defined: LuaInt,
    pub upvalue_count: u8,
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
    pub fn print_disassembly(&mut self, just_describes: bool) {
        let time_taken = self.time_taken;
        println!(
            "-- disassembled by inu in {}\n",
            format_time_taken!(time_taken)
        );

        self.print_proto(self.main_proto.clone(), just_describes);
    }

    fn print_text(&self, text: String) {
        println!("{}{}", ("    ").repeat(self.indent.into()), text);
    }
    fn print_text_str(&self, text: &str) {
        self.print_text(text.to_string());
    }

    fn print_proto(&mut self, proto: Proto, just_describes: bool) {
        if !proto.is_main {
            self.print_text(format!("local function proto_{}({})", proto.id, {
                let count = proto.param_count;
                let vararg = proto.is_vararg;
                let mut arg_str: String = String::new();

                for i in 0..count {
                    arg_str.push_str(format!("arg_{}, ", i).as_str());
                }
                if vararg {
                    arg_str += "...";
                } else if count > 0 {
                    arg_str.truncate(arg_str.len() - 2);
                }

                arg_str
            }));
            self.indent += 1;
        }

        self.print_text_str("--[[ constants:");
        self.indent += 1;
        for i in 0..proto.constants.len() {
            let kst: &Constant = &proto.constants[i];

            let mut i_str = i.to_string();
            i_str.push_str(" - ");

            let mut str = kst.format();
            str.insert_str(0, i_str.as_str());
            self.print_text(str);
        }
        self.indent -= 1;
        self.print_text_str("]]");

        let mut was_proto_printed_map: Vec<bool> = vec![false; proto.code.len()];

        let code_len: usize = proto.code.len();
        let mut code_op_strings: Vec<String> = Vec::with_capacity(code_len);
        let mut code_op_describes: Vec<String> = Vec::with_capacity(code_len);

        for i in 0..code_len {
            let inst: &Instruction = &proto.code[i];

            code_op_strings.push(format!("{:?}", inst.op));
            code_op_describes.push(inst.op.describe(
                &proto.constants,
                &proto,
                &proto.protos,
                i as isize,
            ));
        }

        let max_index_width = if code_len == 0 {
            1
        } else {
            (code_len as f64).log(10.0).floor() as usize + 1
        } + 2;
        let max_op_strings_width = code_op_strings
            .iter()
            .map(|string| string.len())
            .max()
            .unwrap_or(0);

        for i in 0..code_len {
            let inst: &Instruction = &proto.code[i];
            if let OpCode::OpClosure(OpMode::ABX(_, bx)) = inst.op.clone() {
                let index: usize = bx as usize;
                if !was_proto_printed_map[index] {
                    was_proto_printed_map[index] = true;
                    self.print_proto(proto.protos[index].clone(), just_describes);
                }
            }
            if just_describes {
                self.print_text(code_op_describes[i].clone());
            } else {
                self.print_text(format!(
                    "{:<width_index$}{:<width_strings$}  --  {}",
                    i,
                    code_op_strings[i],
                    code_op_describes[i],
                    width_index = max_index_width,
                    width_strings = max_op_strings_width,
                ))
            }
        }

        for i in 0..proto.protos.len() {
            let proto: Proto = proto.protos[i].clone();
            if !was_proto_printed_map[i] {
                self.print_proto(proto, just_describes);
            }
        }

        if !proto.is_main {
            self.indent -= 1;
            self.print_text_str("end")
        }
    }
}
