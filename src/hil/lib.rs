#![crate_name = "hil"]
#![crate_type = "rlib"]
#![feature(asm,lang_items,const_fn)]
#![no_std]

extern crate common;
extern crate process;

pub mod driver;

pub mod led;
pub mod alarm;
pub mod gpio;
pub mod i2c;
pub mod spi_master;
pub mod timer;
pub mod uart;
pub mod adc;

pub use driver::Driver;

pub use process::{Callback, AppSlice, Shared, AppId};
pub use process::process::NUM_PROCS;

pub trait Controller {
    type Config;

    fn configure(&self, Self::Config);
}

//compare with tiny os
// do a null system
//what is a null system call perfromance 
//compare 


//something that takes, something that has arguments
//microbenchmarks
//pain a picture of what the overhead looks
// arm -- system clock
// checks 