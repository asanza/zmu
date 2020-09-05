use crate::core::bits::Bits;
use crate::core::instruction::Imm32Carry;
use crate::core::instruction::Instruction;
use crate::core::instruction::{Reg2ImmCarryParams, Reg3ShiftParams, SRType, SetFlags};
use crate::core::operation::decode_imm_shift;
use crate::core::operation::thumb_expand_imm_c;
use crate::core::register::Reg;

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_ORR_reg_t1(opcode: u16) -> Instruction {
    Instruction::ORR_reg {
        params: Reg3ShiftParams {
            rd: Reg::from(opcode.get_bits(0..3) as u8),
            rn: Reg::from(opcode.get_bits(0..3) as u8),
            rm: Reg::from(opcode.get_bits(3..6) as u8),
            setflags: SetFlags::NotInITBlock,
            shift_t: SRType::LSL,
            shift_n: 0,
        },
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_ORR_reg_t2(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s = opcode.get_bit(20);

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;
    let type_: u8 = opcode.get_bits(4..6) as u8;

    let (shift_t, shift_n) = decode_imm_shift(type_, (imm3 << 2) + imm2);

    Instruction::ORR_reg {
        params: Reg3ShiftParams {
            rd: Reg::from(rd),
            rn: Reg::from(rn),
            rm: Reg::from(rm),
            setflags: if s { SetFlags::True } else { SetFlags::False },
            shift_t,
            shift_n,
        },
        thumb32: true,
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_ORR_imm_t1(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let rn: u8 = opcode.get_bits(16..20) as u8;

    let s: u8 = opcode.get_bit(20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::ORR_imm {
        params: Reg2ImmCarryParams {
            rd: Reg::from(rd),
            rn: Reg::from(rn),
            imm32: Imm32Carry::Carry {
                imm32_c0: thumb_expand_imm_c(&params, &lengths, false),
                imm32_c1: thumb_expand_imm_c(&params, &lengths, true),
            },
            setflags: s == 1,
        },
    }
}
