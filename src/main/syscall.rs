pub const WAIT: u8 = 0;
pub const SUBSCRIBE: u8 = 1;
pub const COMMAND: u8 = 2;
pub const ALLOW: u8 = 3;

//so the only way we know which system call we need to handle is
// based on the number loaded in by the svc call, that way when we return to the kernel we know what syscall
//the application made


pub enum ReturnTo {
  Process = 0,
  Kernel = 1
}

