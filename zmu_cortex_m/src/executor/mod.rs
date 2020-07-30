//!
//! Functionality for running instructions on a Processor.
//!

use crate::bus::Bus;
use crate::core::bits::Bits;
use crate::core::condition::Condition;
use crate::core::exception::{Exception, ExceptionHandling};
use crate::core::fault::Fault;
use crate::core::instruction::{Imm32Carry, Instruction, SetFlags};
use crate::core::monitor::Monitor;
use crate::core::operation::{condition_test, shift, sign_extend, zero_extend, zero_extend_u16};
use crate::core::register::{Apsr, BaseReg, ExtensionReg, ExtensionRegOperations, Reg};
use crate::memory::map::MapMemory;
use crate::peripheral::{dwt::Dwt, systick::SysTick};
use crate::semihosting::{decode_semihostcmd, semihost_return};
use crate::Processor;

mod branch;
mod divide;
mod load_and_store;
mod misc;
mod multiply;
mod packing;
mod parallel_add;
mod shift;
mod signed_multiply;
mod status_register;
mod std_data_processing;
use crate::executor::multiply::IsaMultiply;
use crate::executor::shift::IsaShift;
use crate::executor::signed_multiply::IsaSignedMultiply;
use crate::executor::std_data_processing::IsaStandardDataProcessing;
use branch::IsaBranch;
use divide::IsaDivide;
use load_and_store::IsaLoadAndStore;
use misc::IsaMisc;
use packing::IsaPacking;
use parallel_add::IsaParallelAddSub;
use status_register::IsaStatusRegister;
///
/// Stepping processor with instructions
///
pub trait Executor {
    ///
    /// Run processor forward. Simulates core + peripherals.
    ///
    fn step(&mut self);

    ///
    /// Run processor forward with core sleeping (peripherals only)
    ///
    fn step_sleep(&mut self);

    ///
    /// Execute given instruction. Returns number of clock cycles burn.
    ///
    fn execute(&mut self, instruction: &Instruction, instruction_size: usize) -> u32;
}

type ExecuteResult = Result<ExecuteSuccess, Fault>;
#[derive(PartialEq, Debug, Copy, Clone)]
/// Succesfull execution  an instruction
pub enum ExecuteSuccess {
    /// The instruction was taken normally
    Taken {
        /// Number of clock cycles taken for the operation
        cycles: u32,
    },
    /// The instruction was not taken as the condition did not pass
    NotTaken,
    /// The execution branched to a new address, pc was set accordingly
    Branched {
        /// Number of clock cycles taken for the operation
        cycles: u32,
    },
}

trait ExecutorHelper {
    fn condition_passed(&mut self) -> bool;
    fn condition_passed_b(&mut self, cond: Condition) -> bool;
    fn integer_zero_divide_trapping_enabled(&mut self) -> bool;
    fn set_itstate(&mut self, state: u8);
    fn it_advance(&mut self);
    fn in_it_block(&self) -> bool;
    fn last_in_it_block(&self) -> bool;
    fn execute_internal(&mut self, instruction: &Instruction) -> ExecuteResult;
    fn update_flags_check_it_block(
        &mut self,
        setflags: SetFlags,
        result: u32,
        carry: bool,
        overflow: bool,
    );
}

#[inline(always)]
fn resolve_addressing(rn: u32, imm32: u32, add: bool, index: bool) -> (u32, u32) {
    let offset_address = if add { rn + imm32 } else { rn - imm32 };
    let address = if index { offset_address } else { rn };
    (address, offset_address)
}

fn expand_conditional_carry(imm32: &Imm32Carry, carry: bool) -> (u32, bool) {
    match imm32 {
        Imm32Carry::NoCarry { imm32 } => (*imm32, carry),
        Imm32Carry::Carry { imm32_c0, imm32_c1 } => {
            if carry {
                *imm32_c1
            } else {
                *imm32_c0
            }
        }
    }
}

fn conditional_setflags(setflags: SetFlags, in_it_block: bool) -> bool {
    match setflags {
        SetFlags::True => true,
        SetFlags::False => false,
        SetFlags::NotInITBlock => !in_it_block,
    }
}

impl ExecutorHelper for Processor {
    fn set_itstate(&mut self, state: u8) {
        self.itstate = state;
    }

    fn it_advance(&mut self) {
        if self.itstate != 0 {
            if self.itstate.get_bits(0..3) == 0 {
                self.itstate = 0;
            } else {
                let it = self.itstate.get_bits(0..5);
                self.itstate.set_bits(0..5, (it << 1) & 0b11111);
            }
        }
    }

    fn in_it_block(&self) -> bool {
        self.itstate.get_bits(0..4) != 0
    }

    fn last_in_it_block(&self) -> bool {
        self.itstate.get_bits(0..4) == 0b1000
    }
    fn integer_zero_divide_trapping_enabled(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn condition_passed(&mut self) -> bool {
        let itstate = self.itstate;

        if itstate == 0 {
            true
        } else {
            let cond = u16::from(itstate.get_bits(4..8));
            condition_test(
                Condition::from_u16(cond).unwrap_or(Condition::AL),
                &self.psr,
            )
        }
    }

    fn condition_passed_b(&mut self, cond: Condition) -> bool {
        condition_test(cond, &self.psr)
    }

    fn update_flags_check_it_block(
        &mut self,
        setflags: SetFlags,
        result: u32,
        carry: bool,
        overflow: bool,
    ) {
        if conditional_setflags(setflags, self.in_it_block()) {
            self.psr.set_n(result);
            self.psr.set_z(result);
            self.psr.set_c(carry);
            self.psr.set_v(overflow);
        }
    }

    #[allow(unused_variables)]
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)]
    fn execute_internal(&mut self, instruction: &Instruction) -> ExecuteResult {
        match instruction {
            // --------------------------------------------
            //
            // Group: Branch instructions
            //
            // --------------------------------------------
            Instruction::B_t13 { params, .. } => self.exec_b_t13(*params),
            Instruction::B_t24 { imm32, .. } => self.exec_b_t24(*imm32),

            Instruction::BLX { rm } => self.exec_blx(*rm),
            Instruction::BX { rm } => self.exec_bx(*rm),
            Instruction::BL { imm32 } => self.exec_bl(*imm32),

            Instruction::CBZ { params } => self.exec_cbz(*params),
            Instruction::CBNZ { params } => self.exec_cbnz(*params),

            Instruction::TBB { params } => self.exec_tbb(*params),
            Instruction::TBH { params } => self.exec_tbh(*params),

            // --------------------------------------------
            //
            // Group: Standard data-processing instructions
            //
            // --------------------------------------------
            Instruction::ADD_reg { params, .. } => self.exec_add_reg(params),
            Instruction::ADD_imm { params, .. } => self.exec_add_imm(params),
            Instruction::ADD_sp_reg { params, .. } => self.exec_add_sp_reg(params),

            Instruction::ADC_reg { params, .. } => self.exec_adc_reg(params),
            Instruction::ADC_imm { params } => self.exec_adc_imm(params),

            Instruction::ADR { params, .. } => self.exec_adr(*params),

            Instruction::AND_reg { params, .. } => self.exec_and_reg(params),
            Instruction::AND_imm { params } => self.exec_and_imm(params),

            Instruction::BIC_imm { params } => self.exec_bic_imm(params),
            Instruction::BIC_reg { params, .. } => self.exec_bic_reg(params),

            Instruction::CMN_reg { params, .. } => self.exec_cmn_reg(params),
            Instruction::CMN_imm { params } => self.exec_cmn_imm(*params),

            Instruction::CMP_reg { params, .. } => self.exec_cmp_reg(params),
            Instruction::CMP_imm { params, .. } => self.exec_cmp_imm(*params),

            Instruction::EOR_reg { params, .. } => self.exec_eor_reg(params),
            Instruction::EOR_imm { params } => self.exec_eor_imm(params),

            Instruction::MOV_reg { params, .. } => self.exec_mov_reg(params),
            Instruction::MOV_imm { params, .. } => self.exec_mov_imm(params),

            Instruction::MVN_reg { params, .. } => self.exec_mvn_reg(params),
            Instruction::MVN_imm { params } => self.exec_mvn_imm(params),

            Instruction::ORN_reg { params } => self.exec_orn_reg(params),

            Instruction::ORR_imm { params } => self.exec_orr_imm(params),
            Instruction::ORR_reg { params, .. } => self.exec_orr_reg(params),

            Instruction::RSB_reg { params, .. } => self.exec_rsb_reg(params),
            Instruction::RSB_imm { params, .. } => self.exec_rsb_imm(params),

            Instruction::SBC_reg { params, .. } => self.exec_sbc_reg(params),
            Instruction::SBC_imm { params } => self.exec_sbc_imm(params),

            Instruction::SUB_reg { params, .. } => self.exec_sub_reg(params),
            Instruction::SUB_imm { params, .. } => self.exec_sub_imm(params),

            Instruction::TEQ_reg { params } => self.exec_teq_reg(params),
            Instruction::TEQ_imm { params } => self.exec_teq_imm(params),

            Instruction::TST_reg { params, .. } => self.exec_tst_reg(params),
            Instruction::TST_imm { params } => self.exec_tst_imm(params),

            // --------------------------------------------
            //
            // Group: Shift instructions
            //
            // --------------------------------------------
            Instruction::ASR_imm { params, .. } => self.exec_asr_imm(params),
            Instruction::ASR_reg { params, thumb32 } => self.exec_asr_reg(params),

            Instruction::LSL_imm { params, thumb32 } => self.exec_lsl_imm(params),
            Instruction::LSL_reg { params, thumb32 } => self.exec_lsl_reg(params),

            Instruction::LSR_imm { params, thumb32 } => self.exec_lsr_imm(params),
            Instruction::LSR_reg { params, thumb32 } => self.exec_lsr_reg(params),

            Instruction::ROR_imm { params } => self.exec_ror_imm(params),
            Instruction::ROR_reg { params, .. } => self.exec_ror_reg(params),

            Instruction::RRX { params } => self.exec_rrx(params),
            // --------------------------------------------
            //
            // Group: Multiply instructions
            //
            // --------------------------------------------
            Instruction::MLA { params } => self.exec_mla(params),
            Instruction::MLS { params } => self.exec_mls(params),
            Instruction::MUL { params, .. } => self.exec_mul(params),
            // --------------------------------------------
            //
            // Group: Signed multiply instructions (ArmV7-m)
            //
            // --------------------------------------------
            Instruction::SMLAL { params } => unimplemented!(),
            Instruction::SMULL { params } => self.exec_smull(params),

            // --------------------------------------------
            //
            // Group: Unsigned Multiply instructions (ARMv7-M base architecture)
            //
            // --------------------------------------------
            Instruction::UMLAL { params } => self.exec_umlal(params),
            Instruction::UMULL { params } => self.exec_umull(params),

            // --------------------------------------------
            //
            // Group: Signed Multiply instructions (ARMv7-M DSP extension)
            //
            // --------------------------------------------
            Instruction::SMUL { params } => self.exec_smul(params),
            Instruction::SMLA { params } => self.exec_smla(params),

            // --------------------------------------------
            //
            // Group: Saturating instructions (ARMv7-M base arch)
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Saturating instructions (ARMv7-M DSP extensions)
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Saturating add/sub (ARMv7-M DSP extensions)
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Packing and unpacking instructions
            //
            // --------------------------------------------
            Instruction::SXTB { params, .. } => self.exec_sxtb(params),
            Instruction::SXTH { params, .. } => self.exec_sxth(params),

            Instruction::UXTB { params, .. } => self.exec_uxtb(params),
            Instruction::UXTH { params, .. } => self.exec_uxth(params),

            // --------------------------------------------
            //
            // Group: Packing and unpacking instructions (DSP extensions)
            //
            // --------------------------------------------
            Instruction::UXTAB { params } => self.exec_uxtab(params),

            // --------------------------------------------
            //
            // Group: Divide instructions
            //
            // --------------------------------------------
            // ARMv7-M
            Instruction::SDIV { params } => self.exec_sdiv(params),
            Instruction::UDIV { params } => self.exec_udiv(params),

            // --------------------------------------------
            //
            // Group: Parallel add / sub (DSP extension)
            //
            // --------------------------------------------
            Instruction::UADD8 { params } => self.exec_uadd8(params),

            // --------------------------------------------
            //
            // Group:  Miscellaneous data-processing instructions
            //
            // --------------------------------------------
            Instruction::BFC { params } => self.exec_bfc(params),
            Instruction::BFI { params } => self.exec_bfi(params),

            Instruction::CLZ { params } => self.exec_clz(*params),

            Instruction::MOVT { params } => self.exec_movt(*params),

            Instruction::REV { params, .. } => self.exec_rev(*params),
            Instruction::REV16 { params, .. } => self.exec_rev16(*params),
            Instruction::REVSH { params, .. } => self.exec_revsh(*params),

            Instruction::UBFX { params } => self.exec_ubfx(params),

            // --------------------------------------------
            //
            // Group:  Miscellaneous data-processing instructions (DSP extensions)
            //
            // --------------------------------------------
            Instruction::SEL { params } => self.exec_sel(params),
            // --------------------------------------------
            //
            // Group: Status register access instructions
            //
            // --------------------------------------------
            Instruction::MRS { params } => self.exec_mrs(*params),
            Instruction::MSR_reg { params } => self.exec_msr(*params),

            #[cfg(armv6m)]
            Instruction::CPS { im } => self.exec_cps(*im),

            #[cfg(any(armv7m, armv7em))]
            Instruction::CPS {
                im,
                affect_pri,
                affect_fault,
            } => self.exec_cps(*im, *affect_pri, *affect_fault),

            // --------------------------------------------
            //
            // Group:  Load and Store instructions
            //
            // --------------------------------------------
            Instruction::LDR_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let data = self.read32(address)?;
                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    if rt == &Reg::PC {
                        self.load_write_pc(data)?;
                        return Ok(ExecuteSuccess::Branched { cycles: 2 });
                    } else {
                        self.set_r(*rt, data);
                        return Ok(ExecuteSuccess::Taken { cycles: 2 });
                    }
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDREX { rt, rn, imm32 } => {
                if self.condition_passed() {
                    let (address, _) = resolve_addressing(self.get_r(*rn), *imm32, true, true);

                    self.set_exclusive_monitors(address, 4);

                    let data = self.read32(address)?;
                    self.set_r(*rt, data);

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDREXB { rt, rn } => {
                if self.condition_passed() {
                    let address = self.get_r(*rn);
                    self.set_exclusive_monitors(address, 1);

                    let data = self.read8(address)?;

                    let params = [data];
                    let lengths = [32];
                    self.set_r(*rt, zero_extend(&params, &lengths));

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDREXH { rt, rn } => {
                if self.condition_passed() {
                    let address = self.get_r(*rn);
                    self.set_exclusive_monitors(address, 2);

                    let data = self.read16(address)?;

                    let params = [data];
                    let lengths = [32];
                    self.set_r(*rt, zero_extend_u16(&params, &lengths));

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDRSH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let data = self.read16(address)?;
                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    self.set_r(*rt, sign_extend(u32::from(data), 15, 32) as u32);
                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDRSB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let data = self.read8(address)?;
                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    self.set_r(*rt, sign_extend(data.into(), 7, 32) as u32);
                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDR_reg { params, thumb32 } => self.exec_ldr_reg(params),
            Instruction::LDRB_reg { params, thumb32 } => self.exec_ldrb_reg(params),
            Instruction::LDRH_reg { params, thumb32 } => self.exec_ldrh_reg(params),
            Instruction::LDRSH_reg { params, thumb32 } => self.exec_ldrsh_reg(params),
            Instruction::LDRSB_reg { params, thumb32 } => self.exec_ldrsb_reg(params),

            Instruction::LDRB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let data = self.read8(address)?;
                    self.set_r(*rt, u32::from(data));

                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }


            Instruction::LDRH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let data = self.read16(address)?;
                    if *wback {
                        self.set_r(*rn, offset_address);
                    }
                    self.set_r(*rt, u32::from(data));

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }


            Instruction::STR_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let value = self.get_r(*rt);
                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    self.write32(address, value)?;

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STREX { rd, rt, rn, imm32 } => {
                if self.condition_passed() {
                    let (address, _) = resolve_addressing(self.get_r(*rn), *imm32, true, true);

                    if self.exclusive_monitors_pass(address, 4) {
                        self.write32(address, self.get_r(*rt))?;
                        self.set_r(*rd, 0);
                    } else {
                        self.set_r(*rd, 1);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STREXB { rd, rt, rn } => {
                if self.condition_passed() {
                    let address = self.get_r(*rn);

                    if self.exclusive_monitors_pass(address, 1) {
                        self.write8(address, self.get_r(*rt) as u8)?;
                        self.set_r(*rd, 0);
                    } else {
                        self.set_r(*rd, 1);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STREXH { rd, rt, rn } => {
                if self.condition_passed() {
                    let address = self.get_r(*rn);

                    if self.exclusive_monitors_pass(address, 2) {
                        self.write16(address, self.get_r(*rt) as u16)?;
                        self.set_r(*rd, 0);
                    } else {
                        self.set_r(*rd, 1);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STRD_imm {
                rt,
                rt2,
                rn,
                imm32,
                index,
                add,
                wback,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let value1 = self.get_r(*rt);
                    self.write32(address, value1)?;
                    let value2 = self.get_r(*rt2);
                    self.write32(address + 4, value2)?;

                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDRD_imm {
                rt,
                rt2,
                rn,
                imm32,
                index,
                add,
                wback,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let data = self.read32(address)?;
                    self.set_r(*rt, data);
                    let data2 = self.read32(address + 4)?;
                    self.set_r(*rt2, data2);

                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STR_reg { params, thumb32 } => {
                if self.condition_passed() {
                    let c = self.psr.get_c();
                    let offset = shift(
                        self.get_r(params.rm),
                        params.shift_t,
                        params.shift_n as usize,
                        c,
                    );
                    let address = self.get_r(params.rn) + offset;
                    let value = self.get_r(params.rt);
                    self.write32(address, value)?;

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STRB_reg { params, thumb32 } => {
                if self.condition_passed() {
                    let c = self.psr.get_c();
                    let offset = shift(
                        self.get_r(params.rm),
                        params.shift_t,
                        params.shift_n as usize,
                        c,
                    );
                    let address = self.get_r(params.rn) + offset;
                    let rt: u32 = self.get_r(params.rt);
                    let value = rt.get_bits(0..8);
                    self.write8(address, value as u8)?;
                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STRB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let value = self.get_r(*rt);
                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    self.write8(address, value.get_bits(0..8) as u8)?;

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STRH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let (address, offset_address) =
                        resolve_addressing(self.get_r(*rn), *imm32, *add, *index);

                    let value = self.get_r(*rt);
                    self.write16(address, value.get_bits(0..16) as u16)?;

                    if *wback {
                        self.set_r(*rn, offset_address);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STRH_reg { params, thumb32 } => {
                if self.condition_passed() {
                    let c = self.psr.get_c();
                    let offset = shift(
                        self.get_r(params.rm),
                        params.shift_t,
                        params.shift_n as usize,
                        c,
                    );
                    let address = self.get_r(params.rn) + offset;
                    let value = self.get_r(params.rt).get_bits(0..16);
                    self.write16(address, value as u16)?;
                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDR_lit {
                rt,
                imm32,
                add,
                thumb32,
            } => {
                if self.condition_passed() {
                    let base = self.get_r(Reg::PC) & 0xffff_fffc;
                    let address = if *add { base + imm32 } else { base - imm32 };
                    let data = self.read32(address)?;

                    if rt == &Reg::PC {
                        self.load_write_pc(data)?;
                    } else {
                        self.set_r(*rt, data);
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 2 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            // --------------------------------------------
            //
            // Group:  Load and Store Multiple instructions
            //
            // --------------------------------------------
            Instruction::STM {
                registers,
                rn,
                wback,
                thumb32,
            } => {
                if self.condition_passed() {
                    let regs_size = 4 * (registers.len() as u32);

                    let mut address = self.get_r(*rn);

                    for reg in registers.iter() {
                        let r = self.get_r(reg);
                        self.write32(address, r)?;
                        address += 4;
                    }

                    if *wback {
                        self.add_r(*rn, regs_size);
                    }
                    return Ok(ExecuteSuccess::Taken {
                        cycles: 1 + registers.len() as u32,
                    });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::STMDB {
                registers,
                rn,
                wback,
            } => {
                if self.condition_passed() {
                    let regs_size = 4 * (registers.len() as u32);

                    let mut address = self.get_r(*rn) - regs_size;

                    for reg in registers.iter() {
                        let r = self.get_r(reg);
                        self.write32(address, r)?;
                        address += 4;
                    }

                    if *wback {
                        self.sub_r(*rn, regs_size);
                    }
                    return Ok(ExecuteSuccess::Taken {
                        cycles: 1 + registers.len() as u32,
                    });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::LDM {
                registers,
                rn,
                thumb32,
            } => {
                if self.condition_passed() {
                    let regs_size = 4 * (registers.len() as u32);

                    let mut address = self.get_r(*rn);

                    let mut branched = false;
                    for reg in registers.iter() {
                        let value = self.read32(address)?;
                        if reg == Reg::PC {
                            self.load_write_pc(value)?;
                            branched = true;
                        } else {
                            self.set_r(reg, value);
                        }
                        address += 4;
                    }

                    if !registers.contains(rn) {
                        self.add_r(*rn, regs_size);
                    }
                    let cc = 1 + registers.len() as u32;
                    if branched {
                        return Ok(ExecuteSuccess::Branched { cycles: cc });
                    }
                    return Ok(ExecuteSuccess::Taken { cycles: cc });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::PUSH { registers, thumb32 } => {
                if self.condition_passed() {
                    let regs_size = 4 * (registers.len() as u32);
                    let sp = self.get_r(Reg::SP);
                    let mut address = sp - regs_size;

                    for reg in registers.iter() {
                        let value = self.get_r(reg);
                        self.write32(address, value)?;
                        address += 4;
                    }

                    self.set_r(Reg::SP, sp - regs_size);
                    return Ok(ExecuteSuccess::Taken {
                        cycles: 1 + registers.len() as u32,
                    });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::POP { registers, thumb32 } => {
                if self.condition_passed() {
                    let regs_size = 4 * (registers.len() as u32);
                    let sp = self.get_r(Reg::SP);
                    let mut address = sp;

                    self.set_r(Reg::SP, sp + regs_size);

                    for reg in registers.iter() {
                        let val = self.read32(address)?;
                        if reg == Reg::PC {
                            self.bx_write_pc(val)?;
                        } else {
                            self.set_r(reg, val);
                        }
                        address += 4;
                    }

                    if registers.contains(&Reg::PC) {
                        return Ok(ExecuteSuccess::Branched {
                            cycles: 4 + registers.len() as u32,
                        });
                    } else {
                        return Ok(ExecuteSuccess::Taken {
                            cycles: 1 + registers.len() as u32,
                        });
                    }
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            // --------------------------------------------
            //
            // Group: Miscellaneous
            //
            // --------------------------------------------
            Instruction::DMB | Instruction::DSB | Instruction::ISB => {
                if self.condition_passed() {
                    return Ok(ExecuteSuccess::Taken { cycles: 4 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::IT {
                x,
                y,
                z,
                firstcond,
                mask,
            } => {
                self.set_itstate((((firstcond.value() as u32) << 4) + u32::from(*mask)) as u8);
                Ok(ExecuteSuccess::Taken { cycles: 4 })
            }

            Instruction::NOP { .. } => Ok(ExecuteSuccess::Taken { cycles: 1 }),

            Instruction::PLD_imm { rn, imm32, add } => {
                if self.condition_passed() {
                    Ok(ExecuteSuccess::Taken { cycles: 1 })
                } else {
                    Ok(ExecuteSuccess::NotTaken)
                }
            }

            Instruction::PLD_lit { imm32, add } => {
                if self.condition_passed() {
                    Ok(ExecuteSuccess::Taken { cycles: 1 })
                } else {
                    Ok(ExecuteSuccess::NotTaken)
                }
            }

            Instruction::PLD_reg {
                rn,
                rm,
                shift_t,
                shift_n,
            } => {
                if self.condition_passed() {
                    Ok(ExecuteSuccess::Taken { cycles: 1 })
                } else {
                    Ok(ExecuteSuccess::NotTaken)
                }
            }

            Instruction::SEV { .. } => {
                if self.condition_passed() {
                    println!("SEV");
                    return Ok(ExecuteSuccess::Taken { cycles: 1 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::WFE { .. } | Instruction::YIELD { .. } => {
                if self.condition_passed() {
                    //TODO
                    return Ok(ExecuteSuccess::Taken { cycles: 1 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::WFI { .. } => {
                if self.condition_passed() {
                    if self.get_pending_exception() == None {
                        self.state.set_bit(1, true); // sleeping == true
                    }
                    return Ok(ExecuteSuccess::Taken { cycles: 1 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            // --------------------------------------------
            //
            // Group: Exception generating instructions
            //
            // --------------------------------------------
            Instruction::SVC { imm32 } => {
                if self.condition_passed() {
                    println!("SVC {}", imm32);
                    return Ok(ExecuteSuccess::Taken { cycles: 1 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::BKPT { imm32 } => {
                if *imm32 == 0xab {
                    let r0 = self.get_r(Reg::R0);
                    let r1 = self.get_r(Reg::R1);
                    let semihost_cmd = decode_semihostcmd(r0, r1, self)?;

                    if let Some(sh_func) = &mut self.semihost_func {
                        let semihost_response = (sh_func)(&semihost_cmd);
                        semihost_return(self, &semihost_response);
                    }
                }
                Ok(ExecuteSuccess::Taken { cycles: 1 })
            }

            // --------------------------------------------
            //
            // Group: Coprocessor instructions
            //
            // --------------------------------------------
            // ARMv7-M
            Instruction::MCR {
                rt,
                coproc,
                opc1,
                opc2,
                crn,
                crm,
            } => unimplemented!(),

            // ARMv7-M
            Instruction::MCR2 {
                rt,
                coproc,
                opc1,
                opc2,
                crn,
                crm,
            } => unimplemented!(),

            // ARMv7-M
            Instruction::LDC_imm {
                coproc,
                imm32,
                crd,
                rn,
            } => unimplemented!(),

            // ARMv7-M
            Instruction::LDC2_imm {
                coproc,
                imm32,
                crd,
                rn,
            } => unimplemented!(),
            // --------------------------------------------
            //
            // Group: Floating-point load and store instructions
            //
            // --------------------------------------------
            Instruction::VLDR {
                dd,
                rn,
                add,
                imm32,
                single_reg,
            } => {
                if self.condition_passed() {
                    //self.execute_fp_check();

                    let base = match *rn {
                        Reg::PC => self.get_r(Reg::PC) & 0xffff_fffc,
                        _ => self.get_r(*rn),
                    };

                    let address = if *add { base + imm32 } else { base - imm32 };
                    match *dd {
                        ExtensionReg::Single { reg } => {
                            let data = self.read32(address)?;
                            self.set_sr(reg, data);
                        }
                        ExtensionReg::Double { reg } => {
                            let word1 = self.read32(address)?;
                            let word2 = self.read32(address + 4)?;
                            self.set_dr(reg, word1, word2);
                        }
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 1 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            Instruction::VSTR {
                dd,
                rn,
                add,
                imm32,
                single_reg,
            } => {
                if self.condition_passed() {
                    //self.execute_fp_check();

                    let base = self.get_r(*rn);

                    let address = if *add { base + imm32 } else { base - imm32 };
                    match *dd {
                        ExtensionReg::Single { reg } => {
                            let value = self.get_sr(reg);
                            self.write32(address, value)?;
                        }
                        ExtensionReg::Double { reg } => {
                            let (low_word, high_word) = self.get_dr(reg);
                            self.write32(address, low_word)?;
                            self.write32(address + 4, high_word)?;
                        }
                    }

                    return Ok(ExecuteSuccess::Taken { cycles: 1 });
                }
                Ok(ExecuteSuccess::NotTaken)
            }

            // --------------------------------------------
            //
            // Group: Floating-point register transfer instructions
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Floating-point data-processing instructions
            //
            // --------------------------------------------
            Instruction::UDF { imm32, opcode, .. } => {
                println!("UDF {}, {}", imm32, opcode);
                todo!("undefined");
                //Some(Fault::UndefinedInstruction)
            }
        }
    }
}

impl Executor for Processor {
    #[inline(always)]
    fn step_sleep(&mut self) {
        self.syst_step(1);
        self.check_exceptions();
        self.dwt_tick(1);
    }

    #[inline(always)]
    fn step(&mut self) {
        let pc = self.get_pc();
        let mapped_pc = (self.map_address(pc) >> 1) as usize;
        let (instruction, instruction_size) = self.instruction_cache[mapped_pc];
        let count = self.execute(&instruction, instruction_size);
        self.cycle_count += u64::from(count);
        self.dwt_tick(count);
        self.syst_step(count);
        self.check_exceptions();
        //TODO exception entry also burns cycles that should be accounted for
        //DWT and SYST ticking
    }

    #[inline(always)]
    fn execute(&mut self, instruction: &Instruction, instruction_size: usize) -> u32 {
        self.instruction_count += 1;

        let in_it_block = self.in_it_block();

        match self.execute_internal(&instruction) {
            Err(_fault) => {
                // all faults are mapped to hardfaults on armv6m
                let new_pc = self.get_pc();

                //TODO: map to correct exception
                //TODO: cycles not correctly accumulated yet for exception entry
                self.exception_entry(Exception::HardFault, new_pc)
                    .expect("error handling on exception entry not implemented");
                //TODO: proper amount of cycles calcuation
                12
            }
            Ok(ExecuteSuccess::NotTaken) => {
                self.add_pc(instruction_size as u32);
                if in_it_block {
                    self.it_advance();
                }
                1
            }
            Ok(ExecuteSuccess::Branched { cycles }) => cycles,
            Ok(ExecuteSuccess::Taken { cycles }) => {
                self.add_pc(instruction_size as u32);

                if in_it_block {
                    self.it_advance();
                }
                cycles
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::condition::Condition;
    use crate::core::instruction::instruction_size;
    use crate::core::instruction::{
        BfiParams, CondBranchParams, ITCondition, Reg2ShiftNoSetFlagsParams, Reg3NoSetFlagsParams,
        Reg3ShiftParams, Reg4HighParams, Reg4NoSetFlagsParams, RegImmCarryParams, SRType, SetFlags,
    };

    #[test]
    fn test_udiv() {
        // arrange
        let mut core = Processor::new();
        core.set_r(Reg::R0, 0x7d0);
        core.set_r(Reg::R1, 0x3);
        core.psr.value = 0;

        let instruction = Instruction::UDIV {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R1,
            },
        };

        // act
        let result = core.execute_internal(&instruction);

        assert_eq!(result, Ok(ExecuteSuccess::Taken { cycles: 2 }));

        assert_eq!(core.get_r(Reg::R0), 0x29a);
        assert_eq!(core.get_r(Reg::R1), 0x3);
    }

    #[test]
    fn test_mla() {
        // arrange
        let mut core = Processor::new();
        core.set_r(Reg::R7, 0x2);
        core.set_r(Reg::R2, 0x29a);
        core.set_r(Reg::R1, 0x2000089C);
        core.psr.value = 0;

        let instruction = Instruction::MLA {
            params: Reg4NoSetFlagsParams {
                rd: Reg::R1,
                rn: Reg::R7,
                rm: Reg::R2,
                ra: Reg::R1,
            },
        };

        // act
        let result = core.execute_internal(&instruction);

        assert_eq!(result, Ok(ExecuteSuccess::Taken { cycles: 2 }));

        assert_eq!(core.get_r(Reg::R1), 0x20000DD0);
    }

    #[test]
    fn test_it_block() {
        // arrange
        let mut core = Processor::new();
        core.set_r(Reg::R5, 0x49);
        core.set_r(Reg::R4, 0x01);
        core.set_r(Reg::R0, 0x49);
        core.psr.value = 0;

        let i1 = Instruction::CMP_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R0,
                rm: Reg::R5,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: false,
        };

        let i2 = Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::NE,
            mask: 0b1000,
        };
        let i3 = Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R4,
                imm32: Imm32Carry::NoCarry { imm32: 0 },
                setflags: SetFlags::False,
            },
            thumb32: false,
        };

        core.execute(&i1, instruction_size(&i1));
        core.execute(&i2, instruction_size(&i1));
        core.execute(&i3, instruction_size(&i1));

        assert_eq!(core.get_r(Reg::R4), 0x01);
        assert!(!core.in_it_block());
    }

    #[test]
    fn test_b_cond() {
        // arrange
        let mut core = Processor::new();
        core.psr.value = 0;

        let instruction = Instruction::B_t13 {
            params: CondBranchParams {
                cond: Condition::EQ,
                imm32: 0,
            },
            thumb32: true,
        };

        // act
        let result = core.execute_internal(&instruction);

        assert_eq!(result, Ok(ExecuteSuccess::NotTaken));
    }

    #[test]
    fn test_bfi() {
        // arrange
        let mut core = Processor::new();
        core.psr.value = 0;

        core.set_r(Reg::R2, 0x11223344);
        core.set_r(Reg::R3, 0xaabbccdd);
        core.psr.value = 0;

        let instruction = Instruction::BFI {
            params: BfiParams {
                rd: Reg::R2,
                rn: Reg::R3,
                lsbit: 0,
                width: 8,
            },
        };

        core.execute_internal(&instruction).unwrap();

        assert_eq!(core.get_r(Reg::R3), 0xaabbccdd);
        assert_eq!(core.get_r(Reg::R2), 0x112233dd);
    }
    #[test]
    fn test_bfi_with_shift_8() {
        // arrange
        let mut core = Processor::new();
        core.psr.value = 0;

        core.set_r(Reg::R0, 0);
        core.set_r(Reg::R1, 0x00e000e4);

        let instruction = Instruction::BFI {
            params: BfiParams {
                rd: Reg::R0,
                rn: Reg::R1,
                lsbit: 8,
                width: 24,
            },
        };

        core.execute_internal(&instruction).unwrap();

        assert_eq!(core.get_r(Reg::R0), 0xe000e400);
        assert_eq!(core.get_r(Reg::R1), 0x00e000e4);
    }

    #[test]
    fn test_sub() {
        // arrange
        let mut core = Processor::new();
        core.psr.value = 0;

        //3:418415f7 4:00000418 5:80000000 6:7d17d411
        core.set_r(Reg::R3, 0x418415f7);
        core.set_r(Reg::R4, 0x00000418);
        core.psr.value = 0;

        let instruction = Instruction::SUB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R6,
                rn: Reg::R4,
                rm: Reg::R3,
                setflags: SetFlags::False,
                shift_t: SRType::LSR,
                shift_n: 20,
            },
            thumb32: true,
        };

        core.execute_internal(&instruction).unwrap();

        assert_eq!(core.get_r(Reg::R6), 0);
    }

    #[test]
    fn test_smlabb() {
        // arrange

        //itm_file, &code, semihost_func
        let mut core = Processor::new();
        core.psr.value = 0;

        //
        core.set_r(Reg::R8, 0xffff9d88);
        core.set_r(Reg::R12, 0x0012dfc3);
        core.set_r(Reg::LR, 0xa1);
        core.psr.value = 0;

        let instruction = Instruction::SMLA {
            params: Reg4HighParams {
                rd: Reg::R12,
                rn: Reg::LR,
                rm: Reg::R8,
                ra: Reg::R12,
                n_high: false,
                m_high: false,
            },
        };

        core.execute_internal(&instruction).unwrap();

        assert_eq!(core.get_r(Reg::R12), 0xFFD4F24B);
    }
}
