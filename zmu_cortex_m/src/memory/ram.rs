use byteorder::{ByteOrder, LittleEndian};
use bus::Bus;

pub struct RAM {
    start_address: u32,
    data: Box<[u8]>,
}

impl RAM {
    pub fn new(start_address: u32, size: usize) -> RAM {
        let data = vec![0u8; size].into_boxed_slice();

        RAM {
            start_address: start_address,
            data: data,
        }
    }
}

impl Bus for RAM {
    fn read8(&mut self, addr: u32) -> u8 {
        let a = addr - self.start_address;
        self.data[a as usize]
    }
    fn read16(&mut self, addr: u32) -> u16 {
        let a = (addr - self.start_address) as usize;

        LittleEndian::read_u16(&self.data[a..a + 2])
    }

    fn read32(&mut self, addr: u32) -> u32 {
        let a = (addr - self.start_address) as usize;
        LittleEndian::read_u32(&self.data[a..a + 4])
    }

    fn write8(&mut self, addr: u32, value: u8) {
        let a = addr - self.start_address;
        self.data[a as usize] = value
    }

    fn write16(&mut self, addr: u32, value: u16) {
        let a = (addr - self.start_address) as usize;

        LittleEndian::write_u16(&mut self.data[a..a + 2], value)
    }

    fn write32(&mut self, addr: u32, value: u32) {
        let a = (addr - self.start_address) as usize;
        LittleEndian::write_u32(&mut self.data[a..a + 4], value)
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= self.start_address) && (addr < (self.start_address + self.data.len() as u32)) {
            return true;
        }
        false
    }
}

#[test]
fn test_new() {
    // should be able to make new instance of memory
    let _mem = RAM::new(0, 1024);
}

#[test]
fn test_in_range() {
    {
        /* no offset */
        let mem = RAM::new(0, 1024);
        assert!(mem.in_range(0));
        assert!(mem.in_range(1023));
        assert!(!mem.in_range(1024));
        assert!(!mem.in_range(0xFFFF_FFFF));
    }

    {
        /* offset of 0x8000_0000 */
        let mem = RAM::new(0x8000_0000, 1024);
        assert!(mem.in_range(0x8000_0000));
        assert!(mem.in_range(0x8000_0001));
        assert!(!mem.in_range(0x8000_0000 + 1024));
        assert!(!mem.in_range(0x8000_0000 + 0xffff));
    }
}

#[test]
fn test_write_read() {
    {
        let mut mem = RAM::new(0, 1024);
        mem.write32(0, 0xAABBCCDD);
        assert_eq!(mem.read32(0), 0xAABBCCDD);
        mem.write32(1020, 0xAABBCCDD);
        assert_eq!(mem.read32(1020), 0xAABBCCDD);
    }

    {
        let mut mem = RAM::new(0, 1024);
        mem.write16(0, 0xAABB);
        assert_eq!(mem.read16(0), 0xAABB);
        mem.write16(1022, 0xCCDD);
        assert_eq!(mem.read16(1022), 0xCCDD);
    }

    {
        let mut mem = RAM::new(0, 1024);
        mem.write8(0, 0xAA);
        assert_eq!(mem.read8(0), 0xAA);
        mem.write8(1022, 0xCC);
        assert_eq!(mem.read8(1022), 0xCC);
    }
}
