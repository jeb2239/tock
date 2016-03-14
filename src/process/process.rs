use core::intrinsics::{breakpoint, volatile_load, volatile_store};
use core::mem;
use core::raw::{Repr,Slice};

use common::{RingBuffer, Queue};

#[allow(improper_ctypes)]
extern {
    pub fn switch_to_user(user_stack: *const u8, mem_base: *const u8) -> *mut u8;
}

/// Size of each processes's memory region in bytes
pub const PROC_MEMORY_SIZE : usize = 8192;
pub const NUM_PROCS : usize = 1;

static mut FREE_MEMORY_IDX: usize = 0;

#[link_section = ".app_memory"]
static mut MEMORIES: [[u8; PROC_MEMORY_SIZE]; NUM_PROCS] = [[0; PROC_MEMORY_SIZE]; NUM_PROCS];

pub static mut PROCS : [Option<Process<'static>>; NUM_PROCS] = [None];

pub fn schedule(callback: Callback, appid: ::AppId) -> bool {
    let procs = unsafe { &mut PROCS };
    let idx = appid.idx();
    if idx >= procs.len() {
        return false
    }

    match procs[idx] {
        None => false,
        Some(ref mut p) => {
            // TODO(alevy): validate appid liveness
            p.callbacks.enqueue(callback)
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum State {
    Running,
    Waiting
}


#[derive(Copy,Clone,Debug)]
pub struct Callback {
    pub r0: usize,
    pub r1: usize,
    pub r2: usize,
    pub r3: usize,
    pub pc: usize
}

#[repr(C,packed)]
struct LoadInfo {
    rel_data_size: usize,
    entry_loc: usize,        /* Entry point for user application */
    init_data_loc: usize,    /* Data initialization information in flash */
    init_data_size: usize,    /* Size of initialization information */
    got_start_offset: usize,  /* Offset to start of GOT */
    got_end_offset: usize,    /* Offset to end of GOT */
    bss_start_offset: usize,  /* Offset to start of BSS */
    bss_end_offset: usize    /* Offset to end of BSS */
}

pub struct Process<'a> {
    /// The process's memory.
    memory: Slice<u8>,

    memory_break: *const u8,

    /// Process text segment
    text: Slice<u8>,

    /// The offset in `memory` to use for the process stack.
    cur_stack: *const u8,

    wait_pc: usize,
    psr: usize,

    pub state: State,

    pub callbacks: RingBuffer<'a, Callback>
}

impl<'a> Process<'a> {
    pub unsafe fn create(start_addr: *const usize) -> Option<Process<'a>> {
        let cur_idx = FREE_MEMORY_IDX;
        if cur_idx <= MEMORIES.len() {
            FREE_MEMORY_IDX += 1;
            let memory = MEMORIES[cur_idx].repr();

            let stack_bottom = memory.data.offset(256);

            // Take callback buffer from bottom of process memory
            let callback_len = 10;
            let callback_buf = mem::transmute(Slice {
                data: memory.data as *const Option<Callback>,
                len: callback_len
            });
            let callback_size = mem::size_of::<Option<Callback>>();

            let callbacks = RingBuffer::new(callback_buf);

            let memory_break = memory.data.offset(
                    (memory.len - callback_len * callback_size) as isize);

            let load_result = load(start_addr, memory.data);

            let mut process = Process {
                memory: memory,
                memory_break: memory_break,
                text: Slice {
                    data: load_result.text_start,
                    len: load_result.text_len },
                cur_stack: stack_bottom,
                wait_pc: 0,
                psr: 0x01000000,
                state: State::Waiting,
                callbacks: callbacks
            };

            process.callbacks.enqueue(Callback {
                pc: load_result.init_fn,
                r0: load_result.app_mem_start as usize,
                r1: process.memory_break as usize,
                r2: 0,
                r3: 0
            });

            Some(process)
        } else {
            None
        }
    }


    pub fn in_exposed_bounds(&self, buf_start_addr: *const u8, size: usize)
            -> bool {

        let buf_end_addr = ((buf_start_addr as usize) + size) as *const u8;

        let mem_end =
            ((self.memory.data as usize) + self.memory.len) as *const u8;

        buf_start_addr >= self.memory.data && buf_end_addr <= mem_end
    }

    pub unsafe fn container_for(&self, container_num: usize)
            -> Option<*mut u8> {
        None
    }

    pub unsafe fn alloc(&mut self, size: usize) -> Option<&mut [u8]> {
        None
    }

    pub unsafe fn free<T>(&mut self, _: *mut T) {}

    pub fn pop_syscall_stack(&mut self) {
        let pspr = self.cur_stack as *const usize;
        unsafe {
            self.wait_pc = volatile_load(pspr.offset(6));
            self.psr = volatile_load(pspr.offset(7));
            self.cur_stack =
                (self.cur_stack as *mut usize).offset(8) as *mut u8;
        }
    }

    /// Context switch to the process.
    pub unsafe fn switch_to_callback(&mut self, callback: Callback) {
        // Fill in initial stack expected by SVC handler
        // Top minus 8 u32s for r0-r3, r12, lr, pc and xPSR
        let stack_bottom = (self.cur_stack as *mut usize).offset(-8);
        volatile_store(stack_bottom.offset(7), self.psr);
        volatile_store(stack_bottom.offset(6), callback.pc | 1);
        // Set the LR register to the saved PC so the callback returns to
        // wherever wait was called. Set lowest bit to one because of THUMB
        // instruction requirements.
        volatile_store(stack_bottom.offset(5), self.wait_pc | 0x1);
        volatile_store(stack_bottom, callback.r0);
        volatile_store(stack_bottom.offset(1), callback.r1);
        volatile_store(stack_bottom.offset(2), callback.r2);
        volatile_store(stack_bottom.offset(3), callback.r3);

        self.cur_stack = stack_bottom as *mut u8;
        self.switch_to();
    }

    /// Context switch to the process.
    pub unsafe fn switch_to(&mut self) {
        if self.cur_stack < self.memory.data {
            breakpoint();
        }
        let psp = switch_to_user(self.cur_stack, self.memory.data);
        self.cur_stack = psp;
    }

    pub fn svc_number(&self) -> Option<u8> {
        let psp = self.cur_stack as *const *const u16;
        unsafe {
            let pcptr = volatile_load((psp as *const *const u16).offset(6));
            let svc_instr = volatile_load(pcptr.offset(-1));
            Some((svc_instr & 0xff) as u8)
        }
    }

    pub fn lr(&self) -> usize {
        let pspr = self.cur_stack as *const usize;
        unsafe { volatile_load(pspr.offset(5)) }
    }


    pub fn r0(&self) -> usize {
        let pspr = self.cur_stack as *const usize;
        unsafe { volatile_load(pspr) }
    }

    pub fn set_r0(&mut self, val: isize) {
        let pspr = self.cur_stack as *mut isize;
        unsafe { volatile_store(pspr, val) }
    }

    pub fn r1(&self) -> usize {
        let pspr = self.cur_stack as *const usize;
        unsafe { volatile_load(pspr.offset(1)) }
    }

    pub fn r2(&self) -> usize {
        let pspr = self.cur_stack as *const usize;
        unsafe { volatile_load(pspr.offset(2)) }
    }

    pub fn r3(&self) -> usize {
        let pspr = self.cur_stack as *const usize;
        unsafe { volatile_load(pspr.offset(3)) }
    }

}

struct LoadResult {
    text_start: *const u8,
    text_len: usize,
    init_fn: usize,
    app_mem_start: *const u8
}

unsafe fn load(start_addr: *const usize, mem_base: *const u8) -> LoadResult {
    let mut result = LoadResult {
        text_start: 0 as *const u8,
        text_len: 0,
        init_fn: 0,
        app_mem_start: 0 as *const u8
    };

    let mut start_addr = start_addr as *const u8;
    let load_info : &LoadInfo = mem::transmute(start_addr);
    start_addr = start_addr.offset(mem::size_of::<LoadInfo>() as isize);

    let rel_data : &mut [usize] = mem::transmute(Slice{
        data: start_addr,
        len: load_info.rel_data_size / 4
    });
    start_addr = start_addr.offset(load_info.rel_data_size as isize);

    // Update text location in self
    result.text_start = start_addr;
    result.text_len = load_info.init_data_loc;

    // Zero out BSS
    ::core::intrinsics::write_bytes(
        mem_base.offset(load_info.bss_start_offset as isize) as *mut u8,
        0,
        load_info.bss_end_offset - load_info.bss_start_offset);

    // Copy data into Data section
    let init_data : &[u8] = mem::transmute(Slice{
        data: (load_info.init_data_loc + start_addr as usize) as *mut u8,
        len: load_info.init_data_size
    });

    let target_data : &mut [u8] = mem::transmute(Slice{
        data: mem_base,
        len: load_info.init_data_size
    });

    target_data.clone_from_slice(init_data);

    let fixup = |addr: *mut usize| {
        let entry = *addr;
        if (entry & 0x80000000) == 0 {
            // Regular data (memory relative)
            *addr = entry + (mem_base as usize);
        } else {
            // rodata or function pointer (code relative)
            *addr = (entry ^ 0x80000000) + (start_addr as usize);
        }
    };

    // Fixup Global Offset Table
    let mut got_cur = mem_base.offset(
        load_info.got_start_offset as isize) as *mut usize;
    let got_end = mem_base.offset(
        load_info.got_end_offset as isize) as *mut usize;
    while got_cur != got_end {
        fixup(got_cur);
        got_cur = got_cur.offset(1);
    }

    // Fixup relocation data
    for (i, addr) in rel_data.iter().enumerate() {
        if i % 2 == 0 { // Only the first of every 2 entries is an address
            fixup(mem_base.offset(*addr as isize) as *mut usize);
        }
    }

    // Entry point is offset from app code
    result.init_fn = start_addr as usize + load_info.entry_loc;

    result.app_mem_start = mem_base.offset(load_info.bss_end_offset as isize);

    result
}

