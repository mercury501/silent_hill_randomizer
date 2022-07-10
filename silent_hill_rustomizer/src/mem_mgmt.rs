use vmemory::{self, ProcessMemory};
use byteorder::{ByteOrder, LittleEndian};

pub fn write_u16(proc_id: u32, addr:usize, data: u16){
    let vec: Vec<u8> = data.to_le_bytes().to_vec();

    write_u8_arr(proc_id, addr, &vec);
}

pub fn write_u8(proc_id: u32, addr:usize, data: u8){
    let vec: Vec<u8> = vec![data];

    write_u8_arr(proc_id, addr, &vec);
}

fn write_u8_arr(proc_id: u32, addr: usize, data: &Vec<u8>){ 
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return,
    }

    attached_proc.resume();

    attached_proc.write_memory(addr, data, false);
}

pub fn read_u32(proc_id: u32, addr: usize) -> u32{
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return 0,
    }

    attached_proc.resume();

    let vmem = attached_proc.read_memory(addr, 4, false);    
	    
    vec_to_u32_le_wrapper(&vmem)
}

pub fn read_u16(proc_id: u32, addr: usize) -> u16{
    let attached_proc: ProcessMemory;

    match ProcessMemory::attach_process(proc_id){
        Some(p) => attached_proc = p,
        None => return 0,
    }

    attached_proc.resume();

    let vmem = attached_proc.read_memory(addr, 2, false);    
	    
    vec_to_u16_le_wrapper(&vmem)
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
    
    vec_to_f32_le_wrapper(&vmem)
}
/*
pub fn write_byte_to_addr(proc_id: u32, data: &Vec<u8>, addr: usize){
    let attached_proc = ProcessMemory::attach_process(proc_id).unwrap();
    attached_proc.resume();

    attached_proc.write_memory(addr, data, false);  
}*/

fn vec_to_f32_le_wrapper(vect: &Vec<u8>) -> f32{
    let mut arr: [u8;4] = [0;4];

    for i in 0..4{
        arr[i] = vect[i];
    }
    
    LittleEndian::read_f32(&arr)
}

fn vec_to_u32_le_wrapper(vect: &Vec<u8>) -> u32{
    let mut arr: [u8;4] = [0;4];

    for i in 0..4{
        arr[i] = vect[i];
    }

    LittleEndian::read_u32(&arr)
}

fn vec_to_u16_le_wrapper(vect: &Vec<u8>) -> u16{
    let mut arr: [u8;4] = [0;4];

    for i in 0..2{
        arr[i] = vect[i];
    }

    LittleEndian::read_u16(&arr)
}