/* vim: set sw=2 expandtab tw=80: */

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <firestorm.h>

char hello[] = "Hello World!\r\n";

CB_TYPE nop(int x, int y, int z, void *ud) { return ASYNC; }

void main() {
  putnstr_async(hello, sizeof(hello), nop, NULL);
  //add command system call witch is a no op.....
  //pic a driver add a system call number for nop -- 
  //end to end system call time
  //jmp -> 1 
  // -> hardware swtiching 40 -> svc 
  // register 1 , 2 
  // register r0 -> pointer to driver type


  /**
  impl Foo{
	fn bar(baz:usize)
  }

  let my_driver = Foo . . . 
  ~~~

  my_driver.bar(123);
  Foo::bar(my_driver,123);
  mov r0 , &Foo::Bar
  mov r1 , &my_driver
  mov r2 , &"hello"
  svc 0x484
  pop r0 r1 r2
  wrapper( ... )
  	call function 

  	this whole thing is just 
  	re-arranging the registers
  	//assembly -- where does wrapping matter -->
  	//
  	res == wrapper() <<-- what the rapper returns
  	

  **/




}

