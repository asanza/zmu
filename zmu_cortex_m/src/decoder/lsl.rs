use core::bits::*;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_LSL_reg_t1(command: u16) -> Instruction {
    Instruction::LSL_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}