use crate::core::instruction::{
    AddressingMode, BfcParams, BfiParams, BfxParams, CondBranchParams, Imm32Carry, MovtParams,
    ParamsRegImm32, Reg2DoubleParams, Reg2FullParams, Reg2ImmCarryParams, Reg2ImmParams,
    Reg2Params, Reg2RdRmParams, Reg2RnRmParams, Reg2RtRnImm32Params, Reg2ShiftNParams,
    Reg2ShiftNoSetFlagsParams, Reg2ShiftParams, Reg2UsizeParams, Reg3FullParams, Reg3HighParams,
    Reg3NoSetFlagsParams, Reg3Params, Reg3RdRtRnImm32Params, Reg3ShiftParams, Reg3UsizeParams,
    Reg4HighParams, Reg4NoSetFlagsParams, Reg643232Params, RegImm32AddParams,
    RegImmCarryNoSetFlagsParams, RegImmCarryParams, RegImmParams, SRType, SetFlags,
    VAddSubParamsf32, VCVTParams, VCmpParamsf32, VMRSTarget, VMovCr2DpParams, VMovCrSpParams,
    VMovImmParams32, VMovImmParams64, VMovRegParamsf32,
};

use crate::core::instruction::VLoadAndStoreParams;
use crate::core::register::{DoubleReg, ExtensionReg, SingleReg};

use super::*;

use crate::core::register::Reg;

#[test]
fn test_is_thumb32() {
    assert!(is_thumb32(0b1110_1000_0000_0000));
    assert!(is_thumb32(0b1111_0000_0000_0000));
    assert!(is_thumb32(0b1111_1000_0000_0000));
    assert!(!is_thumb32(0b1110_0000_0000_0000));
    assert!(is_thumb32(0b1111_1111_1111_1111));
    assert!(!is_thumb32(0b0000_0000_0000_0001));
}

#[test]
fn test_decode_mov() {
    match decode_16(0x4600) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R0);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4608) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R1);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4610) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R2);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4618) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R3);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4620) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R4);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4628) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R5);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4630) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R6);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4638) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R7);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4640) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R8);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4648) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R9);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4650) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R10);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4658) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R11);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4660) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R12);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4668) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::SP);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4670) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::LR);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    match decode_16(0x4678) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::PC);
            assert!(!params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }

    match decode_16(0x0001) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R1);
            assert!(params.rm == Reg::R0);
            assert!(params.setflags);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    //MOVS (mov immediate)
    assert_eq!(
        decode_16(0x2001),
        Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R0,
                imm32: Imm32Carry::NoCarry { imm32: 1 },
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false,
        }
    );

    assert_eq!(
        decode_16(0x2101),
        Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R1,
                imm32: Imm32Carry::NoCarry { imm32: 1 },
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_bx() {
    //BX LR
    match decode_16(0x4770) {
        Instruction::BX { rm } => {
            assert!(rm == Reg::LR);
        }
        _ => {
            unreachable!();
        }
    }
    //BX R1
    match decode_16(0x4708) {
        Instruction::BX { rm } => {
            assert!(rm == Reg::R1);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_cmp() {
    //CMP R0, R0
    assert_eq!(
        decode_16(0x2800),
        Instruction::CMP_imm {
            params: RegImmParams {
                r: Reg::R0,
                imm32: 0,
            },
            thumb32: false,
        }
    );
    // CMP R1, R4
    match decode_16(0x42a1) {
        Instruction::CMP_reg { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            assert!(params.rm == Reg::R4);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    // CMP R2, #0
    assert_eq!(
        decode_16(0x2a00),
        Instruction::CMP_imm {
            params: RegImmParams {
                r: Reg::R2,
                imm32: 0,
            },
            thumb32: false,
        }
    );
    // CMP LR, R4
    assert_eq!(
        decode_16(0x45A6),
        Instruction::CMP_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::LR,
                rm: Reg::R4,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: false
        }
    );
}

#[test]
fn test_decode_b() {
    // BEQ.N
    if let Instruction::B_t13 { params, thumb32 } = decode_16(0xd001) {
        assert_eq!(params.cond, Condition::EQ);
        assert_eq!(params.imm32, (1 << 1));
        assert!(!thumb32);
    } else {
        println!(" {}", decode_16(0xd001));
        unreachable!();
    }
    // BNE.N
    match decode_16(0xd1f8) {
        Instruction::B_t13 { params, thumb32 } => {
            assert!(params.cond == Condition::NE);
            assert!(params.imm32 == -16);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
    // B.N (PC + 8)
    match decode_16(0xE004) {
        Instruction::B_t24 { imm32, thumb32 } => {
            assert!(imm32 == (4 << 1));
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_push() {
    // PUSH  {R4, LR}
    match decode_16(0xb510) {
        Instruction::PUSH { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::LR], elems);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_pop() {
    // POP  {R4, LR}
    match decode_16(0xbd10) {
        Instruction::POP { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::PC], elems);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldr() {
    // LDR.N R1, [PC, 0x1c]
    match decode_16(0x4907) {
        Instruction::LDR_lit { params, thumb32 } => {
            assert!(params.rt == Reg::R1);
            assert!(params.imm32 == (7 << 2));
            assert!(!thumb32);
            assert!(params.add);
        }
        _ => {
            unreachable!();
        }
    }
    // LDR R2, [R1]
    match decode_16(0x680a) {
        Instruction::LDR_imm { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            assert!(params.rt == Reg::R2);
            assert!(params.imm32 == 0);
            assert!(params.index);
            assert!(params.add);
            assert!(!params.wback);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_add_reg_pc() {
    // ADD R1,R1, PC
    assert_eq!(
        decode_16(0x4479),
        Instruction::ADD_reg {
            params: Reg3ShiftParams {
                rd: Reg::R1,
                rn: Reg::R1,
                rm: Reg::PC,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_add_reg_imm() {
    // ADDS R1, R1, 24
    match decode_16(0x3118) {
        Instruction::ADD_imm { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            assert!(params.rd == Reg::R1);
            assert!(params.imm32 == 24);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_add_reg_sp() {
    // ADD R1, SP, #0xc
    match decode_16(0xa903) {
        Instruction::ADD_imm { params, thumb32 } => {
            assert!(params.rn == Reg::SP);
            assert!(params.rd == Reg::R1);
            assert!(params.imm32 == 0xc);
            assert!(params.setflags == SetFlags::False);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_sub() {
    // SUB SP,SP, #0x8
    match decode_16(0xb082) {
        Instruction::SUB_imm { params, thumb32 } => {
            assert!(params.rd == Reg::SP);
            assert!(params.rn == Reg::SP);
            assert!(params.imm32 == 0x8);
            assert!(params.setflags == SetFlags::False);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_sub2() {
    // SUBS R2,R2,#48
    match decode_16(0x3a30) {
        Instruction::SUB_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rn == Reg::R2);
            assert!(params.imm32 == 48);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_tst() {
    // TST R4, R1
    match decode_16(0x420c) {
        Instruction::TST_reg { params, thumb32 } => {
            assert!(params.rn == Reg::R4);
            assert!(params.rm == Reg::R1);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldrb_imm() {
    // LDRB R0, [R0m 0]
    assert_eq!(
        decode_16(0x7800),
        Instruction::LDRB_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::R0,
                imm32: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_ldrb_imm2() {
    // LDRB R2, [R0, 0x10]
    assert_eq!(
        decode_16(0x7c02),
        Instruction::LDRB_imm {
            params: Reg2FullParams {
                rt: Reg::R2,
                rn: Reg::R0,
                imm32: 0x10,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_mvns() {
    // MVNS R5,R5
    match decode_16(0x43ed) {
        Instruction::MVN_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R5);
            assert!(params.rm == Reg::R5);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_lsls() {
    // LSLS R1, R4, #2
    match decode_16(0x00a1) {
        Instruction::LSL_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R1);
            assert!(params.rm == Reg::R4);
            assert!(params.shift_n == 2);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_adr() {
    // ADR R0, PC, #(7<<2)
    match decode_16(0xa007) {
        Instruction::ADR { params, thumb32 } => {
            assert!(params.r == Reg::R0);
            assert!(params.imm32 == 7 << 2);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_bkpt() {
    // BKPT #0xab
    assert_eq!(decode_16(0xbeab), Instruction::BKPT { imm32: 0xab });
}

#[test]
fn test_decode_strb() {
    // STRB R0, [R1]
    assert_eq!(
        decode_16(0x7008),
        Instruction::STRB_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::R1,
                imm32: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_str_reg() {
    // STR R0, [R1, R2]
    assert_eq!(
        decode_16(0x5088),
        Instruction::STR_reg {
            params: Reg3FullParams {
                rt: Reg::R0,
                rn: Reg::R1,
                rm: Reg::R2,
                shift_t: SRType::LSL,
                shift_n: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_nop() {
    // NOP
    match decode_16(0xbf00) {
        Instruction::NOP { thumb32 } => {
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_mul() {
    // MULS R4, R0, R4
    match decode_16(0x4344) {
        Instruction::MUL { params, thumb32 } => {
            assert!(params.rd == Reg::R4);
            assert!(params.rn == Reg::R0);
            assert!(params.rm == Reg::R4);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_orr() {
    // ORRS R3, R3, R1
    match decode_16(0x430b) {
        Instruction::ORR_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R3);
            assert!(params.rn == Reg::R3);
            assert!(params.rm == Reg::R1);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_lsr_imm() {
    // LSRS R3, R0, #8
    match decode_16(0x0a03) {
        Instruction::LSR_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R3);
            assert!(params.rm == Reg::R0);
            assert!(params.shift_n == 8);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_lsr_reg() {
    // LSRS R1, R1, R4
    match decode_16(0x40e1) {
        Instruction::LSR_reg { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R1);
            assert_eq!(params.rn, Reg::R1);
            assert_eq!(params.rm, Reg::R4);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_adc_reg() {
    // ADCS R2,R2,R2
    match decode_16(0x4152) {
        Instruction::ADC_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rm == Reg::R2);
            assert!(params.rn == Reg::R2);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_asr_imm() {
    // ASR R2,R2,#8
    match decode_16(0x1212) {
        Instruction::ASR_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rm == Reg::R2);
            assert!(params.shift_n == 8);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_strh_imm() {
    // STRH R0, [R1, #0x38]
    assert_eq!(
        decode_16(0x8708),
        Instruction::STRH_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::R1,
                imm32: 0x38,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_uxtb() {
    // UXTB R1,R1
    match decode_16(0xb2c9) {
        Instruction::UXTB { params, thumb32 } => {
            assert!(params.rd == Reg::R1);
            assert!(params.rm == Reg::R1);
            assert!(!thumb32);
            assert!(params.rotation == 0);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_bic() {
    // BICS R2,R2,R0
    match decode_16(0x4382) {
        Instruction::BIC_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rn == Reg::R2);
            assert!(params.rm == Reg::R0);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldm() {
    // LDM R2!, {R0, R1}
    match decode_16(0xca03) {
        Instruction::LDM { params, thumb32 } => {
            assert!(params.rn == Reg::R2);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1], elems);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldm2() {
    // LDM R1!, {R3}
    match decode_16(0xc908) {
        Instruction::LDM { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R3], elems);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldm3() {
    // LDM R4!, {R0-R2}
    match decode_16(0xcc07) {
        Instruction::LDM { params, thumb32 } => {
            assert!(params.rn == Reg::R4);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_stm() {
    // STM R2!, {R0, R1}
    match decode_16(0xc203) {
        Instruction::STM { params, thumb32 } => {
            assert!(params.rn == Reg::R2);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1], elems);
            assert!(params.wback);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_stm2() {
    // STM R3!, {R0-R2}
    match decode_16(0xc307) {
        Instruction::STM { params, thumb32 } => {
            assert!(params.rn == Reg::R3);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
            assert!(params.wback);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldrh() {
    // LDRH R0,[R0, #0x38]
    match decode_16(0x8f00) {
        Instruction::LDRH_imm { params, thumb32 } => {
            assert!(params.rn == Reg::R0);
            assert!(params.rt == Reg::R0);
            assert!(params.imm32 == 0x38);
            assert!(params.index);
            assert!(params.add);
            assert!(!params.wback);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_and() {
    // ANDS R2,R2,R3
    match decode_16(0x401a) {
        Instruction::AND_reg { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R2);
            assert_eq!(params.rn, Reg::R2);
            assert_eq!(params.rm, Reg::R3);
            assert!(!thumb32);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
            assert!(params.setflags == SetFlags::NotInITBlock);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_cmn() {
    // CMN R4,R5
    match decode_16(0x42ec) {
        Instruction::CMN_reg { params, thumb32 } => {
            assert!(params.rn == Reg::R4);
            assert!(params.rm == Reg::R5);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_sbc() {
    // SBCS R5, R5, R3
    match decode_16(0x419d) {
        Instruction::SBC_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R5);
            assert!(params.rn == Reg::R5);
            assert!(params.rm == Reg::R3);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_strb2() {
    // STRB R2, [R0, R5]
    assert_eq!(
        decode_16(0x5542),
        Instruction::STRB_reg {
            params: Reg3FullParams {
                rt: Reg::R2,
                rn: Reg::R0,
                rm: Reg::R5,
                index: true,
                add: true,
                wback: false,
                shift_n: 0,
                shift_t: SRType::LSL,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_ldrsh() {
    // LDRSH R0, [R6, R0]
    assert_eq!(
        decode_16(0x5e30),
        Instruction::LDRSH_reg {
            params: Reg3FullParams {
                rt: Reg::R0,
                rn: Reg::R6,
                rm: Reg::R0,
                shift_t: SRType::LSL,
                shift_n: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_strh_reg() {
    // STRH R4, [R6, R1]
    assert_eq!(
        decode_16(0x5274),
        Instruction::STRH_reg {
            params: Reg3FullParams {
                rt: Reg::R4,
                rn: Reg::R6,
                rm: Reg::R1,
                index: true,
                add: true,
                wback: false,
                shift_n: 0,
                shift_t: SRType::LSL,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_eor_reg() {
    // EOR R0, R0, R4
    match decode_16(0x4060) {
        Instruction::EOR_reg { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R0);
            assert_eq!(params.rn, Reg::R0);
            assert_eq!(params.rm, Reg::R4);
            assert_eq!(params.setflags, SetFlags::NotInITBlock);
            assert!(!thumb32);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_ldrsb_reg() {
    // LDRSB R4, [R4, R0]
    assert_eq!(
        decode_16(0x5624),
        Instruction::LDRSB_reg {
            params: Reg3FullParams {
                rt: Reg::R4,
                rn: Reg::R4,
                rm: Reg::R0,
                shift_t: SRType::LSL,
                shift_n: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_sxth_reg() {
    // SXTH R1,R1
    match decode_16(0xb209) {
        Instruction::SXTH { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R1);
            assert_eq!(params.rm, Reg::R1);
            assert!(!thumb32);
            assert_eq!(params.rotation, 0);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_rsb_imm() {
    // RSB R2, R0, #0
    assert_eq!(
        decode_16(0x4242),
        Instruction::RSB_imm {
            params: Reg2ImmParams {
                rd: Reg::R2,
                rn: Reg::R0,
                imm32: 0,
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false
        }
    );
}

/*#[test]
fn test_decode_mrs() {
    // MRS R0, ipsr
    assert_eq!(
        decode_32(0xf3ef_8005),
        Instruction::MRS {
            rd: Reg::R0,
            spec_reg: SpecialReg::IPSR,
        }
    );
}*/

#[cfg(feature = "armv6m")]
#[test]
fn test_decode_cpsid() {
    // CPSID i
    assert_eq!(decode_16(0xB672), Instruction::CPS { im: true });
}

#[cfg(any(feature = "armv7m", feature = "armv7em"))]
#[test]
fn test_decode_cpsid() {
    // CPSID i
    assert_eq!(
        decode_16(0xB672),
        Instruction::CPS {
            im: true,
            affect_pri: true,
            affect_fault: false
        }
    );
}

#[test]
fn test_decode_lsl_2() {
    // LSL r1, r1, #31
    assert_eq!(
        decode_16(0x07c9),
        Instruction::LSL_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R1,
                rm: Reg::R1,
                shift_n: 31,
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_bl_t1() {
    // BL -130
    assert_eq!(decode_32(0xf7ff_ffbf), Instruction::BL { imm32: -130 });

    // BL -5694
    assert_eq!(decode_32(0xf7fe_fce1), Instruction::BL { imm32: -5694 });
}

#[test]
fn test_decode_ldrw_imm() {
    // LDR.W R1, [R0], #0x4
    assert_eq!(
        decode_32(0xf85_01b04),
        Instruction::LDR_imm {
            params: Reg2FullParams {
                rt: Reg::R1,
                rn: Reg::R0,
                imm32: 0x4,
                index: false,
                add: true,
                wback: true,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strw_imm() {
    // STR.W R4, [R3], #0x4
    assert_eq!(
        decode_32(0xf843_4b04),
        Instruction::STR_imm {
            params: Reg2FullParams {
                rt: Reg::R4,
                rn: Reg::R3,
                imm32: 4,
                index: false,
                add: true,
                wback: true,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_cbz() {
    // CBZ R1, 0x3be4 (executed on addr 0x3bc2)
    assert_eq!(
        decode_16(0xb179),
        Instruction::CBZ {
            params: ParamsRegImm32 {
                rn: Reg::R1,
                imm32: 30,
            }
        }
    );
}

#[test]
fn test_decode_cbnz() {
    // bb4b            cbnz    r3, (82 offset)
    assert_eq!(
        decode_16(0xbb4b),
        Instruction::CBNZ {
            params: ParamsRegImm32 {
                rn: Reg::R3,
                imm32: 82,
            }
        }
    );
}

#[test]
fn test_decode_it() {
    // ITT MI
    assert_eq!(
        decode_16(0xbf44),
        Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::MI,
            mask: 0x4,
        }
    );
}

#[test]
fn test_decode_itttt_cc() {
    // 0xbf3f ITTTT CC
    assert_eq!(
        decode_16(0xbf3f),
        Instruction::IT {
            x: Some(ITCondition::Then),
            y: Some(ITCondition::Then),
            z: Some(ITCondition::Then),
            firstcond: Condition::CC,
            mask: 0b1111,
        }
    );
}

#[test]
fn test_decode_itt_cc() {
    // 0xbf3c ITTCC
    assert_eq!(
        decode_16(0xbf3c),
        Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::CC,
            mask: 0b1100,
        }
    );
}

#[test]
fn test_decode_pushw() {
    // PUSH.W {R4-R11, LR}
    // PUSH  {R4, LR}
    match decode_32(0xe92d_4ff0) {
        Instruction::PUSH { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(
                vec![
                    Reg::R4,
                    Reg::R5,
                    Reg::R6,
                    Reg::R7,
                    Reg::R8,
                    Reg::R9,
                    Reg::R10,
                    Reg::R11,
                    Reg::LR,
                ],
                elems
            );

            assert!(thumb32);
        }
        _ => {
            unreachable!()
        }
    }
}

#[test]
fn test_decode_subw_imm() {
    // SUBW SP,SP,#2084
    assert_eq!(
        decode_32(0xf6ad_0d24),
        Instruction::SUB_imm {
            params: Reg2ImmParams {
                rd: Reg::SP,
                rn: Reg::SP,
                imm32: 2084,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_tbb() {
    // TBB [PC, R0]
    assert_eq!(
        decode_32(0xe8df_f000),
        Instruction::TBB {
            params: Reg2RnRmParams {
                rn: Reg::PC,
                rm: Reg::R0,
            }
        }
    );
}

#[test]
fn test_decode_strh_w() {
    // STRH.W R0, [SP, #0x10]
    assert_eq!(
        decode_32(0xf8ad_0010),
        Instruction::STRH_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::SP,
                imm32: 0x10,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strh_w_2() {
    // 0xf8a8_7000 -> STRH.W R7, [R8]
    assert_eq!(
        decode_32(0xf8a8_7000),
        Instruction::STRH_imm {
            params: Reg2FullParams {
                rt: Reg::R7,
                rn: Reg::R8,
                imm32: 0x0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_mov_w() {
    // MOV.W R8, #-1
    assert_eq!(
        decode_32(0xf04f_38ff),
        Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R8,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0xffff_ffff, false),
                    imm32_c1: (0xffff_ffff, true),
                },
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsh_reg_w() {
    // LDRSH.W R0, [R0, R0, LSL #0]
    assert_eq!(
        decode_32(0xf930_0000),
        Instruction::LDRSH_reg {
            params: Reg3FullParams {
                rt: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R0,
                shift_t: SRType::LSL,
                shift_n: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsh_imm_w() {
    // LDRSH.W R0, [SP, #0x10]
    assert_eq!(
        decode_32(0xf9bd_0010),
        Instruction::LDRSH_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::SP,
                imm32: 0x10,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ubfx() {
    // UBFX R1, R0, #1, #1
    assert_eq!(
        decode_32(0xf3c0_0140),
        Instruction::UBFX {
            params: BfxParams {
                rd: Reg::R1,
                rn: Reg::R0,
                lsb: 1,
                widthminus1: 0,
            }
        }
    );
}

#[test]
fn test_decode_sbfx() {
    // SBFX    r3, r3, #0, #1
    assert_eq!(
        decode_32(0xf343_0300),
        Instruction::SBFX {
            params: BfxParams {
                rd: Reg::R3,
                rn: Reg::R3,
                lsb: 0,
                widthminus1: 0,
            }
        }
    );
}

#[test]
fn test_decode_udiv() {
    // UDIV R0, R0, R1
    assert_eq!(
        decode_32(0xfbb0_f0f1),
        Instruction::UDIV {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R1,
            }
        }
    );
}

#[test]
fn test_decode_mla() {
    // MLA R1, R7, R2, R1
    assert_eq!(
        decode_32(0xfb07_1102),
        Instruction::MLA {
            params: Reg4NoSetFlagsParams {
                rd: Reg::R1,
                rn: Reg::R7,
                rm: Reg::R2,
                ra: Reg::R1,
            }
        }
    );
}

#[test]
fn test_decode_ldrb_w() {
    // 0xf896_0020 LDRB.W R0 [R6, #0x20]
    assert_eq!(
        decode_32(0xf896_0020),
        Instruction::LDRB_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::R6,
                imm32: 0x20,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_add_reg_w() {
    // 0xeb01_03ca ADD.W R3, R1, R10, LSL #3
    assert_eq!(
        decode_32(0xeb01_03ca),
        Instruction::ADD_reg {
            params: Reg3ShiftParams {
                rd: Reg::R3,
                rn: Reg::R1,
                rm: Reg::R10,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 3,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_cmp_imm_w() {
    // 0xf1ba_0f00 CMP.W R10, #0
    assert_eq!(
        decode_32(0xf1ba_0f00),
        Instruction::CMP_imm {
            params: RegImmParams {
                r: Reg::R10,
                imm32: 0,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_and_imm_w() {
    // 0xf01a_0c03 ANDS.W R12, R10, 3
    assert_eq!(
        decode_32(0xf01a_0c03),
        Instruction::AND_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R12,
                rn: Reg::R10,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (3, false),
                    imm32_c1: (3, true),
                },
                setflags: true,
            }
        }
    );
}

#[test]
fn test_decode_eor_reg_w() {
    // 0xea8e_0402 EOR.W R4, LR, R2
    assert_eq!(
        decode_32(0xea8e_0402),
        Instruction::EOR_reg {
            params: Reg3ShiftParams {
                rd: Reg::R4,
                rn: Reg::LR,
                rm: Reg::R2,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_orr_reg_w() {
    // 0xea44_04c8  ORR.W R4, R4, R8, LSL #3
    assert_eq!(
        decode_32(0xea44_04c8),
        Instruction::ORR_reg {
            params: Reg3ShiftParams {
                rd: Reg::R4,
                rn: Reg::R4,
                rm: Reg::R8,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 3,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_lsl_w_imm() {
    // LSL.W R8,R8,1
    assert_eq!(
        decode_32(0xea4f_0848),
        Instruction::LSL_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R8,
                rm: Reg::R8,
                shift_n: 1,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_lsr_w_imm() {
    // LSRS.W R12,R10,2
    assert_eq!(
        decode_32(0xea5f_0c9a),
        Instruction::LSR_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R12,
                rm: Reg::R10,
                shift_n: 2,
                setflags: SetFlags::True,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_pop_w() {
    //0xe8bd_47f0 POP.W {R4-R10, LR}
    match decode_32(0xe8bd_47f0) {
        Instruction::POP { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(
                vec![
                    Reg::R4,
                    Reg::R5,
                    Reg::R6,
                    Reg::R7,
                    Reg::R8,
                    Reg::R9,
                    Reg::R10,
                    Reg::LR
                ],
                elems
            );
            assert!(thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_mul_w() {
    //0xfb04_f604 MUL R6, R4, R4
    assert_eq!(
        decode_32(0xfb04_f604),
        Instruction::MUL {
            params: Reg3Params {
                rd: Reg::R6,
                rn: Reg::R4,
                rm: Reg::R4,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_asr_w() {
    //0xEA4f_39e2 ASR.W R9, R2, #15
    assert_eq!(
        decode_32(0xea4f_39e2),
        Instruction::ASR_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R9,
                rm: Reg::R2,
                shift_n: 15,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrh_w() {
    //0xf834_9b02 LDRH.W R9, [R4], #0x2
    assert_eq!(
        decode_32(0xf834_9b02),
        Instruction::LDRH_imm {
            params: Reg2FullParams {
                rt: Reg::R9,
                rn: Reg::R4,
                imm32: 2,
                add: true,
                index: false,
                wback: true,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_uxtb_w() {
    //0xfa5f_f989 UXTB.W R9, R9
    assert_eq!(
        decode_32(0xfa5f_f989),
        Instruction::UXTB {
            params: Reg2UsizeParams {
                rd: Reg::R9,
                rm: Reg::R9,
                rotation: 0,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldr_lit_w() {
    //0xf8df_90cc LDR.W R9, [PC, #0xcc]
    assert_eq!(
        decode_32(0xf8df_90cc),
        Instruction::LDR_lit {
            params: RegImm32AddParams {
                rt: Reg::R9,
                imm32: 0xcc,
                add: true,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldr_reg_w() {
    //0xf859_4024 LDR.W R4, [R9,R4, LSL #2]
    assert_eq!(
        decode_32(0xf859_4024),
        Instruction::LDR_reg {
            params: Reg3FullParams {
                rt: Reg::R4,
                rn: Reg::R9,
                rm: Reg::R4,
                shift_t: SRType::LSL,
                shift_n: 2,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strb_imm_w() {
    //0xf80e_ab01 STRB.W R10, [LR], #1
    assert_eq!(
        decode_32(0xf80e_ab01),
        Instruction::STRB_imm {
            params: Reg2FullParams {
                rt: Reg::R10,
                rn: Reg::LR,
                imm32: 1,
                index: false,
                add: true,
                wback: true,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strb_reg_w() {
    //0xf80c_e007 STRB.W LR, [R12, R7]
    assert_eq!(
        decode_32(0xf80c_e007),
        Instruction::STRB_reg {
            params: Reg3FullParams {
                rt: Reg::LR,
                rn: Reg::R12,
                rm: Reg::R7,
                index: true,
                add: true,
                wback: false,
                shift_n: 0,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_mov_reg_w() {
    // MOV.W R8, R3
    assert_eq!(
        decode_32(0xea4f_0803),
        Instruction::MOV_reg {
            params: Reg2Params {
                rd: Reg::R8,
                rm: Reg::R3,
                setflags: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_sxth_w() {
    // SXTH.W R10, R10
    assert_eq!(
        decode_32(0xfa0f_fa8a),
        Instruction::SXTH {
            params: Reg2UsizeParams {
                rd: Reg::R10,
                rm: Reg::R10,
                rotation: 0
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_adds_w() {
    // 0xf118_0801 ADDS.W R8, R8, #1
    assert_eq!(
        decode_32(0xf118_0801),
        Instruction::ADD_imm {
            params: Reg2ImmParams {
                rn: Reg::R8,
                rd: Reg::R8,
                imm32: 1,
                setflags: SetFlags::True
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_bfi_w() {
    // 0xf363_0407 BFI R4, R3, #0, #8
    assert_eq!(
        decode_32(0xf363_0407),
        Instruction::BFI {
            params: BfiParams {
                rd: Reg::R4,
                rn: Reg::R3,
                lsbit: 0,
                width: 8,
            }
        }
    );
}

#[test]
fn test_decode_sdiv() {
    // 0xfb99_f2fa SDIV, R2, R9, R10
    assert_eq!(
        decode_32(0xfb99_f2fa),
        Instruction::SDIV {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R9,
                rm: Reg::R10,
            }
        }
    );
}

#[test]
fn test_decode_mls() {
    // 0xfb02_921a MLS R2, R2, R10, R9
    assert_eq!(
        decode_32(0xfb02_921a),
        Instruction::MLS {
            params: Reg4NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R2,
                rm: Reg::R10,
                ra: Reg::R9,
            }
        }
    );
}

#[test]
fn test_decode_strh_reg_w() {
    //  STRH.W  R12, [R6, R9, LSL #1]
    assert_eq!(
        decode_32(0xf826_c019),
        Instruction::STRH_reg {
            params: Reg3FullParams {
                rt: Reg::R12,
                rn: Reg::R6,
                rm: Reg::R9,
                index: true,
                add: true,
                wback: false,
                shift_n: 1,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_cmn_w_reg() {
    // CMN.W R12, R1, LSL #1
    assert_eq!(
        decode_32(0xeb1c_0f41),
        Instruction::CMN_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R12,
                rm: Reg::R1,
                shift_n: 1,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_subw_reg() {
    // 0xebb0_0b09
    // SUBS.W R11, R0, R9
    assert_eq!(
        decode_32(0xebb0_0b09),
        Instruction::SUB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R11,
                rn: Reg::R0,
                rm: Reg::R9,
                setflags: SetFlags::True,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: true,
        }
    );

    // 0xEBA4_5613
    // SUB.W R6, R4, R3, LSR #20
    assert_eq!(
        decode_32(0xEBA4_5613),
        Instruction::SUB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R6,
                rn: Reg::R4,
                rm: Reg::R3,
                setflags: SetFlags::False,
                shift_t: SRType::LSR,
                shift_n: 20,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_str_reg_w() {
    // 0xf841_002a
    // STR.W R0, [R1, R10, LSL #2]
    assert_eq!(
        decode_32(0xf841_002a),
        Instruction::STR_reg {
            params: Reg3FullParams {
                rt: Reg::R0,
                rn: Reg::R1,
                rm: Reg::R10,
                shift_t: SRType::LSL,
                shift_n: 2,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_orr_imm_w() {
    // 0xf040_0010
    // ORR.W R0, R0, #16
    assert_eq!(
        decode_32(0xf040_0010),
        Instruction::ORR_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R0,
                rn: Reg::R0,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (16, false),
                    imm32_c1: (16, true)
                },
                setflags: false
            }
        }
    );
}

#[test]
fn test_decode_strd_w() {
    // 0xe9cd_0100 -> STRD R0, R1, [SP]
    assert_eq!(
        decode_32(0xe9cd_0100),
        Instruction::STRD_imm {
            params: Reg2DoubleParams {
                rt: Reg::R0,
                rt2: Reg::R1,
                rn: Reg::SP,
                imm32: 0,
                index: true,
                add: true,
                wback: false,
            }
        }
    );
}

#[test]
fn test_decode_ldrd_w() {
    // 0xe9d5_0100 -> LDRD R0, R1, [R5]
    assert_eq!(
        decode_32(0xe9d5_0100),
        Instruction::LDRD_imm {
            params: Reg2DoubleParams {
                rt: Reg::R0,
                rt2: Reg::R1,
                rn: Reg::R5,
                imm32: 0,
                index: true,
                add: true,
                wback: false,
            },
        }
    );
}

#[test]
fn test_decode_ulmull() {
    // 0xfba4_2300 -> UMULL R2, R3, R4, R0
    assert_eq!(
        decode_32(0xfba4_2300),
        Instruction::UMULL {
            params: Reg643232Params {
                rdlo: Reg::R2,
                rdhi: Reg::R3,
                rn: Reg::R4,
                rm: Reg::R0,
            }
        }
    );
}

#[test]
fn test_decode_smull() {
    // fb83 320b       smull   r3, r2, r3, fp
    assert_eq!(
        decode_32(0xfb83_320b),
        Instruction::SMULL {
            params: Reg643232Params {
                rdlo: Reg::R3,
                rdhi: Reg::R2,
                rn: Reg::R3,
                rm: Reg::R11,
            }
        }
    );
}

#[test]
fn test_decode_lsr_w_reg() {
    // 0xfa30_f009 -> LSRS.W R0, R0, R9
    assert_eq!(
        decode_32(0xfa30_f009),
        Instruction::LSR_reg {
            params: Reg3Params {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R9,
                setflags: SetFlags::True,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_rsb_w_reg() {
    //0xf1c6_003c -> RSB.W R0, R6, #60
    assert_eq!(
        decode_32(0xf1c6_003c),
        Instruction::RSB_imm {
            params: Reg2ImmParams {
                rd: Reg::R0,
                rn: Reg::R6,
                imm32: 60,
                setflags: SetFlags::False,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_b_pl_w() {
    //0xf57f_ad69 -> BPL.W -1326
    assert_eq!(
        decode_32(0xf57f_ad69),
        Instruction::B_t13 {
            params: CondBranchParams {
                cond: Condition::PL,
                imm32: -1326,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_tst_imm_w() {
    //0xf011_3f80 -> TST.W R1, 0x8080_8080
    assert_eq!(
        decode_32(0xf011_3f80),
        Instruction::TST_imm {
            params: RegImmCarryNoSetFlagsParams {
                rn: Reg::R1,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0x8080_8080, false),
                    imm32_c1: (0x8080_8080, true),
                },
            }
        }
    );
}

//

#[test]
fn test_decode_sbc_reg_w() {
    //0xeb6a_0a4a -> SBC.W R10, R10, R10, LSL #1
    assert_eq!(
        decode_32(0xeb6a_0a4a),
        Instruction::SBC_reg {
            params: Reg3ShiftParams {
                rd: Reg::R10,
                rn: Reg::R10,
                rm: Reg::R10,
                shift_n: 1,
                shift_t: SRType::LSL,
                setflags: SetFlags::False,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_stmdb_w() {
    //0xe920_003c -> STMDB R0!, {R2-R5}
    match decode_32(0xe920_003c) {
        Instruction::STMDB { params } => {
            assert!(params.rn == Reg::R0);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R2, Reg::R3, Reg::R4, Reg::R5], elems);
            assert!(params.wback);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_bic_imm_w() {
    //0xf024_00ff -> BIC.W R0, R4, #255

    assert_eq!(
        decode_32(0xf024_00ff),
        Instruction::BIC_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R0,
                rn: Reg::R4,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (255, false),
                    imm32_c1: (255, true),
                },
                setflags: false,
            }
        }
    );
}

#[test]
fn test_decode_ldrh_reg_w() {
    //0xf838_301a -> LDRH.W R3, [R8, R10, LSL #1]

    assert_eq!(
        decode_32(0xf838_301a),
        Instruction::LDRH_reg {
            params: Reg3FullParams {
                rt: Reg::R3,
                rn: Reg::R8,
                rm: Reg::R10,
                shift_t: SRType::LSL,
                shift_n: 1,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_eor_imm_w() {
    //0xf481_4120 -> EOR.W R1, R1, #40960 ; 0xa000
    assert_eq!(
        decode_32(0xf481_4120),
        Instruction::EOR_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R1,
                rn: Reg::R1,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0xa000, false),
                    imm32_c1: (0xa000, false)
                },
                setflags: false
            }
        }
    );
}

#[test]
fn test_decode_clz_w() {
    //0xfab0_f180 -> CLZ R1, R0
    assert_eq!(
        decode_32(0xfab0_f180),
        Instruction::CLZ {
            params: Reg2RdRmParams {
                rd: Reg::R1,
                rm: Reg::R0,
            }
        }
    );
}

#[test]
fn test_decode_pop_t3_w() {
    //0xf85d_eb04 -> LDR.W LR, [SP], #4   // POP.W LR  (pop 3)
    match decode_32(0xf85d_eb04) {
        Instruction::POP { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::LR], elems);
            assert!(thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_and_reg_w() {
    //0xea15_5411 -> ANDS.W R4, R5, R1, LSR #20
    assert_eq!(
        decode_32(0xea15_5411),
        Instruction::AND_reg {
            params: Reg3ShiftParams {
                rd: Reg::R4,
                rn: Reg::R5,
                rm: Reg::R1,
                setflags: SetFlags::True,
                shift_t: SRType::LSR,
                shift_n: 20,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_rsb_reg_w() {
    //0xebc0_1046 -> RSB.W R0, R0, R6, LSL #5
    assert_eq!(
        decode_32(0xebc0_1046),
        Instruction::RSB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R6,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 5,
            },
            thumb32: true,
        }
    );
    //0xebd7_20c0 -> RSBS.W R0, R7, R0, LSL #11
    assert_eq!(
        decode_32(0xebd7_20c0),
        Instruction::RSB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R0,
                rn: Reg::R7,
                rm: Reg::R0,
                setflags: SetFlags::True,
                shift_t: SRType::LSL,
                shift_n: 11,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_sbc_imm_w() {
    //0xf167_0700 -> SBC.W R7, R7, #0
    assert_eq!(
        decode_32(0xf167_0700),
        Instruction::SBC_imm {
            params: Reg2ImmParams {
                rd: Reg::R7,
                rn: Reg::R7,
                setflags: SetFlags::False,
                imm32: 0
            }
        }
    );
}

#[test]
fn test_decode_adc_reg_w() {
    //0xeb50_500e -> ADCS.W R0, R0, LR, LSL #20

    assert_eq!(
        decode_32(0xeb50_500e),
        Instruction::ADC_reg {
            params: Reg3ShiftParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::LR,
                setflags: SetFlags::True,
                shift_t: SRType::LSL,
                shift_n: 20,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_bic_reg_w() {
    //0xea23_5345 -> BIC.W R3, R3, R5, LSL #21

    assert_eq!(
        decode_32(0xea23_5345),
        Instruction::BIC_reg {
            params: Reg3ShiftParams {
                rd: Reg::R3,
                rn: Reg::R3,
                rm: Reg::R5,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 21,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_adc_imm_w() {
    // 0xf154_0401 -> ADCS.W R4, R4, #1

    assert_eq!(
        decode_32(0xf154_0401),
        Instruction::ADC_imm {
            params: Reg2ImmParams {
                rd: Reg::R4,
                rn: Reg::R4,
                setflags: SetFlags::True,
                imm32: 1
            }
        }
    );
}

#[test]
fn test_decode_teq_reg_w() {
    // 0xea91_0f03 -> TEQ.W R1, R3

    assert_eq!(
        decode_32(0xea91_0f03),
        Instruction::TEQ_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R1,
                rm: Reg::R3,
                shift_t: SRType::LSL,
                shift_n: 0
            }
        }
    );
}

#[test]
fn test_decode_ror_imm_w() {
    // 0xea4f_74f4 -> ROR.W R4, R4, #31

    assert_eq!(
        decode_32(0xea4f_74f4),
        Instruction::ROR_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R4,
                rm: Reg::R4,
                shift_n: 31,
                setflags: SetFlags::False
            }
        }
    );
}

#[test]
fn test_decode_ldm_t2_w() {
    // 0xe8b1_1008 -> LDM R1!, {R3, R12}

    match decode_32(0xe8b1_1008) {
        Instruction::LDM { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            let elems: Vec<_> = params.registers.iter().collect();
            assert_eq!(vec![Reg::R3, Reg::R12], elems);
            assert!(thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_cmp_reg_w() {
    // 0xebb7_1f46 -> CMP.W R7, R6, LSL #5
    assert_eq!(
        decode_32(0xebb7_1f46),
        Instruction::CMP_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R7,
                rm: Reg::R6,
                shift_t: SRType::LSL,
                shift_n: 5,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsb_imm_w() {
    // 0xf995_6000 -> LDRSB R6, [R5]
    assert_eq!(
        decode_32(0xf995_6000),
        Instruction::LDRSB_imm {
            params: Reg2FullParams {
                rt: Reg::R6,
                rn: Reg::R5,
                imm32: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsb_imm_t2() {
    // 0xf917_0c09 -> ldrsb.w r0, [r7, #-9]
    assert_eq!(
        decode_32(0xf917_0c09),
        Instruction::LDRSB_imm {
            params: Reg2FullParams {
                rt: Reg::R0,
                rn: Reg::R7,
                imm32: 9,
                index: true,
                add: false,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_smul_bb() {
    // 0xfb1e_fe08 -> SMULBB LR, LR, R8
    assert_eq!(
        decode_32(0xfb1e_fe08),
        Instruction::SMUL {
            params: Reg3HighParams {
                rd: Reg::LR,
                rn: Reg::LR,
                rm: Reg::R8,
                n_high: false,
                m_high: false
            }
        }
    );
}

#[test]
fn test_decode_smla_bb() {
    // 0xfb15_ee0b -> SMLABB LR, R5, R11, LR
    assert_eq!(
        decode_32(0xfb15_ee0b),
        Instruction::SMLA {
            params: Reg4HighParams {
                rd: Reg::LR,
                rn: Reg::R5,
                rm: Reg::R11,
                ra: Reg::LR,
                n_high: false,
                m_high: false
            }
        }
    );
}

#[test]
fn test_decode_ldrb_reg_w() {
    //0xf816_c004 -> LDRB.W R12, [R6, R4]

    assert_eq!(
        decode_32(0xf816_c004),
        Instruction::LDRB_reg {
            params: Reg3FullParams {
                rt: Reg::R12,
                rn: Reg::R6,
                rm: Reg::R4,
                shift_t: SRType::LSL,
                shift_n: 0,
                index: true,
                add: true,
                wback: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_uxtab_() {
    //0xfa54_f480 UXTAB.W R4, R4, R0

    assert_eq!(
        decode_32(0xfa54_f480),
        Instruction::UXTAB {
            params: Reg3UsizeParams {
                rd: Reg::R4,
                rn: Reg::R4,
                rm: Reg::R0,
                rotation: 0
            }
        }
    );
}

#[test]
fn test_decode_tst_reg_w() {
    // 0xea18_0f03 tst.w   r8, r3

    assert_eq!(
        decode_32(0xea18_0f03),
        Instruction::TST_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R8,
                rm: Reg::R3,
                shift_n: 0,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_pld_reg() {
    // 0xf890_f000 pld [r0]

    assert_eq!(
        decode_32(0xf890_f000),
        Instruction::PLD_imm {
            rn: Reg::R0,
            imm32: 0,
            add: true
        }
    );
}

#[test]
fn test_decode_lsl_reg_t2() {
    // 0xfa0c_f505 ->     lsl.w   r5, ip, r5

    assert_eq!(
        decode_32(0xfa0c_f505),
        Instruction::LSL_reg {
            params: Reg3Params {
                rd: Reg::R5,
                rn: Reg::R12,
                rm: Reg::R5,
                setflags: SetFlags::False
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_orn_reg_t2() {
    // 0xea62 0205       orn     r2, r2, r5

    assert_eq!(
        decode_32(0xea62_0205),
        Instruction::ORN_reg {
            params: Reg3ShiftParams {
                rd: Reg::R2,
                rn: Reg::R2,
                rm: Reg::R5,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 0,
            }
        }
    );
}

#[test]
fn test_decode_uadd8() {
    // fa82 f24c       uadd8   r2, r2, ip
    assert_eq!(
        decode_32(0xfa82_f24c),
        Instruction::UADD8 {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R2,
                rm: Reg::R12,
            }
        }
    );
}

#[test]
fn test_decode_sel() {
    //0xfaa4_f28c       sel     r2, r4, ip

    assert_eq!(
        decode_32(0xfaa4_f28c),
        Instruction::SEL {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R4,
                rm: Reg::R12,
            }
        }
    );
}

#[test]
fn test_decode_tbh() {
    // e8df f013       tbh     [pc, r3, lsl #1]

    assert_eq!(
        decode_32(0xe8df_f013),
        Instruction::TBH {
            params: Reg2RnRmParams {
                rn: Reg::PC,
                rm: Reg::R3,
            }
        }
    );
}

#[test]
fn test_decode_movt() {
    // f2c2 0100       movt    r1, #8192

    assert_eq!(
        decode_32(0xf2c2_0100),
        Instruction::MOVT {
            params: MovtParams {
                rd: Reg::R1,
                imm16: 0x2000
            }
        }
    );
}

#[test]
fn test_decode_mvn_reg_w() {
    // ea6f 5507       mvn.w   r5, r7, lsl #20

    assert_eq!(
        decode_32(0xea6f_5507),
        Instruction::MVN_reg {
            params: Reg2ShiftParams {
                rd: Reg::R5,
                rm: Reg::R7,
                setflags: SetFlags::False,
                shift_n: 20,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_teq_w() {
    //f090 0f00       teq     r0, #0
    assert_eq!(
        decode_32(0xf090_0f00),
        Instruction::TEQ_imm {
            params: RegImmCarryNoSetFlagsParams {
                rn: Reg::R0,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0, false),
                    imm32_c1: (0, true),
                },
            }
        }
    );
}

#[test]
fn test_decode_mov_rxx_w() {
    //ea4f 0232       mov.w   r2, r2, rrx
    assert_eq!(
        decode_32(0xea4f_0232),
        Instruction::RRX {
            params: Reg2Params {
                rd: Reg::R2,
                rm: Reg::R2,
                setflags: false,
            }
        }
    );
}

#[test]
fn test_decode_str_imm_t4() {
    //f84d cd04       str.w   ip, [sp, #-4]!
    // => same as PUSH r12
    match decode_32(0xf84d_cd04) {
        Instruction::PUSH { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R12], elems);
            assert!(thumb32);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_subw_imm_t4() {
    // f2a4 4333       subw    r3, r4, #1075   ; 0x433
    assert_eq!(
        decode_32(0xf2a4_4333),
        Instruction::SUB_imm {
            params: Reg2ImmParams {
                rd: Reg::R3,
                rn: Reg::R4,
                imm32: 1075,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_asrw_reg_t2() {
    //  fa43 f305       asr.w   r3, r3, r5
    assert_eq!(
        decode_32(0xfa43_f305),
        Instruction::ASR_reg {
            params: Reg3Params {
                rd: Reg::R3,
                rn: Reg::R3,
                rm: Reg::R5,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_dmb() {
    //  f3bf 8f5f       dmb sy
    assert_eq!(decode_32(0xf3bf_8f5f), Instruction::DMB);
}

#[test]
fn test_decode_ldrex() {
    //  e850 3f00       ldrex   r3, [r0]
    assert_eq!(
        decode_32(0xe850_3f00),
        Instruction::LDREX {
            params: Reg2RtRnImm32Params {
                rt: Reg::R3,
                rn: Reg::R0,
                imm32: 0,
            }
        }
    );
}

#[test]
fn test_decode_strex() {
    //  e840 2c00       strex   ip, r2, [r0]
    assert_eq!(
        decode_32(0xe840_2c00),
        Instruction::STREX {
            params: Reg3RdRtRnImm32Params {
                rd: Reg::R12,
                rt: Reg::R2,
                rn: Reg::R0,
                imm32: 0,
            }
        }
    );
}

#[test]
fn test_decode_bfc() {
    //  f36f 011f       bfc     r1, #0, #32
    assert_eq!(
        decode_32(0xf36f_011f),
        Instruction::BFC {
            params: BfcParams {
                rd: Reg::R1,
                lsbit: 0,
                msbit: 31,
            }
        }
    );
}

#[test]
fn test_decode_vldr() {
    //  ed9f 7b86       vldr    d7, [pc, #536]  ; 448 <_vfprintf_r+0x290>
    assert_eq!(
        decode_32(0xed9f_7b86),
        Instruction::VLDR {
            params: VLoadAndStoreParams {
                dd: ExtensionReg::Double { reg: DoubleReg::D7 },
                rn: Reg::PC,
                add: true,
                imm32: 0x86 << 2,
            }
        }
    );
}

#[test]
fn test_decode_vldr_2() {
    //  eddf 7a23             vldr    s15, [pc, #140] @ 180 <floating_point+0xb0>
    assert_eq!(
        decode_32(0xeddf_7a23),
        Instruction::VLDR {
            params: VLoadAndStoreParams {
                dd: ExtensionReg::Single {
                    reg: SingleReg::S15
                },
                rn: Reg::PC,
                add: true,
                imm32: 140,
            }
        }
    );
}

#[test]
fn test_decode_vstr() {
    //250:       ed8d 7b12       vstr    d7, [sp, #72]   ; 0x48
    assert_eq!(
        decode_32(0xed8d_7b12),
        Instruction::VSTR {
            params: VLoadAndStoreParams {
                dd: ExtensionReg::Double { reg: DoubleReg::D7 },
                rn: Reg::SP,
                add: true,
                imm32: 0x48,
            }
        }
    );
}

#[test]
fn test_decode_vpush() {
    //  ed2d 8b02       vpush   {d8}

    match decode_32(0xed2d_8b02) {
        Instruction::VPUSH { params } => {
            assert!(!params.single_regs);
            let double_regs: Vec<_> = params.double_precision_registers.iter().collect();
            assert_eq!(vec![DoubleReg::D8], double_regs);
            assert_eq!(params.imm32, 8);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_vpop() {
    //  ecbd 8b06       vpop    {d8-d10}

    match decode_32(0xecbd_8b06) {
        Instruction::VPOP { params } => {
            assert!(!params.single_regs);
            let double_regs: Vec<_> = params.double_precision_registers.iter().collect();
            assert_eq!(
                vec![DoubleReg::D8, DoubleReg::D9, DoubleReg::D10],
                double_regs
            );
            assert_eq!(params.imm32, 3 * 8);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_vmov_cr_sp() {
    //  ee09 0a10       vmov    s18, r0

    assert_eq!(
        decode_32(0xee09_0a10),
        Instruction::VMOV_cr_sp {
            params: VMovCrSpParams {
                to_arm_register: false,
                rt: Reg::R0,
                sn: SingleReg::S18,
            }
        }
    );
}

#[test]
fn test_decode_vmov_cr_sp_2() {
    //  ee08 3a90       vmov    s17, r3

    assert_eq!(
        decode_32(0xee08_3a90),
        Instruction::VMOV_cr_sp {
            params: VMovCrSpParams {
                to_arm_register: false,
                rt: Reg::R3,
                sn: SingleReg::S17,
            }
        }
    );
}

#[test]
fn test_decode_vmov_cr_sp_3() {
    //  ee19 0a10       vmov    r0, s18

    assert_eq!(
        decode_32(0xee19_0a10),
        Instruction::VMOV_cr_sp {
            params: VMovCrSpParams {
                to_arm_register: true,
                rt: Reg::R0,
                sn: SingleReg::S18,
            }
        }
    );
}

#[test]
fn test_decode_vmov_cr2_dp() {
    //  ec51 0b18       vmov    r0, r1, d8

    assert_eq!(
        decode_32(0xec51_0b18),
        Instruction::VMOV_cr2_dp {
            params: VMovCr2DpParams {
                to_arm_registers: true,
                rt: Reg::R0,
                rt2: Reg::R1,
                dm: DoubleReg::D8,
            }
        }
    );
}

#[test]
fn test_decode_vmov_reg_f32() {
    //eeb0 0a4a       vmov.f32        s0, s20

    assert_eq!(
        decode_32(0xeeb0_0a4a),
        Instruction::VMOV_reg_f32 {
            params: VMovRegParamsf32 {
                sd: SingleReg::S0,
                sm: SingleReg::S20,
            }
        }
    );
}

#[test]
fn test_decode_vmov_imm() {
    // eeb7 6b08       vmov.f64        d6, #120

    assert_eq!(
        decode_32(0xeeb7_6b08),
        Instruction::VMOV_imm_64 {
            params: VMovImmParams64 {
                dd: DoubleReg::D6,
                imm64: 1.5f64.to_bits()
            }
        }
    );
}

#[test]
fn test_decode_vmov_imm_f32() {
    //  eef0 6a00       vmov.f32        s13, #0 @ 0x40000000  2.0

    assert_eq!(
        decode_32(0xeef0_6a00),
        Instruction::VMOV_imm_32 {
            params: VMovImmParams32 {
                sd: SingleReg::S13,
                imm32: 2.0f32.to_bits()
            }
        }
    );
}

#[test]
fn test_decode_vmov_imm_2() {
    //eeff 7a00       vmov.f32        s15, #240       @ 0xbf800000 -1.0

    assert_eq!(
        decode_32(0xeeff_7a00),
        Instruction::VMOV_imm_32 {
            params: VMovImmParams32 {
                sd: SingleReg::S15,
                imm32: 0xbf800000 // -1.0
            }
        }
    );
}

#[test]
fn test_decode_vstm_32_ia() {
    //ecee 7a01       vstmia  lr!, {s15}

    match decode_32(0xecee_7a01) {
        Instruction::VSTM_T2 { params } => {
            assert_eq!(params.mode, AddressingMode::IncrementAfter);
            let single_regs: Vec<_> = params.list.iter().collect();
            assert_eq!(vec![SingleReg::S15], single_regs);
            assert_eq!(params.imm32, 4);
            assert_eq!(params.rn, Reg::LR);
            assert!(params.write_back);
        }
        _ => {
            unreachable!();
        }
    }
}

#[test]
fn test_decode_vabs_32() {
    //eef0 7ae7       vabs.f32        s15, s15

    assert_eq!(
        decode_32(0xeef07ae7),
        Instruction::VABS_f32 {
            params: VMovRegParamsf32 {
                sd: SingleReg::S15,
                sm: SingleReg::S15,
            }
        }
    );
}

#[test]
fn test_decode_vcmp_f32() {
    //eef4 7a47       vcmp.f32        s15, s14

    assert_eq!(
        decode_32(0xeef47a47),
        Instruction::VCMP_f32 {
            params: VCmpParamsf32 {
                sd: SingleReg::S15,
                sm: SingleReg::S14,
                with_zero: false,
                quiet_nan_exc: false,
            }
        }
    );
}

#[test]
fn test_decode_vmrs() {
    //0xeef1 fa10       vmrs    APSR_nzcv, fpscr

    assert_eq!(
        decode_32(0xeef1fa10),
        Instruction::VMRS {
            rt: VMRSTarget::APSRNZCV
        }
    );
}

#[test]
fn test_decode_vadd_f32() {
    // ee77 5a26       vadd.f32        s11, s14, s13

    assert_eq!(
        decode_32(0xee775a26),
        Instruction::VADD_f32 {
            params: VAddSubParamsf32 {
                sd: SingleReg::S11,
                sn: SingleReg::S14,
                sm: SingleReg::S13,
            }
        }
    );
}

#[test]
fn test_decode_vsub_f32() {
    // ee37 5a66       vsub.f32        s10, s14, s13

    assert_eq!(
        decode_32(0xee375a66),
        Instruction::VSUB_f32 {
            params: VAddSubParamsf32 {
                sd: SingleReg::S10,
                sn: SingleReg::S14,
                sm: SingleReg::S13,
            }
        }
    );
}

#[test]
fn test_decode_vct() {
    //eefd 7ac0       vcvt.s32.f32    s15, s0

    assert_eq!(
        decode_32(0xeefd7ac0),
        Instruction::VCVT {
            params: VCVTParams {
                d: ExtensionReg::Single {
                    reg: SingleReg::S15
                },
                m: ExtensionReg::Single { reg: SingleReg::S0 },
                dp_operation: false,
                to_integer: true,
                unsigned: false,
                round_nearest: false,
                round_zero: true,
            }
        }
    );
}
