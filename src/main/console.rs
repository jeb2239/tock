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
macro_rules! print_as {
    ($str:expr) => (puts(String::new($str)));
    ($fmt:expr, $($arg:tt)*) => (print_async(format_args!($fmt,$($arg)*)));
}

pub fn print_async(args: fmt::Arguments ){
    use core::fmt::Write;
    let mut string = String::new("");
    let _ = string.write_fmt(args);
    puts(string);
}


pub fn puts(string: String){
    
    syscalls::allow(0, 1, string.as_str() as *const str as *mut (), string.len());
    let bx = Box::new(string);
    syscalls::subscribe(0, 1, write_done as usize, bx.raw() as usize);
    mem::forget(bx);
    while syscalls::wait() != WRITE_DONE_TOKEN {}
    
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


// const WRITE_DONE_TOKEN : isize = 0xbeef;

// fn write_done(_: usize, _: usize, _: usize, strptr: *mut String) -> isize {
//     unsafe { mem::drop(Box::<String>::from_raw(strptr)); }
//     WRITE_DONE_TOKEN
// }

// macro_rules! print {
//     ($str:expr) => (puts(super::string::String::new($str)));
//     ($fmt:expr, $($arg:tt)*) => (print(format_args!($fmt, $($arg)*)));
// }

// pub fn print(args: ::core::fmt::Arguments) {
//     use core::fmt::Write;
//     let mut string = String::new("");
//     let _ = string.write_fmt(args);
//     puts(string);
// }

// pub fn puts(string: String) {
//     allow(0, 1, string.as_str() as *const str as *mut (), string.len());
//     let bx = Box::new(string);
//     subscribe(0, 1, write_done as usize, bx.raw() as usize);
//     mem::forget(bx);
//     while wait() != WRITE_DONE_TOKEN {}
// }

// #[allow(dead_code)]
// pub fn putc(c: u8) {
//     command(0, 0, c as usize);
// }

// #[allow(dead_code)]
// pub fn subscribe_read_line(buf: *mut u8, len: usize,
//                            f: fn(usize, *mut u8)) -> isize {
//     let res =  allow(0, 0, buf as *mut (), len);
//     if res < 0 {
//         res
//     } else {
//         subscribe(0, 0, f as usize, 0)
//     }
// }

