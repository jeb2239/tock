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

