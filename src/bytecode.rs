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
macro_rules! FORMAT_CONSTANT_RK {
    ( $constants:expr, $value:expr ) => {
        if IS_RK!($value) {
            $constants[INDEXK!($value) as usize].format()
        } else {
            format!("r_{}", $value)
        }
    };
}
macro_rules! FORMAT_BINARY {
    ( $op:expr, $constants:expr, $a:expr, $b:expr, $c:expr ) => {
        format!(
            "r_{} = {} {} {}",
            $a,
            FORMAT_CONSTANT_RK!($constants, $b),
            $op,
            FORMAT_CONSTANT_RK!($constants, $c)
        )
    };
}
macro_rules! FORMAT_UNARY {
    ( $op:expr, $a:expr, $b:expr ) => {
        format!("r_{} = {}r_{}", $a, $op, $b)
    };
}

macro_rules! FORMAT_CONDITION {
    ( $yes:expr, $no:expr, $constants:expr, $a:expr, $b:expr, $c:expr, $pc:expr ) => {
        format!(
            "if {} {} {} then goto {}",
            FORMAT_CONSTANT_RK!($constants, $b),
            if $a == 1 { $no } else { $yes },
            FORMAT_CONSTANT_RK!($constants, $c),
            $pc + 2
        )
    };
}

macro_rules! SIMPLE_REG_LIST {
    ( $from:expr, $to:expr ) => {
        match $from == $to {
            true => format!("r_{}", $from),
            false => format!("r_{} ... r_{}", $from, $to),
        }
    };
}

impl OpCode {
    fn describe(&self, constants: &Vec<Constant>, pc: isize) -> String {
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
            format!("{} = nil", SIMPLE_REG_LIST!(*a, *b))
        } else if let OpCode::OpGetUpval(OpMode::ABC(a, b, _c)) = self {
            format!("r_{} = upvalue_{}", a, b)
        } else if let OpCode::OpGetGlobal(OpMode::ABX(a, bx)) = self {
            format!(
                "r_{} = {}",
                a,
                GET_RAW_CONSTANT_AND_EXPECT_STRING!(constants, *bx as usize, "GETGLOBAL")
            )
        } else if let OpCode::OpGetTable(OpMode::ABC(a, b, c)) = self {
            format!("r_{} = r_{}[{}]", a, b, FORMAT_CONSTANT_RK!(constants, *c))
        } else if let OpCode::OpSetGlobal(OpMode::ABX(a, bx)) = self {
            format!(
                "{} = r_{}",
                GET_RAW_CONSTANT_AND_EXPECT_STRING!(constants, *bx as usize, "SETGLOBAL"),
                a
            )
        } else if let OpCode::OpSetUpval(OpMode::ABC(a, b, _c)) = self {
            format!("upvalue_{} = r_{}", b, a)
        } else if let OpCode::OpSetTable(OpMode::ABC(a, b, c)) = self {
            format!(
                "r_{}[{}] = {}",
                a,
                FORMAT_CONSTANT_RK!(constants, b),
                FORMAT_CONSTANT_RK!(constants, c)
            )
        } else if let OpCode::OpNewTable(OpMode::ABC(a, b, c)) = self {
            format!("r_{} = {{}} -- {} list & {} record", a, b, c)
        } else if let OpCode::OpSelf(OpMode::ABC(a, b, c)) = self {
            format!(
                "r_{} = r_{}; r_{} = r_{}[{}]",
                a + 1,
                b,
                a,
                b,
                FORMAT_CONSTANT_RK!(constants, c)
            )
        } else if let OpCode::OpAdd(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('+', constants, *a, *b, *c)
        } else if let OpCode::OpSub(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('-', constants, *a, *b, *c)
        } else if let OpCode::OpMul(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('*', constants, *a, *b, *c)
        } else if let OpCode::OpDiv(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('/', constants, *a, *b, *c)
        } else if let OpCode::OpMod(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('%', constants, *a, *b, *c)
        } else if let OpCode::OpPow(OpMode::ABC(a, b, c)) = self {
            FORMAT_BINARY!('^', constants, *a, *b, *c)
        } else if let OpCode::OpUnm(OpMode::ABC(a, b, _c)) = self {
            FORMAT_UNARY!('-', *a, *b)
        } else if let OpCode::OpNot(OpMode::ABC(a, b, _c)) = self {
            FORMAT_UNARY!("not ", *a, *b)
        } else if let OpCode::OpLen(OpMode::ABC(a, b, _c)) = self {
            FORMAT_UNARY!('#', *a, *b)
        } else if let OpCode::OpConcat(OpMode::ABC(a, b, c)) = self {
            format!("r_{} = r_{} .. ... .. r_{}", a, b, c)
        } else if let OpCode::OpJmp(OpMode::ASBX(_a, sbx)) = self {
            format!("goto {}", pc + *sbx as isize + 1)
        } else if let OpCode::OpEq(OpMode::ABC(a, b, c)) = self {
            FORMAT_CONDITION!("==", "~=", constants, *a, *b, *c, pc)
        } else if let OpCode::OpLt(OpMode::ABC(a, b, c)) = self {
            FORMAT_CONDITION!("<", ">=", constants, *a, *b, *c, pc)
        } else if let OpCode::OpLe(OpMode::ABC(a, b, c)) = self {
            FORMAT_CONDITION!("<=", ">", constants, *a, *b, *c, pc)
        } else if let OpCode::OpTest(OpMode::ABC(a, _b, c)) = self {
            format!(
                "if {}r_{} then goto {}",
                if *c == 0 { "" } else { "not " },
                a,
                pc + 2
            )
        } else if let OpCode::OpTestSet(OpMode::ABC(a, b, c)) = self {
            format!(
                "if {}r_{} then goto {} else r_{} = r_{}",
                if *c == 0 { "" } else { "not " },
                b,
                pc + 2,
                a,
                b
            )
        } else if let OpCode::OpCall(OpMode::ABC(a, b, c)) = self {
            format!(
                "{}r_{}({})",
                if *c == 0 {
                    String::from("top ... ??? = ")
                } else if *c == 1 {
                    String::new()
                } else {
                    format!("{} = ", SIMPLE_REG_LIST!(*a, *a + *c - 2))
                },
                a,
                if *b == 0 {
                    String::from("top ... ???")
                } else if *b == 1 {
                    String::new()
                } else {
                    SIMPLE_REG_LIST!(*a + 1, *a + *b - 1)
                }
            )
        } else if let OpCode::OpTailCall(OpMode::ABC(a, b, _c)) = self {
            format!(
                "return r_{}({})",
                a,
                if *b == 0 {
                    String::from("top ... ???")
                } else if *b == 1 {
                    String::new()
                } else {
                    SIMPLE_REG_LIST!(*a + 1, *a + *b - 1)
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
                    format!(" {}", SIMPLE_REG_LIST!(*a, *a + *b - 2))
                }
            )
        } else if let OpCode::OpForLoop(OpMode::ASBX(a, sbx)) = self {
            format!(
                "r_{} += r_{}; if r_{} <?= r_{} then {{ goto {}; r_{} = r_{} }}",
                a,
                *a + 2,
                a,
                *a + 1,
                pc + *sbx as isize + 2,
                *a + 3,
                a
            )
        } else if let OpCode::OpForPrep(OpMode::ASBX(a, sbx)) = self {
            format!("r_{} -= r_{}; goto {}", a, *a + 2, pc + *sbx as isize + 2)
        } else if let OpCode::OpTForLoop(OpMode::ABC(a, _b, c)) = self {
            format!(
                "{} = r_{}(r_{}, r_{}); if r_{} ~= nil {{r_{} = r_{}}} else goto {}",
                SIMPLE_REG_LIST!(*a + 3, *a + 2 + *c),
                *a,
                *a + 1,
                *a + 2,
                *a + 3,
                *a + 2,
                *a + 3,
                pc + 2
            )
        } else if let OpCode::OpSetList(OpMode::ABC(a, b, c)) = self {
            let offset = (*c - 1) * 50;
            format!(
                "r_{}[{} ... {}] = r_{} ... r_{}",
                a,
                offset + 1,
                offset + *b,
                *a + 1,
                *a + *b
            )
        } else if let OpCode::OpClose(OpMode::ABX(a, _bx)) = self {
            format!("close all variables in the stack up to r_{}", a)
        } else if let OpCode::OpClosure(OpMode::ABX(a, bx)) = self {
            format!(
                "r_{} = proto_{} -- [num upvalues] proceeding getupval or move instructions are upvalues",
                a, bx,
            )
        } else if let OpCode::OpVararg(OpMode::ABC(a, b, _c)) = self {
            format!(
                "r_{}{} = vararg",
                a,
                if *b == 0 {
                    String::from(", top ... ???")
                } else if *b == 1 {
                    String::new()
                } else {
                    format!(", {}", SIMPLE_REG_LIST!(*a, *a + *b - 2))
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
            panic!("Failed to get opcode: {}", op);
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

        let code_len: usize = proto.code.len();
        let mut code_op_strings: Vec<String> = Vec::with_capacity(code_len);
        let mut code_op_describes: Vec<String> = Vec::with_capacity(code_len);

        for i in 0..code_len {
            let inst: &Instruction = &proto.code[i];

            code_op_strings.push(format!("{:?}", inst.op));
            code_op_describes.push(inst.op.describe(&proto.constants, i as isize));
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
        let max_op_describes_width = code_op_describes
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
                    self.print_proto(proto.protos[index].clone());
                }
            }
            self.print_text(format!(
                "{:<width_index$}{:<width_strings$}  --  {:<width_describes$}",
                i,
                code_op_strings.get(i).unwrap(),
                code_op_describes.get(i).unwrap(),
                width_index = max_index_width,
                width_strings = max_op_strings_width,
                width_describes = max_op_describes_width
            ))
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
