use core::bits::*;
use core::instruction::Instruction;
use core::operation::zero_extend;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_imm_t1(command: u16) -> Instruction {
    Instruction::MOV_imm {
        rd: Reg::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_reg_t1(command: u16) -> Instruction {
    Instruction::MOV_reg {
        rd: Reg::from((bit_7(command) << 3) + bits_0_3(command)),
        rm: Reg::from(bits_3_7(command)),
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_MOV_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_reg_t2_LSL_imm_t1(command: u16) -> Instruction {
    let imm5 = bits_6_11(command) as u8;

    if imm5 == 0 {
        Instruction::MOV_reg {
            rd: Reg::from(bits_0_3(command)),
            rm: Reg::from(bits_3_6(command)),
            setflags: true,
        }
    } else {
        Instruction::LSL_imm {
            rd: Reg::from(bits_0_3(command)),
            rm: Reg::from(bits_3_6(command)),
            imm5: imm5,
            setflags: true,
        }
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_imm_t2(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_imm_t3(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8, 11);
    let _imm4: u8 = opcode.get_bits(16, 19);
    let _imm3: u8 = opcode.get_bits(12, 14);
    let _imm8: u8 = opcode.get_bits(0, 7);
    let _i: u8 = opcode.get_bits(26, 26);

    Instruction::MOV_imm {
        rd: Reg::from(rd),
        imm32: zero_extend(/*{imm4,4}, {i, 1}, {imm3, 3}, imm8,8, 32 */),
        setflags: false,
    }
}
