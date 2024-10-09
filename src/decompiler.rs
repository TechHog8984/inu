use std::time::Instant;

use crate::{bytecode::Bytecode, util::util::format_time_taken};

pub struct Decompiler {
    bytecode: Bytecode,
}
pub fn build_decompiler(bytecode: Bytecode) -> Decompiler {
    Decompiler { bytecode }
}

impl Decompiler {
    pub fn decompile(&mut self) -> String {
        let start_instant: Instant = Instant::now();

        let time_taken = start_instant.elapsed();

        format!("-- decompiled by inu in {}", format_time_taken!(time_taken))
    }
}
