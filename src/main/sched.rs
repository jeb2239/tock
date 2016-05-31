use platform::Firestorm;
use process;
use process::Process;
use process::{AppSlice,AppId,Callback};
use common::Queue;

use hil::{Shared};
use hil;
use syscall;
use app;





pub fn set_led(platform: &mut Firestorm, process: &mut Process, appid: AppId){
    platform.typedgpio.enable_output(0);
    platform.typedgpio.set_pin(0);
}





pub unsafe fn do_process(platform: &mut Firestorm, process: &mut Process,
                  appid: AppId,f:fn(&mut Firestorm, &mut Process, AppId)) {
                    
          
    loop {
        match process.state {  //so here we have to check the state of the process running
            process::State::Running => {
                process.switch_to();
               // println!("heyy"); // when you do inline asm do not bx lr at the end or you will hard fault
            }
            process::State::Waiting => {
                match process.callbacks.dequeue() {
                    None => { return },
                    Some(cb) => {
                     //   println!("yooyoyoyo");
                        
                        process.state = process::State::Running;
                        process.switch_to_callback(cb);
                        
                        
                    }
                }
            }
        }
        match process.svc_number() {
            Some(syscall::WAIT) => {
               // println!("heyfool");
                process.state = process::State::Waiting;
                process.pop_syscall_stack();
                
                break;
            },
            Some(syscall::SUBSCRIBE) => {
                println!("SUBSCRIBE");
                let driver_num = process.r0();
                let subdriver_num = process.r1();// ----- in stead of passing in a number, just pass in a pointer to the driver 
                                                //function
                           
                let callback_ptr = process.r2() as *mut ();
                let appdata = process.r3();
                // println!("{:?}",driver_num);
                // println!("{:?}",subdriver_num );
                // println!("{:?}",callback_ptr );
                // println!("{:?}", appdata );
                let res = platform.with_driver(driver_num, |driver| {
                    let callback =
                        hil::Callback::new(appid, appdata, callback_ptr);
                    match driver {
                        Some(d) => d.subscribe(subdriver_num,
                                               callback),
                        None => -1
                    }
                });
                process.set_r0(res);
            },
            Some(syscall::COMMAND) => {
                //println!("Hello");
                let res = platform.with_driver(process.r0(), |driver| {
                    match driver {
                        Some(d) => d.command(process.r1(),
                                             process.r2()),
                        None => -1
                    }
                });
                process.set_r0(res);
            },
            Some(syscall::ALLOW) => {
                println!("ALLOW");
                let res = platform.with_driver(process.r0(), |driver| {
                    match driver {
                        Some(d) => {
                            let start_addr = process.r2() as *mut u8;
                            let size = process.r3();
                            if process.in_exposed_bounds(start_addr, size) {
                                let slice = AppSlice::new(start_addr as *mut u8, size, appid);
                                d.allow(appid, process.r1(), slice)
                            } else {
                                -1
                            }
                        },
                        None => -1
                    }
                });
                process.set_r0(res);
            },
            Some(syscall::SAFE) => {
                
                let driver_num = process.r0();
                let subdriver_num = process.r1();// ----- in stead of passing in a number, just pass in a pointer to the driver 
                                                //function
                          
                let callback_ptr = process.r2() as *mut ();
                let appdata = process.r3();
              
             
                let res = platform.with_driver(process.r0(), |driver| {
                    match driver {
                        Some(d) => d.command(process.r1(),
                                             process.r2()),
                        None => -1
                    }
                });
                process.set_r0(res);
                
             
                    
                    
                  
                
                  
            },
            
            Some(syscall::FAST_PRINT_ASYNC) => {
                println!("FAST_PRINT_ASYNC");
                let res = platform.with_driver(0, |driver| {
                    match driver {
                        Some(d) => {
                            let start_addr = process.r0() as *mut u8;
                            let size = process.r1();
                            if process.in_exposed_bounds(start_addr, size) {
                                let slice = AppSlice::new(start_addr as *mut u8, size, appid);
                                d.allow(appid, 1, slice)
                            } else {
                                -1
                            }
                        },
                        None => -1
                    }
                });
                
                          
                let callback_ptr = process.r2() as *mut ();
                let appdata = process.r3();
                
                let res = platform.with_driver(0, |driver| {
                    let callback =
                        hil::Callback::new(appid, appdata, callback_ptr);
                    match driver {
                        Some(d) => d.subscribe(1,
                                               callback),
                        None => -1
                    }
                });
                process.set_r0(res);
                
                
                
            },
            _ => {println!("end of the line"); }
        
    }
    }
}
