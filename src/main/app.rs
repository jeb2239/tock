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
use sched;

//use super::boxed::BoxMgr;





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




pub fn svc5()  {
    
    
    
    unsafe{
       asm!("push {r4-r11}
            svc 5
            pop {r4-r11}
            ");
    }
}

pub struct App {
    pub memory: BoxMgr,
    pub system_call : fn(&mut Firestorm, &mut Process, AppId) ,
    
    
}

pub fn init(g:usize) {
   // print_as!("Welcome to Tock!\r\n");
   
  //  let a = String::new("Heyyyyyyyyyyyyyyyyyyyyyy"); 
    
    syscalls::start_count();
    print_as_slow!( "{}\r\n" );
   print!("{}\r\n",syscalls::end_count());
   
   
   //let g= g as *mut u8;
   // print!("{:?}\r\n",unsafe { (a.as_ptr() as usize) - (g as usize) } );
    //let stats = (unsafe { &*super::app }).memory.stats();
    
  //  print!("\tNum Allocated: {:?}\r\n", a.len());
   // print_as!("haye");
  
 // (unsafe { &mut *super::app}).system_call = sched::set_led;//the pc
 //  let a = sched::set_led as usize;
   //  svc5();
    //syscalls::safe_call();
    
 //   print!("\tNum Allocated: {}\r\n", stats.num_allocated);
    // print!("\tNum Allocs: {}\r\n", stats.allocs);
    // print!("\tDrops: {}\r\n", stats.drops);
    // print!("\tAllocated Bytes: {}\r\n", stats.allocated_bytes);
    // print!("\tFree Bytes: {}\r\n", stats.free);
    // print!("\tActive: {}\r\n", stats.active);
    
    
}


