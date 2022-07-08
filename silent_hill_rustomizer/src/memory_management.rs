use vmemory::{self, ProcessMemory};
use byteorder::{BigEndian, ByteOrder};

pub fn read_highscore(proc_id: u32) -> u32{
    let addr:usize = 0x070E66F0- 0x400000;
  

    let attached_proc = ProcessMemory::attach_process(proc_id).unwrap();
    attached_proc.resume();


    let vmem = attached_proc.read_memory(addr, 4, true);
    
	
    println!("Base addr: {:08X}",attached_proc.base());


    println!("{:08X}, {:08X}, {:08X}, {:08X}", vmem[0], vmem[1], vmem[2], vmem[3]);

    //BigEndian::read_u32_into(&vmem[1..=8], &mut vec_hs);
    //1740050 //1A8D12
    
    u32_le_wrapper(&vmem)
}

pub fn kill_proc(proc_id: u32){
	let attached_proc = ProcessMemory::attach_process(proc_id).unwrap();
	attached_proc.kill()
}

pub fn read_bonus(proc_id: u32) -> u8{
    let addr:usize = 0x0712C59C; 

    let attached_proc = ProcessMemory::attach_process(proc_id).unwrap();
    attached_proc.resume();


    let vmem = attached_proc.read_memory(addr, 4, true);
    


    println!("{:08X}", vmem[0]);

    //BigEndian::read_u32_into(&vmem[1..=8], &mut vec_hs);
    //1740050

    
	vmem[0]
}


fn u32_le_wrapper(vect: &Vec<u8>) -> u32 {
    let mut arr: [u8;4] = [0;4];

    for i in 0..4{
        arr[i] = vect[i];
    }

	as_u32_le(&arr)

}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}