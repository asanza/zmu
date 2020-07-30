use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::{resolve_addressing, ExecuteResult};

use crate::{
    bus::Bus,
    core::{
        bits::Bits,
        instruction::{
            Reg2FullParams, Reg2RtRnImm32Params, Reg2RtRnParams, Reg3FullParams,
            Reg3RdRtRnImm32Params, Reg3RdRtRnParams,
        },
        monitor::Monitor,
        operation::{shift, sign_extend, zero_extend, zero_extend_u16},
        register::{Apsr, BaseReg, Reg},
    },
};

/// Load and Store operations
pub trait IsaLoadAndStore {
    fn exec_ldr_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrsb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrsh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;

    fn exec_str_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_strb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_strh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;

    fn exec_ldr_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;
    fn exec_ldrb_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;
    fn exec_ldrh_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;
    fn exec_ldrsb_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;
    fn exec_ldrsh_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;

    fn exec_str_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;
    fn exec_strb_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;
    fn exec_strh_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult;

    fn exec_ldrex(&mut self, params: Reg2RtRnImm32Params) -> ExecuteResult;
    fn exec_ldrexb(&mut self, params: Reg2RtRnParams) -> ExecuteResult;
    fn exec_ldrexh(&mut self, params: Reg2RtRnParams) -> ExecuteResult;

    fn exec_strex(&mut self, params: Reg3RdRtRnImm32Params) -> ExecuteResult;
    fn exec_strexb(&mut self, params: Reg3RdRtRnParams) -> ExecuteResult;
    fn exec_strexh(&mut self, params: Reg3RdRtRnParams) -> ExecuteResult;
}

impl IsaLoadAndStore for Processor {
    fn exec_ldr_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let rn = self.get_r(params.rn);
            let (address, offset_address) =
                resolve_addressing(rn, offset, params.add, params.index);

            let data = self.read32(address)?;
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            if params.rt == Reg::PC {
                self.load_write_pc(data)?;
                return Ok(ExecuteSuccess::Branched { cycles: 2 });
            } else {
                self.set_r(params.rt, data);
                return Ok(ExecuteSuccess::Taken { cycles: 2 });
            }
        }

        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();

            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read8(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, data);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read16(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, data);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrsh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read16(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, sign_extend(data, 15, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrsb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read8(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, sign_extend(data, 7, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_str_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
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

    fn exec_strb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
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

    fn exec_strh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
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

    fn exec_ldr_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let data = self.read32(address)?;
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            if params.rt == Reg::PC {
                self.load_write_pc(data)?;
                return Ok(ExecuteSuccess::Branched { cycles: 2 });
            } else {
                self.set_r(params.rt, data);
                return Ok(ExecuteSuccess::Taken { cycles: 2 });
            }
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_ldrb_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let data = self.read8(address)?;
            self.set_r(params.rt, u32::from(data));

            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_ldrh_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let data = self.read16(address)?;
            if params.wback {
                self.set_r(params.rn, offset_address);
            }
            self.set_r(params.rt, u32::from(data));

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrsb_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let data = self.read8(address)?;
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, sign_extend(data.into(), 7, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_ldrsh_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let data = self.read16(address)?;
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, sign_extend(u32::from(data), 15, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_str_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let value = self.get_r(params.rt);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.write32(address, value)?;

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_strb_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let value = self.get_r(params.rt);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.write8(address, value.get_bits(0..8) as u8)?;

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_strh_imm(&mut self, params: &Reg2FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let (address, offset_address) = resolve_addressing(
                self.get_r(params.rn),
                params.imm32,
                params.add,
                params.index,
            );

            let value = self.get_r(params.rt);
            self.write16(address, value.get_bits(0..16) as u16)?;

            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrex(&mut self, params: Reg2RtRnImm32Params) -> ExecuteResult {
        if self.condition_passed() {
            let (address, _) = resolve_addressing(self.get_r(params.rn), params.imm32, true, true);

            self.set_exclusive_monitors(address, 4);

            let data = self.read32(address)?;
            self.set_r(params.rt, data);

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrexb(&mut self, params: Reg2RtRnParams) -> ExecuteResult {
        if self.condition_passed() {
            let address = self.get_r(params.rn);
            self.set_exclusive_monitors(address, 1);

            let data = self.read8(address)?;

            let data_params = [data];
            let lengths = [32];
            self.set_r(params.rt, zero_extend(&data_params, &lengths));

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrexh(&mut self, params: Reg2RtRnParams) -> ExecuteResult {
        if self.condition_passed() {
            let address = self.get_r(params.rn);
            self.set_exclusive_monitors(address, 2);

            let data = self.read16(address)?;

            let data_params = [data];
            let lengths = [32];
            self.set_r(params.rt, zero_extend_u16(&data_params, &lengths));

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_strex(&mut self, params: Reg3RdRtRnImm32Params) -> ExecuteResult {
        if self.condition_passed() {
            let (address, _) = resolve_addressing(self.get_r(params.rn), params.imm32, true, true);

            if self.exclusive_monitors_pass(address, 4) {
                self.write32(address, self.get_r(params.rt))?;
                self.set_r(params.rd, 0);
            } else {
                self.set_r(params.rd, 1);
            }

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    
    fn exec_strexb(&mut self, params: Reg3RdRtRnParams) -> ExecuteResult {
        if self.condition_passed() {
            let address = self.get_r(params.rn);

            if self.exclusive_monitors_pass(address, 1) {
                self.write8(address, self.get_r(params.rt) as u8)?;
                self.set_r(params.rd, 0);
            } else {
                self.set_r(params.rd, 1);
            }

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_strexh(&mut self, params: Reg3RdRtRnParams) -> ExecuteResult {
        if self.condition_passed() {
            let address = self.get_r(params.rn);

            if self.exclusive_monitors_pass(address, 2) {
                self.write16(address, self.get_r(params.rt) as u16)?;
                self.set_r(params.rd, 0);
            } else {
                self.set_r(params.rd, 1);
            }

            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
