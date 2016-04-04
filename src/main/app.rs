
#[allow(improper_ctypes)]
extern {
    fn __allow(driver_num: usize, allownum: usize, ptr: *mut (), len: usize) -> isize;
    fn __subscribe(driver_num: usize, subnum: usize, cb: usize, appdata: usize) -> isize;
    fn __command(driver_num: usize, cmdnum: usize, arg1: usize) -> isize;
    fn __wait() -> isize;
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


pub fn wait() -> isize {
    unsafe {
        __wait()
    }
}


pub fn rust_app() -> ! {

        // fn time_repeat_sub(cb:usize)-> isize{
        //     subscribe(3,1,cb,0)
            
        // }

        // fn timer_cb() -> isize {
        //     let a =toggle_pin(0);
        //     println!("in timer_cb");
        //     a 
        // }

        // let timer_cb = timer_cb as usize;
        

       // println!("{:?}",add(3,1));
       // set_pin(0);
       // toggle_pin(0);
      //   println!("{:?}",time_repeat_sub(timer_cb));
      //   //println!("{:?}");
      //   //  println!("hello");
      //  // let a = enable_pin(0);
      //  // println!("{:?}",a);
      //   println!("Hello" );
      // //  let b = set_pin(0);
      // //  println!("{:?}",b);
      //   println!("Hello");

        
      //  sam4l::gpio::enable_output(10);

      //  sam4l::gpio::PC[10].enable_output();
      // sam4l::gpio::PC[10].set();
     

        

        loop {
            wait();
        }

}

