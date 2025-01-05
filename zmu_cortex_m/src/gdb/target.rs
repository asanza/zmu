use gdbstub::target::Target;
use gdbstub::target::TargetResult;
use gdbstub::target;
use gdbstub::target::ext::monitor_cmd::MonitorCmd;
use gdbstub::common::Signal;
use gdbstub::outputln;

use log::debug;

use crate::MemoryMapConfig;
use crate::gdb::simulation;

use gdbstub::target::ext::base::singlethread::SingleThreadBase;
use gdbstub::target::ext::base::singlethread::SingleThreadResume;
use gdbstub::target::ext::base::singlethread::SingleThreadSingleStep;
use gdbstub::target::ext::base::singlethread::SingleThreadSingleStepOps;
use gdbstub::target::ext::base::singlethread::SingleThreadResumeOps;
use gdbstub::target::ext::base::singlethread::SingleThreadRangeStepping;
use gdbstub::target::ext::base::singlethread::SingleThreadRangeSteppingOps;

use crate::gdb::simulation::SimulationRunEvent;
use crate::gdb::simulation::SimulationEvent;

use crate::core::register::Reg;
use crate::core::register::BaseReg;

use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

pub struct ZmuTarget {
    simulation: simulation::Simulation,
}

impl ZmuTarget {
    pub fn new(
        code: &[u8],
        semihost_func: Box<dyn FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
        map: Option<MemoryMapConfig>,
        flash_size: usize,
    ) -> ZmuTarget {
        let simulation = simulation::Simulation::new(code, semihost_func, map, flash_size);
        ZmuTarget {
            simulation: simulation.unwrap(),
        }
    }
 
    pub fn run(&mut self, poll_incomming_data: impl FnMut() -> bool ) -> SimulationRunEvent {
        self.simulation.run(poll_incomming_data)
    }

    pub fn step(&mut self) -> SimulationEvent {
        self.simulation.step()
    }
}

impl Target for ZmuTarget {
    type Arch = gdbstub_arch::arm::Armv4t;
    type Error = &'static str;

    #[inline(always)]
    fn base_ops(&mut self) -> target::ext::base::BaseOps<'_, Self::Arch, Self::Error> {
        target::ext::base::BaseOps::SingleThread(self)
    }

    #[inline(always)]
    fn use_no_ack_mode(&self) -> bool {
        true
    }

    #[inline(always)]
    fn use_x_upcase_packet(&self) -> bool {
        true
    }

    #[inline(always)]
    fn support_monitor_cmd(&mut self) -> Option<target::ext::monitor_cmd::MonitorCmdOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_breakpoints(
        &mut self,
    ) -> Option<target::ext::breakpoints::BreakpointsOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadBase for ZmuTarget {
    fn read_registers(
        &mut self,
        regs: &mut gdbstub_arch::arm::reg::ArmCoreRegs,
    ) -> TargetResult<(), Self> {
        debug!("> read_registers");
        regs.r = self.simulation.processor.r0_12;
        regs.sp = self.simulation.processor.get_r(Reg::SP);
        regs.lr = self.simulation.processor.lr;
        regs.pc = self.simulation.processor.get_pc();
        regs.cpsr = self.simulation.processor.cfsr;
        Ok(())
    }

    #[inline(never)]
    fn write_registers(
        &mut self,
        _regs: &gdbstub_arch::arm::reg::ArmCoreRegs
    ) -> TargetResult<(), Self> {
        debug!("> write_registers");
        Ok(())
    }

    #[inline(never)]
    fn read_addrs(
        &mut self,
        _start_addr: u32,
        data: &mut [u8],
    ) -> TargetResult<usize, Self> {
        debug!("> read_addrs");
        data.iter_mut().for_each(|b| *b = 0x55);
        Ok(data.len())
    }

    #[inline(never)]
    fn write_addrs(
        &mut self,
        _start_addr: u32,
        _data: &[u8],
    ) -> TargetResult<(), Self> {
        debug!("> write_addrs");
        Ok(())
    }

    #[inline(always)]
    fn support_resume(
        &mut self,
    ) -> Option<SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadResume for ZmuTarget {
    #[inline(never)]
    fn resume(&mut self, _signal: Option<Signal>) -> Result<(), Self::Error> {
        self.simulation.exec_mode = simulation::SimulationExecMode::Continue;
        Ok(())
    }

    #[inline(always)]
    fn support_single_step(
        &mut self
    ) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        self.simulation.exec_mode = simulation::SimulationExecMode::Step;
        Some(self)
    }

    #[inline(always)]
    fn support_range_step(&mut self) -> Option<SingleThreadRangeSteppingOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadSingleStep for ZmuTarget {
    #[inline(never)]
    fn step(&mut self, _signal: Option<Signal>) -> Result<(), Self::Error> {
        self.simulation.exec_mode = simulation::SimulationExecMode::Step;
        Ok(())
    }
}

impl SingleThreadRangeStepping for ZmuTarget {
    #[inline(never)]
    fn resume_range_step(
        &mut self,
        start: u32,
        end: u32,
    ) -> Result<(), Self::Error> {
        self.simulation.exec_mode = simulation::SimulationExecMode::RangeStep(start, end);
        Ok(())
    }
    
}


impl target::ext::breakpoints::Breakpoints for ZmuTarget {
    #[inline(always)]
    fn support_sw_breakpoint(
        &mut self,
    ) -> Option<target::ext::breakpoints::SwBreakpointOps<'_, Self>> {
        Some(self)
    }
}

impl target::ext::breakpoints::SwBreakpoint for ZmuTarget {
    #[inline(never)]
    fn add_sw_breakpoint(
        &mut self,
        addr: u32,
        _kind: gdbstub_arch::arm::ArmBreakpointKind,
    ) -> TargetResult<bool, Self> {
        self.simulation.breakpoints.push(addr);
        Ok(true)
    }

    #[inline(never)]
    fn remove_sw_breakpoint(
        &mut self,
        addr: u32,
        _kind: gdbstub_arch::arm::ArmBreakpointKind,
    ) -> TargetResult<bool, Self> {
        self.simulation.breakpoints.retain(|&x| x != addr);
        Ok(true)
    }
}

impl MonitorCmd for ZmuTarget {
    #[inline(never)]
    fn handle_monitor_cmd(
        &mut self,
        cmd: &[u8],
        mut out: gdbstub::target::ext::monitor_cmd::ConsoleOutput<'_>,
    ) -> Result<(), Self::Error> {
        debug!("> handle_monitor_cmd {:?}", cmd);
        let cmd = core::str::from_utf8(cmd).map_err(|_| "Invalid UTF-8")?;
        match cmd  {
            "reset" => {
                match self.simulation.reset() {
                    Ok(_) => {
                        outputln!(out, "Target reset");
                    },
                    Err(_) => {
                        outputln!(out, "Error resetting target");
                        return Err("Error resetting target");
                    }
                }
            }
            _ => {
                outputln!(out, "Unknown command: {:?}", cmd);
            }
        }
        Ok(())
    }
}
