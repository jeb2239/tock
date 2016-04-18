use platform::Firestorm;
use process;
use process::Process;
use process::{AppSlice,AppId};
use common::Queue;
use hil;
use syscall;

pub unsafe fn do_process(platform: &mut Firestorm, process: &mut Process,
                  appid: AppId) {
    loop {
        match process.state {  //so here we have to check the state of the process running
            process::State::Running => {
                process.switch_to();
            }
            process::State::Waiting => {
                match process.callbacks.dequeue() {
                    None => { return },
                    Some(cb) => {
                        process.state = process::State::Running;
                        process.switch_to_callback(cb);
                    }
                }
            }
        }
        match process.svc_number() {
            Some(syscall::WAIT) => {
                process.state = process::State::Waiting;
                process.pop_syscall_stack();
                break;
            },
            Some(syscall::SUBSCRIBE) => {
                let driver_num = process.r0();
                let subdriver_num = process.r1();// ----- in stead of passing in a number, just pass in a pointer to the driver 
                                                //function
                                                
                let callback_ptr = process.r2() as *mut ();
                let appdata = process.r3();
                println!("{:?}",driver_num);
                println!("{:?}",subdriver_num );
                println!("{:?}",callback_ptr );
                println!("{:?}", appdata );
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
            }
            _ => {}
        }
    }
}
