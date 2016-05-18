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
use console;
use string;
use syscalls;
//use super::boxed::BoxMgr;


pub fn svc()  {
    unsafe{
       asm!("push {r4-r11}
            svc 5
            pop {r4-r11}
            ");
    }
}



use super::boxed::BoxMgr;
use super::string::String;

pub struct App {
    pub memory: BoxMgr
    
}

pub fn init() {
    print!("Welcome to Tock!\r\n");

    let stats = (unsafe { &*super::app }).memory.stats();
    print!("Memory Stats:{}\r\n", "");
    print!("\tNum Allocated: {}\r\n", stats.num_allocated);
    print!("\tNum Allocs: {}\r\n", stats.allocs);
    print!("\tDrops: {}\r\n", stats.drops);
    syscalls::enable_pin(0);
    syscalls::start_count();
    console::puts(String::new("Hello!"));
    for i in 0..100 {
    //svc();
    syscalls::set_pin(0);
   // clear_pin(0);
    }
    
    let a = syscalls::end_count();
    print!("Clock Cycles for svc : {}\r\n",a);
    print!("\tAllocated Bytes: {}\r\n", stats.allocated_bytes);
    print!("\tFree Bytes: {}\r\n", stats.free);
    print!("\tActive: {}\r\n", stats.active);
    
    
    
}


