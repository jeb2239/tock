# Testing journal
note that all tests are run with one application and the code snippets was the only code in said app

### Turn on the led using 2 command syscalls wrapped separately
 time = 616 cycles 1 trial

```rust

  syscalls::start_count();
  syscalls::enable_pin(0);
  syscalls::set_pin(0);
  let a = syscalls::end_count();
  print!("{}\r\n",a);


```
 time = 154007 cycles 300 trials
 averages out to about 514
```rust
syscalls::start_count();
  for i in 0..300 {

  syscalls::enable_pin(0);
  syscalls::set_pin(0);

  }

  let a = syscalls::end_count();
  print!("{}\r\n",a);

```

time = 256607 cycles 500 trials  
       = 513107 cycles 1000 trials


```rust

pub fn enable_pin(pin: usize) -> isize {
    command(1, 0, pin)
}

pub fn set_pin(pin: usize) -> isize{
    command(1, 1, pin)
}

```


### Turn on the led using enable_and_set
 time = 435 cycles 1 trial

```rust
  syscalls::start_count();
  syscalls::enable_and_set(0);
  let a = syscalls::end_count();
  print!("{}\r\n",a);

```

 time = 98213 cycles over 300 trials

```rust
syscalls::start_count();
  for i in 0..300 {

 syscalls::enable_and_set(0);

  }
  let a = syscalls::end_count();
  print!("{}\r\n",a);
 ```
time = 163613  cycles over 500 trials
       327113  cycles over 1000 trials
               cycles over 2000 trials
               cycles over 4000 trials

#### Implementation of enable_and_set that uses 1 kernel interrupt

```rust

pub fn enable_and_set(pin: usize) -> isize {
    unsafe{
        __enable_and_set(1,0,1,pin) // arg1 = driver number
                                    // arg2 = first command
                                    // arg3 = second command
                                    // arg4 = pin number
    }
}

```

### Turn on the led using 2 commands wrapped together

 time = 616 cycles 1 trial
    * the prior example probably optimized away up to the foreign function calls
```rust

  syscalls::start_count();
  syscalls::enable_and_set_cmd(0);
  let a = syscalls::end_count();
  print!("{}\r\n",a);


```
#### Implementation of enable_and_set_cmd
```rust

    pub fn enable_and_set_cmd(pin: usize) -> isize {
    unsafe{
        __command(1,0,pin);
        __command(1,1,pin)
    }
}

```




# Printing to console asynchronously

time =  5401 cycles 1 trial

using new implementation

```rust

syscalls::start_count();

  print_as_fast!("Hello");

  let a = syscalls::end_count();
  print!("{}\r\n",a);

```
time = 54745 for 10 tries


time = out of memory for 300
        for 500
        for 1000

--------------------------------------------------------------------------
time = 4910 for 1 tri

time = 49016 for 10 tries


```rust

 syscalls::start_count();
 for i in 0..10 {

  print_as_slow!("Hello");

  }
  let a = syscalls::end_count();
  print!("{}\r\n",a);


```

Implementation
---------------

```rust
use core::mem;
use super::boxed::Box;
use syscall;
use syscalls;
use super::string::String;
use core::fmt::{self, Write};

const WRITE_DONE_TOKEN : isize = 0xbeef;


fn write_done(_:usize,_ :usize, strptr: *mut String) -> isize {

    unsafe {
        mem::drop(Box::<String>::from_raw(strptr));
    }
    WRITE_DONE_TOKEN   
}
#[macro_export]
macro_rules! print_as_fast {
    ($str:expr) => (puts_wrapper(String::new($str)));
    ($fmt:expr, $($arg:tt)*) => (print_async(format_args!($fmt,$($arg)*)));
}

#[macro_export]
macro_rules! print_as_slow {
    ($str:expr) => (puts_old(String::new($str)));
    ($fmt:expr, $($arg:tt)*) => (print_old(format_args!($fmt,$($arg)*)));
}

pub fn print_async(args: fmt::Arguments ){
    use core::fmt::Write;
    let mut string = String::new("");
    let _ = string.write_fmt(args);
    let a = string.as_str() as *const str as *mut(); //copy
    let b = string.len(); //copy
    puts(string,a ,b );
}

pub fn puts_wrapper(string : String){
    //let mut string = String::new("");
    //let a = string.write_str(instr) as *const str as *mut();
    let a = string.as_str() as *const str as *mut (); //copy
    let b = string.len(); //copy

    puts(string,a , b);

}

//take this model and push this down to the driver level
//we will already be in the kernel so this will cost one context switch
// instead of two
pub fn puts(string: String,a:*mut (),b:usize){

    let bx = Box::<String>::new(string);
    syscalls::fast_print_async( a , b,
    write_done as usize,bx.raw() as usize);
    mem::forget(bx);
  //  while syscalls::wait() != WRITE_DONE_TOKEN {}

}

pub fn print_old(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    let mut string = String::new("");
    let _ = string.write_fmt(args);
    puts_old(string);
}


pub fn puts_old(string: String) {
    syscalls::allow(0, 1, string.as_str() as *const str as *mut (), string.len());
    let bx = Box::new(string);
    syscalls::subscribe(0, 1, write_done as usize, bx.raw() as usize);
    mem::forget(bx);
  //  while syscalls::wait() != WRITE_DONE_TOKEN {}
}

#[allow(dead_code)]
pub fn putc(c: u8){
    syscalls::command(0, 1, c as usize);
}

#[allow(dead_code)]
pub fn subscribe_read_line(buf: *mut u8, len: usize,
                           f: fn(usize, *mut u8)) -> isize {
    let res =  syscalls::allow(0, 1, buf as *mut (), len);
    if res < 0 {
        res
    } else {
        syscalls::subscribe(0, 1, f as usize, 0)
    }
}




```

Other critical area of modification
------------------------------------

When we trap to the kernel this function is called


```rust

use platform::Firestorm;
use process;
use process::Process;
use process::{AppSlice,AppId,Callback};
use common::Queue;

use hil::{Shared};
use hil;
use syscall;
use app;






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
              Some(syscall::FAST_PRINT_ASYNC) => {
               // println!("FAST_PRINT_ASYNC");
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
            Some(syscall::SUBSCRIBE) => {
              //  println!("SUBSCRIBE");
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
               // println!("ALLOW");
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
            Some(syscall::ENABLE_AND_SET) => {


            /*    let driver_num = process.r0(); //
                let subdriver_num = process.r1(); //
                let callback_ptr = process.r2(); // this will hold our pin number
                let appdata = process.r3(); //just maybe this is useful
                */
             //   println!("{}",process.r0());
              //  println!("{}",process.r1());
              //  println!("{}",process.r2());
              //  println!("{}",process.r3());
                let res = platform.with_driver(process.r0(),|driver| {
                    match driver {
                        Some(d) =>{ d.command(process.r1(),process.r3());
                                    d.command(process.r2(),process.r3()) },
                        None => -1
                    }
                });
                process.set_r0(res);





            },


            _ => {println!("end of the line"); }

    }
    }
}




```


TinyOS tests
-----------------------

This is blinking the light

```c

// $Id: BlinkC.nc,v 1.6 2010-06-29 22:07:16 scipio Exp $

/*									tab:4
 * Copyright (c) 2000-2005 The Regents of the University  of California.  
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the
 *   distribution.
 * - Neither the name of the University of California nor the names of
 *   its contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Copyright (c) 2002-2003 Intel Corporation
 * All rights reserved.
 *
 * This file is distributed under the terms in the attached INTEL-LICENSE     
 * file. If you do not find these files, copies can be found by writing to
 * Intel Research Berkeley, 2150 Shattuck Avenue, Suite 1300, Berkeley, CA,
 * 94704.  Attention:  Intel License Inquiry.
 */

/**
 * Implementation for Blink application.  Toggle the red LED when a
 * Timer fires.
 **/

#include "Timer.h"
#include "printf.h"
#include <usarthardware.h>
#include <stdint.h>



module BlinkC @safe()
{
  uses interface Timer<TMilli> as Timer0;
  uses interface Timer<TMilli> as Timer1;
  uses interface Timer<TMilli> as Timer2;
  uses interface GeneralIO as Led;
  uses interface HplSam4lUSART as SpiHPL;
  uses interface Boot;
  uses interface SpiPacket;
}
implementation
{
  event void Boot.booted()
  {
    call Timer0.startPeriodic( 250 );
    call Timer1.startPeriodic( 500 );
    call Timer2.startPeriodic( 1000 );
    call Led.makeOutput();
    printf("Configuring SPI\n");
    //Because you can have one usart present on multiple pins (like multiple TX pins)
    //you need to speak to the HPL directly. Not sure what the best way to implement
    //this is.
    call SpiHPL.enableUSARTPin(USART2_TX_PC12);
    call SpiHPL.enableUSARTPin(USART2_RX_PC11);
    call SpiHPL.enableUSARTPin(USART2_CLK_PA18);
    call SpiHPL.enableUSARTPin(USART2_RTS_PC07);
    call SpiHPL.initSPIMaster();
    call SpiHPL.setSPIMode(0,0);
    call SpiHPL.setSPIBaudRate(20000);
    call SpiHPL.enableTX();
    call SpiHPL.enableRX();

  }

  async event void SpiPacket.sendDone(uint8_t* txBuf, uint8_t* rxBuf, uint16_t len, error_t error)
  {
    printf("got: '%s'",rxBuf);
  }
  uint8_t txbuf [80];
  uint8_t rxbuf [80];
  volatile uint32_t count = 0;

// addresses of registers
  volatile uint32_t *DWT_CONTROL = (uint32_t *)0xE0001000;
  volatile uint32_t *DWT_CYCCNT = (uint32_t *)0xE0001004;
  volatile uint32_t *DEMCR = (uint32_t *)0xE000EDFC;

  extern uint32_t __start_count()
{


// enable the use DWT
*DEMCR = *DEMCR | 0x01000000;

// Reset cycle counter
*DWT_CYCCNT = 0;

// enable cycle counter
*DWT_CONTROL = *DWT_CONTROL | 1 ;

}



extern uint32_t __end_count()
{
// number of cycles stored in count variable
count = *DWT_CYCCNT;
return count;
}
uint32_t a=0;
  event void Timer0.fired()
  {

     __start_count(); //this interval is 4
    //dbg("BlinkC", "Timer 0 fired @ %s.\n", sim_time_string());
    //call Led.toggle();
     call Led.toggle();

     a=__end_count();

    printf("%d\n",a);
  }

  event void Timer1.fired()
  {
    dbg("BlinkC", "Timer 1 fired @ %s \n", sim_time_string());
    call Led.toggle();
  }

  event void Timer2.fired()
  {
    dbg("BlinkC", "Timer 2 fired @ %s.\n", sim_time_string());
    call Led.toggle();
  }
}



```

Toggling the led is much much faster on TinyOS because
it is not performing any context switch. When an application in
tiny OS needs a resource it turns it on directly. When an application in
Tock OS needs a resource the requesting process must make an svc call
and save all of the current processes registers.

A consequence of not using hardware protection or other means of separation
means that a buggy program can take over the microcontroller and prevent it
from performing other tasks. Because Tock separates applications at both a hardware,
process, and language level it is more tolerant to faults. Also it makes it impossible
for an application to take over the microcontroller's resources.
