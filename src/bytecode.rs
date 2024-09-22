#[derive(Debug)]
pub enum LuaVersion {
    Lua51,
}

pub type LuaInt = i32; // make sure this type's size matches size_luaint (or is bigger than it)
pub type LuaNumber = f64; // make sure this type's size matches size_luanumber (or is bigger than it)

#[derive(Debug)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(LuaNumber),
    String(String),
}

#[derive(Debug)]
pub struct Proto {
    pub source: Option<String>,
    pub line_defined: LuaInt,
    pub last_line_defined: LuaInt,
    pub upvalues_count: u8,
    pub param_count: u8,
    pub is_vararg: bool,
    pub max_stack_size: u8,

    pub code: Vec<u32>,
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
}
