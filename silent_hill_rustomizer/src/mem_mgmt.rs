use vmemory::{self, ProcessMemory};
use byteorder::{ByteOrder, LittleEndian};

pub fn read_highscore(proc_id: u32) -> u32{
    let addr:usize = 0x070E66F0;

    read_u32(proc_id, addr)

}

pub fn read_u32(proc_id: u32, addr: usize) -> u32{
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return 0,
    }

    attached_proc.resume();

    let vmem = attached_proc.read_memory(addr, 4, false);    
	    
    u32_le_wrapper(&vmem)
}

pub fn read_u8(proc_id: u32, addr: usize) -> u8{
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return 0,
    }

    attached_proc.resume();

    let vmem = attached_proc.read_memory(addr, 1, false);    
	    
    vmem[0]
}
#[allow(unused)]
pub fn read_f32(proc_id: u32, addr: usize) -> f32{
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return 0.0,
    }
    
    attached_proc.resume();

    let vmem = attached_proc.read_memory(addr, 4, false);
    
    f32_le_wrapper(&vmem)
}

pub fn read_bonus(proc_id: u32) -> u32{
    let addr:usize = 0x0712C59C; 
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return 0,
    }
    
    attached_proc.resume();

    let vmem = attached_proc.read_memory(addr, 4, false);
    
    f32_le_wrapper(&vmem) as u32
}

pub fn write_byte_to_addr(proc_id: u32, data: &Vec<u8>, addr: usize){
    let attached_proc = ProcessMemory::attach_process(proc_id).unwrap();
    attached_proc.resume();

    attached_proc.write_memory(addr, data, false);  
}


fn f32_le_wrapper(vect: &Vec<u8>) -> f32{
    let mut arr: [u8;4] = [0;4];

    for i in 0..4{
        arr[i] = vect[i];
    }
    
    LittleEndian::read_f32(&arr)
}

fn u32_le_wrapper(vect: &Vec<u8>) -> u32{
    let mut arr: [u8;4] = [0;4];

    for i in 0..4{
        arr[i] = vect[i];
    }

    LittleEndian::read_u32(&arr)
}
