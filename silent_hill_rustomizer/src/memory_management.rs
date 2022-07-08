use std::process::Child;
use vmemory::{self, ProcessMemory};
use byteorder::{BigEndian, ByteOrder};

pub fn read_highscore(proc_id: u32) -> u32{
    let hs:u32;
    let addr:usize = 0x070E66F0;
  

    let mut attached_proc = ProcessMemory::attach_process(proc_id).unwrap();
    attached_proc.resume();

    /*
    let write_test: Vec<u8> = vec![7, 7, 9, 9, 9];
    attached_proc.write_memory(addr, &write_test, false);
    */

    let vmem = attached_proc.read_memory(addr, 5, true);
    let mut vec_hs: Vec<u32> = Vec::new();

    BigEndian::read_u32_into(&vmem[1..=4], &mut vec_hs);
    //hs = (vmem[0] << 3 + vmem[1] << 2 + vmem[2] << 1 + vmem[3]) as i32;

    hs = vec_hs[0];
    return hs;
}

