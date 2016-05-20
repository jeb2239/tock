use platform::Firestorm;
use process;
use process::Process;
use process::{AppSlice,AppId};
use common::Queue;
use hil;
use syscall;
use core::raw::Slice;
use core::ops::{Deref,DerefMut};
use core::ptr::Unique;
#[macro_use]
use console::*;
use super::string;
use syscalls;
//use super::boxed::BoxMgr;


// pub fn svc5()  {
//     unsafe{
//        asm!("push {r4-r11}
//             svc 5
//             pop {r4-r11}
//             ");
//     }
// }

// pub fn svc6(){
//     unsafe{
//         asm!(
//             "
//             push {r4-r11}
//             svc 6
//             pop {r4-r11}
//             "
//         );
//     }
// }




use super::boxed::BoxMgr;
use super::string::String;

pub struct App {
    pub memory: BoxMgr
    
}

pub fn init() {
    print!("Welcome to Tock!\r\n");
    let a = String::new("Heyy");
    let stats = (unsafe { &*super::app }).memory.stats();
    puts(a);
    print!("Memory Stats:{}\r\n", "");
    print!("\tNum Allocated: {}\r\n", stats.num_allocated);
    print!("\tNum Allocs: {}\r\n", stats.allocs);
    print!("\tDrops: {}\r\n", stats.drops);
    print!("\tAllocated Bytes: {}\r\n", stats.allocated_bytes);
    print!("\tFree Bytes: {}\r\n", stats.free);
    print!("\tActive: {}\r\n", stats.active);
    
    
}


