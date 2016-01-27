#![crate_name = "cortexm4"]
#![crate_type = "rlib"]
#![feature(core_intrinsics,const_fn)]
#![no_std]

use core::intrinsics;

pub const BASE_ADDRESS: usize = 0xE000ED90;

#[repr(C, packed)]
struct Registers {
    mpu_type:   u32,
    mpu_ctrl:   u32,
    mpu_rnr:    u32,
    mpu_rbar:   u32,
    mpu_rasr:   u32,
    mpu_rbar1:  u32,
    mpu_rasr1:  u32,
    mpu_rbar2:  u32,
    mpu_rasr2:  u32,
    mpu_rbar3:  u32,
    mpu_rasr3:  u32
}

pub static mut MPU : Mpu = Mpu::new();

pub struct Mpu {
    regs: *mut Registers
}

impl Mpu {
    const fn new() -> Mpu {
        Mpu {
            regs: BASE_ADDRESS as *mut Registers
        }
    }

    pub unsafe fn num_regions(&self) -> u8 {
        (intrinsics::volatile_load(&(*self.regs).mpu_type) >> 8) as u8
    }
}

