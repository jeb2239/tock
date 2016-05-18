

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
//use super::boxed::BoxMgr;
#[allow(improper_ctypes)]
extern {
    fn __safe_call(driver_num:usize , ptr: *mut ()) -> isize;
    fn __allow(driver_num: usize, allownum: usize, ptr: *mut (), len: usize) -> isize;
    fn __subscribe(driver_num: usize, subnum: usize, cb: usize, appdata: usize) -> isize;
    fn __command(driver_num: usize, cmdnum: usize, arg1: usize) -> isize;
    fn __wait() -> isize;
    fn __start_count() -> isize;
    fn __end_count() -> isize;
    fn switch_to_user(user_stack: *mut u8, mem_base: *mut u8) -> *mut u8;
}

pub fn start_count() -> isize {
  unsafe{
    __start_count()
  }
}



pub fn end_count() -> isize {
  unsafe{
    __end_count()
  }
}

pub fn allow(driver_num: usize, allownum: usize, ptr: *mut (), len: usize) -> isize {
    unsafe {
      __allow(driver_num, allownum, ptr, len)
    }
}

pub fn command(driver_num: usize, cmdnum: usize, arg1: usize) -> isize {
    unsafe {
        __command(driver_num, cmdnum, arg1)
    }
}

pub fn subscribe(driver_num: usize, cmdnum: usize, callback: usize, appdata: usize) -> isize {
    unsafe {
        __subscribe(driver_num, cmdnum, callback, appdata)
    }
}

pub fn enable_pin(pin: usize) -> isize {
    command(1, 0, pin)
}

pub fn set_pin(pin: usize) -> isize{
    command(1, 1, pin) //1,2,pin
}

pub fn clear_pin(pin: usize) -> isize {
    command(1, 2, pin) //3
}

pub fn toggle_pin(pin: usize) -> isize {
    command(1, 3, pin) //4
}


pub fn wait() -> isize {
    unsafe {
        __wait()
    }
}

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
    enable_pin(0);
    start_count();
    
    for i in 0..100 {
    //svc();
    set_pin(0);
   // clear_pin(0);
    }
    
    let a = end_count();
    print!("Clock Cycles for svc : {}\r\n",a);
    print!("\tAllocated Bytes: {}\r\n", stats.allocated_bytes);
    print!("\tFree Bytes: {}\r\n", stats.free);
    print!("\tActive: {}\r\n", stats.active);
    
    
    
}


