#![feature(core_str_ext,core_slice_ext,const_fn,no_std,raw,core_char_ext,unique,slice_bytes,asm)]
#![no_main]
#![no_std]
extern crate common;
extern crate support;
extern crate hil;
extern crate process;
#[macro_use(println,print)]
extern crate platform;
extern crate drivers;

#[macro_use]
pub mod console;
pub mod boxed;
pub mod sched;
pub mod app;
pub mod syscall;
pub mod string;
pub mod syscalls;



#[allow(improper_ctypes)]
extern {
    static _sapps : usize;
}

/*rust app*/
static mut app : *mut app::App = 0 as  *mut app::App;

pub fn rust_app(mem_start: *mut u8, mem_size: usize  )->!{
   // mem_start: *mut u8, mem_size: usize
use core::mem::size_of;
   
    let myapp = unsafe {
        app = mem_start as *mut app::App;
        &mut *app
    };
    let appsize = size_of::<app::App>();
    
    myapp.memory = boxed::BoxMgr::new(mem_start, mem_size, appsize);
    
    
    app::init(mem_start as usize);
    
    loop {
        
        syscalls::wait();
    }

}

#[no_mangle]
pub extern fn main() {
    use process::Process;
    use process::AppId;
    
    let mut platform = unsafe {
        platform::init()
    };
   
     // println!("{:?}",40 );
    
      
    //(unsafe { &mut *app}).system_call;
    
   
    //println!("{:?}", 4);
    let processes = unsafe {
        process::process::PROCS = [Process::create(rust_app as *const usize)];
        &mut process::process::PROCS
    };

    loop {
        unsafe {
           // println!("{:?}", 30);
            platform.service_pending_interrupts();//handle everything that wants to interrupt us 
        //    println!("{:?}",(unsafe { &mut *app}).val);
            for (i, p) in processes.iter_mut().enumerate() { //in here we have process 
                p.as_mut().map(|process| {
                
         //            println!("{:?}",(unsafe { &mut *app}).val); 
                    sched::do_process(platform, process, AppId::new(i),(unsafe {&mut * app}).system_call);
                   
                });
            }

            support::atomic(|| {
                if !platform.has_pending_interrupts() {
             //       println!("{:?}", 20);
                   
                    support::wfi();
                }
            })
        };
       // println!("{:?}","yo" );
    }
}

