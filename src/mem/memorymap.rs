
use spin::Mutex;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MemorySection {
    pub base_addr: u64,
    pub length: u64,
    pub typ: u32,   
    pub reserved: u32,  
}

pub static MEMORYMAP: Mutex<[MemorySection; 8]> = Mutex::new([
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0}, 
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0},
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0}, 
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0},
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0}, 
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0},
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0}, 
MemorySection{base_addr: 0, length: 0, typ: 0, reserved: 0}
]);
