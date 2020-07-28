use crate::core::bits::Bits;
use crate::core::instruction::{Reg2VanillaParams, Instruction};

#[allow(non_snake_case)]
pub fn decode_TBH_t1(opcode: u32) -> Instruction {
    Instruction::TBH {
        params: Reg2VanillaParams {
            rn: opcode.get_bits(16..20).into(),
            rm: opcode.get_bits(0..4).into(),
        },
    }
}
