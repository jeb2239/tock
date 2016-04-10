#![feature(core_str_ext,core_slice_ext,const_fn,no_std,raw,core_char_ext,unique,slice_bytes)]
#![no_main]
#![no_std]

extern crate common;
extern crate support;
extern crate hil;
extern crate process;
#[macro_use(println,print)]
extern crate platform;



mod sched;
mod app;
pub mod syscall;


#[allow(improper_ctypes)]
extern {
    static _sapps : usize;
}

/*rust app*/


#[no_mangle]
pub extern fn main() {
    use process::Process;
    use process::AppId;

    let mut platform = unsafe {
        platform::init()
    };
   
     // println!("{:?}",40 );
      
        

   
    //println!("{:?}", 4);
    let processes = unsafe {
        process::process::PROCS = [Process::create(app::rust_app as *const usize)];
        &mut process::process::PROCS
    };

    loop {
        unsafe {
            platform.service_pending_interrupts();

            for (i, p) in processes.iter_mut().enumerate() {
                p.as_mut().map(|process| {
                    sched::do_process(platform, process, AppId::new(i));
                });
            }

            support::atomic(|| {
                if !platform.has_pending_interrupts() {
                    support::wfi();
                }
            })
        };
    }
}

