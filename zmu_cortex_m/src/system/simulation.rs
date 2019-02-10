use crate::bus::ahblite::AHBLite;
use crate::bus::busmatrix::BusMatrix;
use crate::bus::internal::InternalBus;
use crate::core::instruction::instruction_size;
use crate::core::instruction::Instruction;
use crate::core::Core;
use crate::core::ThumbCode;
use crate::memory::flash::FlashMemory;
use crate::memory::ram::RAM;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use std::io;

pub struct TraceData {
    pub opcode: ThumbCode,
    pub count: u64,
    pub pc: u32,
    pub instruction: Instruction,
    pub r0_12: [u32; 13],
    pub psr_value: u32,
}

pub fn simulate<F>(
    code: &[u8],
    mut semihost_func: F,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64
where
    F: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    let mut flash_memory = FlashMemory::new(0, 65536);
    let mut ram_memory = RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd);

    flash_memory.load(code);

    let mut internal_bus = InternalBus::new(itm_file);
    let mut ahb = AHBLite::new(&mut flash_memory, &mut ram_memory);
    let mut bus = BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            core.set_pc(pc);
            let thumb = core.fetch();
            let instruction = core.decode(thumb);
            instruction_cache.push((instruction, instruction_size(&instruction)));
            pc += 2;
        }
    }

    core.reset();

    while core.running {
        let pc = core.get_pc();
        let (instruction, instruction_size) = &instruction_cache[(pc >> 1) as usize];
        core.step(
            instruction,
            *instruction_size,
            |semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                semihost_func(semihost_cmd)
            },
        );

        count += 1;
    }

    count
}

pub fn simulate_trace<F, G>(
    code: &[u8],
    mut trace_func: F,
    mut semihost_func: G,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64
where
    F: FnMut(&TraceData),
    G: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    let mut flash_memory = FlashMemory::new(0, 65536);
    let mut ram_memory = RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd);

    flash_memory.load(code);

    let mut internal_bus = InternalBus::new(itm_file);
    let mut ahb = AHBLite::new(&mut flash_memory, &mut ram_memory);
    let mut bus = BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            core.set_pc(pc);
            let thumb = core.fetch();
            let instruction = core.decode(thumb);
            instruction_cache.push((thumb, instruction, instruction_size(&instruction)));
            pc += 2;
        }
    }

    core.reset();

    while core.running {
        let pc = core.get_pc();
        let (opcode, instruction, instruction_size) = &instruction_cache[(pc >> 1) as usize];
        core.step(
            instruction,
            *instruction_size,
            |semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                semihost_func(semihost_cmd)
            },
        );

        let trace_data = TraceData {
            opcode: *opcode,
            count: core.cycle_count,
            pc: pc,
            instruction: *instruction,
            r0_12: core.r0_12,
            psr_value: core.psr.value,
        };
        trace_func(&trace_data);
        count += 1;
    }

    count
}
