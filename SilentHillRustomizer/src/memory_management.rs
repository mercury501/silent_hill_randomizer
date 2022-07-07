use std::process::Child;

use process_memory::*;

pub fn read_highscore(proc: Child) -> i32{
    let hs:i32;
    let addr:usize = 0x070E66F0;
    let handle = (proc.id() as Pid).try_into_process_handle().unwrap();
    let member = DataMember::new_offset(handle, vec![addr as *const _ as usize]);


    return hs;
}