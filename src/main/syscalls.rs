#[allow(improper_ctypes)]
extern {
    fn __safe_call(driver_num: usize, subnum: usize, cb: usize, appdata: usize ) -> isize;
    fn __allow(driver_num: usize, allownum: usize, ptr: *mut (), len: usize) -> isize;
    fn __subscribe(driver_num: usize, subnum: usize, cb: usize, appdata: usize) -> isize;
    fn __command(driver_num: usize, cmdnum: usize, arg1: usize) -> isize;
    fn __fast_print_async(ptr:*mut (), len: usize, cb:usize, appdata:usize) -> isize;
    fn __wait() -> isize;
    fn __start_count() -> isize;
    fn __end_count() -> isize;
    fn __enable_and_set(driver_num: usize, cmdnum: usize, arg1: usize, arg2: usize) -> isize;
    fn switch_to_user(user_stack: *mut u8, mem_base: *mut u8) -> *mut u8;
    
}

pub fn start_count() -> isize {
  unsafe{
    __start_count()
  }
}

pub fn fast_print_async(ptr: *mut (), len: usize, cb: usize, appdata:usize) -> isize {
    
    unsafe{
        __fast_print_async(ptr,len,cb,appdata)
    }
    
}

pub fn end_count() -> isize {
  unsafe{
    __end_count()
  }
}

pub fn safe_call(driver_num: usize, subnum: usize, cb: usize, appdata: usize) -> isize {
    unsafe{
        __safe_call(driver_num, subnum, cb, appdata)
    }
}
pub fn allow(driver_num: usize, allownum: usize, ptr: *mut (), len: usize) -> isize {
    unsafe {
      __allow(driver_num, allownum, ptr, len)
    }
}

pub fn command(driver_num: usize, cmdnum: usize, arg1: usize) -> isize {
    unsafe {
        __command(driver_num, cmdnum, arg1)
    }
}

pub fn subscribe(driver_num: usize, cmdnum: usize, callback: usize, appdata: usize) -> isize {
    unsafe {
        __subscribe(driver_num, cmdnum, callback, appdata)
    }
}

pub fn enable_pin(pin: usize) -> isize {
    command(1, 0, pin)
}

pub fn set_pin(pin: usize) -> isize{
    command(1, 1, pin) //1,2,pin
}

pub fn clear_pin(pin: usize) -> isize {
    command(1, 2, pin) //3
}

pub fn toggle_pin(pin: usize) -> isize {
    command(1, 3, pin) //4
}

pub fn enable_and_set(pin: usize) -> isize {
    unsafe{
        __enable_and_set(1,0,1,pin) // arg1 = driver number
                                    // arg2 = first command
                                    // arg3 = second command
                                    // arg4 = pin number
    }
}

pub fn enable_and_set_cmd(pin: usize) -> isize {
    unsafe{
        __command(1,0,pin);
        __command(1,1,pin)
    }
}


pub fn wait() -> isize {
    unsafe {
        __wait()
    }
}