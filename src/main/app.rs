

use platform::Firestorm;
use process;
use process::Process;
use process::{AppSlice,AppId};
use common::Queue;
use hil;
use syscall;
use core::raw::Slice;
use core::ops::{Deref,DerefMut};
use core::ptr::Unique;

//use super::boxed::BoxMgr;
#[allow(improper_ctypes)]
extern {
    fn __allow(driver_num: usize, allownum: usize, ptr: *mut (), len: usize) -> isize;
    fn __subscribe(driver_num: usize, subnum: usize, cb: usize, appdata: usize) -> isize;
    fn __command(driver_num: usize, cmdnum: usize, arg1: usize) -> isize;
    fn __wait() -> isize;
    fn __start_count() -> isize;
    fn __end_count() -> isize;
    fn switch_to_user(user_stack: *mut u8, mem_base: *mut u8) -> *mut u8;
}

pub fn start_count() -> isize {
  unsafe{
    __start_count()
  }
}



pub fn end_count() -> isize {
  unsafe{
    __end_count()
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


pub fn wait() -> isize {
    unsafe {
        __wait()
    }
}
// pub fn enable_tmp006() {
//     command(2, 0, 0);
// }

// pub fn subscribe_temperature(f: fn(i16)) {
//     subscribe(2, 0, f as usize);
// }


// pub struct App {
//     pub memory: BoxMgr
// }
// static mut app: App = 0 as *mut App;
// #[derive(Clone,Copy)]
// #[repr(C)]
// pub struct Chunk {
//     pub inuse: bool,
//     pub len: usize
// }

// impl Chunk {
//     unsafe fn data(&self) -> *const u8 {
//         (self as *const Chunk).offset(1) as *const u8
//     }
// }

// pub struct BoxMgr {
//     pub mem: Slice<u8>,
//     pub offset: usize,
//     pub drops: usize,
//     pub allocs: usize,
//     pub chunks: [*mut Chunk; 100]
// }

// pub struct BoxMgrStats {
//     pub allocated_bytes: usize,
//     pub num_allocated: usize,
//     pub active: usize,
//     pub allocs: usize,
//     pub drops: usize,
//     pub free: usize
// }

// impl BoxMgr {
//     #[inline(never)]
//     pub fn new(mem_start: *mut u8, mem_size: usize, appsize: usize) -> BoxMgr {
//         BoxMgr {
//             mem: Slice {
//                 data: unsafe { mem_start.offset(appsize as isize) },
//                 len: mem_size - appsize
//             },
//             offset: 0,
//             drops: 0,
//             allocs: 0,
//             chunks: [0 as *mut Chunk; 100]
//         }
//     }

//     pub fn stats(&self) -> BoxMgrStats {
//         let allocated = self.offset;
//         let num_allocated = self.chunks.iter().
//                 filter(|c| !c.is_null()).count();
//         let active = unsafe {
//             self.chunks.iter().
//                 filter(|c| !c.is_null() && (***c).inuse).count()
//         };
//         BoxMgrStats {
//             allocated_bytes: allocated,
//             num_allocated: num_allocated,
//             active: active,
//             drops: self.drops,
//             allocs: self.allocs,
//             free: self.mem.len - num_allocated
//         }
//     }
// }

// pub struct Box<T>{ inner: Unique<T> }

// impl<T> Box<T> {
    
//     pub unsafe fn from_raw(raw: *mut T) -> Box<T> {
//         Box { inner: Unique::new(raw) }
//     }

//     pub fn raw(&self) -> *mut T {
//         *self.inner
//     }

//     pub unsafe fn uninitialized(size: usize) -> Box<T> {
//         use core::mem;
//         let myapp = &mut (&mut *app).memory;
//         myapp.allocs += 1;

//         // First, see if there is an available chunk of the right size
//         for chunk in myapp.chunks.iter_mut().filter(|c| !c.is_null()) {
//             let c : &mut Chunk = mem::transmute(*chunk);
//             if !c.inuse && c.len >= size {
//                 c.inuse = true;
//                 let data = c.data();
//                 return Box {
//                     inner: Unique::new(data as *mut T)
//                 };
//             }
//         }

//         match myapp.chunks.iter_mut().filter(|c| c.is_null()).next() {
//             Some(slot) => {
//                 let freemem = myapp.mem.data.offset(myapp.offset as isize);
//                 let chunk = &mut *(freemem as *mut Chunk);
//                 myapp.offset += mem::size_of::<Chunk>();

//                 let chunk_align = mem::align_of::<Chunk>();
//                 let size = if size % chunk_align == 0 {
//                     size
//                 } else {
//                     size + chunk_align - (size % chunk_align)
//                 };
//                 chunk.len = size;
//                 chunk.inuse = true;

//                 *slot = chunk as *mut Chunk;

//                 let data = myapp.mem.data.offset(myapp.offset as isize);
//                 myapp.offset += size;

//                 Box{ inner: Unique::new(data as *mut T) }
//             },
//             None => {
//                 panic!("OOM")
//             }
//         }
//     }

//     pub fn new(x: T) -> Box<T> {
//         use core::mem;
//         use core::intrinsics::copy;

//         let size = mem::size_of::<T>();
//         unsafe {
//             let mut d = Self::uninitialized(size);
//             copy(&x, &mut *d, 1);
//             mem::forget(x);
//             d
//         }
//     }
// }

// impl<T> Deref for Box<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         unsafe {
//             &**self.inner
//         }
//     }
// }

// impl<T> DerefMut for Box<T> {
//     fn deref_mut(&mut self) -> &mut T {
//         unsafe {
//             &mut **self.inner
//         }
//     }
// }

// impl<T> Drop for Box<T> {
//     fn drop(&mut self) {
//         unsafe {
//             use core::{mem, ptr};

//             mem::drop(ptr::read(*self.inner));

//             let chunk = (*self.inner as *mut T as *mut Chunk).offset(-1);
//             (&mut *chunk).inuse = false;
//             let myapp = &mut (*app).memory;
//             myapp.drops += 1;
//         }
//     }
// }

// pub unsafe fn uninitialized_box_slice<T>(size: usize) -> Box<&'static mut [T]> {
//     use core::mem;
//     let slice_size = mem::size_of::<Slice<u8>>();
//     let mut bx : Box<Slice<u8>> =
//         Box::uninitialized(slice_size + size * mem::size_of::<T>());
//     bx.len = size;
//     bx.data = (bx.raw()).offset(1) as *const u8;
//     mem::transmute(bx)
// }



use super::boxed::BoxMgr;

pub struct App {
    pub memory: BoxMgr,
    pub platform: Firestorm
}

pub fn init() {
    print!("Welcome to Tock!\r\n");

    let stats = (unsafe { &*super::app }).memory.stats();
    print!("Memory Stats:{}\r\n", "");
    print!("\tNum Allocated: {}\r\n", stats.num_allocated);
    print!("\tNum Allocs: {}\r\n", stats.allocs);
    print!("\tDrops: {}\r\n", stats.drops);
    print!("\tAllocated Bytes: {}\r\n", stats.allocated_bytes);
    print!("\tFree Bytes: {}\r\n", stats.free);
    print!("\tActive: {}\r\n", stats.active);
    
}


//pub fn rust_app() -> ! {
   /*       enable_pin(0);

     /*    fn time_repeat_sub(cb:usize)-> isize{
             println!("{:?}","yo" );
             subscribe(3,1,cb,0)
            
         }

         fn timer_cb() -> isize {
             let a =toggle_pin(0);
             println!("in timer_cb");
             a 
         }

         let timer_cb = timer_cb as usize;*/
       enable_pin(0);

       // println!("{:?}",add(3,1));
        start_count();
        //for i in 0..300{
        
        toggle_pin(0);
       // }
      // set_pin(0);
       println!("{:?}", end_count());
       // toggle_pin(0);
    //    time_repeat_sub(timer_cb);
      
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
      //println!("{:?}", 4);
      // enable_pin(0); //commands are not pending interupts
      // start_count();
      // set_pin(0);
      // let count =end_count();
      // println!("{:?}",count );
      loop{
      wait(); // we make a syscal so this causes us to jump out of do process;
      }
        */
        

      //  loop {
       //     wait();
        //}

//}

