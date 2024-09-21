//!
//! Processor Bus related operations
//!

use crate::Processor;

use crate::core::fault::Fault;
use crate::memory::map::MapMemory;
use crate::peripheral::dwt::Dwt;
use crate::peripheral::itm::InstrumentationTraceMacrocell;
use crate::peripheral::nvic::NVIC;
use crate::peripheral::scb::SystemControlBlock;
use crate::peripheral::systick::SysTick;

///
/// Trait for reading and writing via a memory bus.
///
pub trait Bus {
    /// Reads a 32 bit value via the bus from the given address.
    ///
    fn read32(&mut self, addr: u32) -> Result<u32, Fault>;

    /// Reads a 16 bit value via the bus from the given address.
    ///
    fn read16(&self, addr: u32) -> Result<u16, Fault>;

    /// Reads a 8 bit value via the bus from the given address.
    ///
    fn read8(&self, addr: u32) -> Result<u8, Fault>;

    /// Writes a 32 bit value to the bus targeting the given address.
    ///
    fn write32(&mut self, addr: u32, value: u32) -> Result<(), Fault>;

    /// Writes a 16 bit value to the bus targeting the given address.
    ///
    fn write16(&mut self, addr: u32, value: u16) -> Result<(), Fault>;

    /// Writes a 8 bit value to the bus targeting the given address.
    ///
    fn write8(&mut self, addr: u32, value: u8) -> Result<(), Fault>;

    /// Checks if given address can be reached via the bus.
    ///
    fn in_range(&self, addr: u32) -> bool;
}

impl Bus for Processor {
    fn read8(&self, bus_addr: u32) -> Result<u8, Fault> {
        let addr = self.map_address(bus_addr);

        let result = match addr {
            0xE000_E400..=0xE000_E5EC => {
                self.nvic_read_ipr_u8(((addr - 0xE000_E400) >> 2) as usize)
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED18..=0xE000_ED1B => self.read_shpr1_u8((addr - 0xE000_ED18) as usize),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED1C..=0xE000_ED1F => self.read_shpr2_u8((addr - 0xE000_ED1C) as usize),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED20..=0xE000_ED23 => self.read_shpr3_u8((addr - 0xE000_ED20) as usize),

            _ => {
                if self.sram.in_range(addr) {
                    return self.sram.read8(addr);
                } else if self.code.in_range(addr) {
                    return self.code.read8(addr);
                } else if self.device.in_range(addr) {
                    return self.device.read8(addr);
                } else {
                    return Err(Fault::DAccViol);
                }
            }
        };
        Ok(result)
    }

    fn read16(&self, bus_addr: u32) -> Result<u16, Fault> {
        let addr = self.map_address(bus_addr);
        match addr {
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED18..=0xE000_ED1B => {
                Ok(self.read_shpr1_u16(((addr - 0xE000_ED18) >> 1) as usize))
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED1C..=0xE000_ED1F => {
                Ok(self.read_shpr2_u16(((addr - 0xE000_ED1C) >> 1) as usize))
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED20..=0xE000_ED23 => {
                Ok(self.read_shpr3_u16(((addr - 0xE000_ED20) >> 1) as usize))
            }
            0xE000_E400..=0xE000_E5EC => {
                Ok(self.nvic_read_ipr_u16(((addr - 0xE000_E400) >> 1) as usize))
            }

            _ => {
                if self.sram.in_range(addr) {
                    self.sram.read16(addr)
                } else if self.code.in_range(addr) {
                    self.code.read16(addr)
                } else if self.device.in_range(addr) {
                    self.device.read16(addr)
                } else {
                    Err(Fault::DAccViol)
                }
            }
        }
    }

    fn read32(&mut self, bus_addr: u32) -> Result<u32, Fault> {
        let addr = self.map_address(bus_addr);

        let result = match addr {
            0xE000_0000 => self.read_stim0(),

            0xE000_1004 => self.dwt_cyccnt,

            0xE000_E004 => self.ictr,
            0xE000_E008 => self.actlr,
            0xE000_E010 => self.syst_read_csr(),
            0xE000_E014 => self.syst_read_rvr(),
            0xE000_E018 => self.syst_read_cvr(),
            0xE000_E01C => self.syst_read_calib(),
            0xE000_E100..=0xE000_E13C => self.nvic_read_iser(((addr - 0xE000_E100) >> 5) as usize),
            0xE000_E180..=0xE000_E1BC => self.nvic_read_icer(((addr - 0xE000_E180) >> 5) as usize),
            0xE000_E200..=0xE000_E23C => self.nvic_read_ispr(((addr - 0xE000_E200) >> 5) as usize),
            0xE000_E280..=0xE000_E2BC => self.nvic_read_icpr(((addr - 0xE000_E280) >> 5) as usize),
            0xE000_E300..=0xE000_E33C => self.nvic_read_iabr(((addr - 0xE000_E300) >> 5) as usize),
            0xE000_E400..=0xE000_E5EC => self.nvic_read_ipr(((addr - 0xE000_E400) >> 2) as usize),

            0xE000_ED00 => self.cpuid,
            0xE000_ED04 => self.read_icsr(),
            0xE000_ED08 => self.read_vtor(),
            0xE000_ED0C => self.aircr,
            0xE000_ED10 => self.read_scr(),
            0xE000_ED14 => self.ccr,
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED18 => self.read_shpr1(),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED1C => self.read_shpr2(),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED20 => self.read_shpr3(),
            0xE000_ED24 => self.shcsr,
            0xE000_ED28 => self.cfsr,
            0xE000_ED2C => self.hfsr,
            0xE000_ED30 => self.dfsr,
            0xE000_ED34 => self.mmfar,
            0xE000_ED38 => self.bfar,
            0xE000_ED3C => self.afsr,

            0xE000_ED88 => self.cpacr,

            0xE000_EF34 => self.fpccr,
            0xE000_EF38 => self.fpcar,
            0xE000_EF3C => self.fpdscr,

            0xE000_EF40 => self.mvfr0,
            0xE000_EF44 => self.mvfr1,
            0xE000_EF48 => self.mvfr2,

            0xE000_EDFC => self.read_demcr(),

            // DWT
            0xE000_1000 => self.dwt_ctrl,
            _ => {
                if self.sram.in_range(addr) {
                    self.sram.read32(addr)?
                } else if self.code.in_range(addr) {
                    self.code.read32(addr)?
                } else if self.device.in_range(addr) {
                    self.device.read32(addr)?
                } else {
                    return Err(Fault::DAccViol);
                }
            }
        };
        Ok(result)
    }

    fn write32(&mut self, addr: u32, value: u32) -> Result<(), Fault> {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u32(((addr - 0xE000_0000) >> 2) as u8, value)
            }

            0xE000_1000 => self.dwt_write_ctrl(value),
            0xE000_1004 => self.dwt_write_cyccnt(value),

            0xE000_1FB0 => self.itm_write_lar_u32(value),

            0xE000_ED04 => self.write_icsr(value),
            0xE000_ED08 => self.write_vtor(value),
            0xE000_ED10 => self.write_scr(value),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED18 => self.write_shpr1(value),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED1C => self.write_shpr2(value),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED20 => self.write_shpr3(value),

            0xE000_EDFC => self.write_demcr(value),

            0xE000_E010 => self.syst_write_csr(value),
            0xE000_E014 => self.syst_write_rvr(value),
            0xE000_E018 => self.syst_write_cvr(value),
            0xE000_E100..=0xE000_E13C => {
                self.nvic_write_iser(((addr - 0xE000_E100) >> 5) as usize, value)
            }
            0xE000_E180..=0xE000_E1BC => {
                self.nvic_write_icer(((addr - 0xE000_E180) >> 5) as usize, value)
            }
            0xE000_E200..=0xE000_E23C => {
                self.nvic_write_ispr(((addr - 0xE000_E200) >> 5) as usize, value)
            }
            0xE000_E280..=0xE000_E2BC => {
                self.nvic_write_icpr(((addr - 0xE000_E280) >> 5) as usize, value)
            }
            0xE000_E400..=0xE000_E5EC => {
                self.nvic_write_ipr(((addr - 0xE000_E400) >> 2) as usize, value)
            }

            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_EF00 => self.write_stir(value),
            _ => {
                if self.sram.in_range(addr) {
                    return self.sram.write32(addr, value);
                } else if self.code.in_range(addr) {
                    return self.code.write32(addr, value);
                } else if self.device.in_range(addr) {
                    return self.device.write32(addr, value);
                } else {
                    return Err(Fault::DAccViol);
                }
            }
        }
        Ok(())
    }

    fn write16(&mut self, addr: u32, value: u16) -> Result<(), Fault> {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u16(((addr - 0xE000_0000) >> 2) as u8, value)
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED18..=0xE000_ED1B => {
                self.write_shpr1_u16(((addr - 0xE000_ED18) >> 1) as usize, value)
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED1C..=0xE000_ED1F => {
                self.write_shpr2_u16(((addr - 0xE000_ED1C) >> 1) as usize, value)
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED20..=0xE000_ED23 => {
                self.write_shpr3_u16(((addr - 0xE000_ED20) >> 1) as usize, value)
            }
            0xE000_E400..=0xE000_E5EC => {
                self.nvic_write_ipr_u16(((addr - 0xE000_E400) >> 1) as usize, value)
            }
            _ => {
                if self.sram.in_range(addr) {
                    return self.sram.write16(addr, value);
                } else if self.code.in_range(addr) {
                    return self.code.write16(addr, value);
                } else if self.device.in_range(addr) {
                    return self.device.write16(addr, value);
                } else {
                    return Err(Fault::DAccViol);
                }
            }
        }
        Ok(())
    }

    fn write8(&mut self, addr: u32, value: u8) -> Result<(), Fault> {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u8(((addr - 0xE000_0000) >> 2) as u8, value)
            }
            0xE000_E400..=0xE000_E5EC => {
                self.nvic_write_ipr_u8((addr - 0xE000_E400) as usize, value)
            }
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED18..=0xE000_ED1B => self.write_shpr1_u8((addr - 0xE000_ED18) as usize, value),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED1C..=0xE000_ED1F => self.write_shpr2_u8((addr - 0xE000_ED1C) as usize, value),
            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            0xE000_ED20..=0xE000_ED23 => self.write_shpr3_u8((addr - 0xE000_ED20) as usize, value),

            _ => {
                if self.sram.in_range(addr) {
                    return self.sram.write8(addr, value);
                } else if self.code.in_range(addr) {
                    return self.code.write8(addr, value);
                } else if self.device.in_range(addr) {
                    return self.device.write8(addr, value);
                } else {
                    return Err(Fault::DAccViol);
                }
            }
        }
        Ok(())
    }

    #[allow(unused)]
    fn in_range(&self, addr: u32) -> bool {
        self.code.in_range(addr) || self.sram.in_range(addr) || self.device.in_range(addr)
    }
}
