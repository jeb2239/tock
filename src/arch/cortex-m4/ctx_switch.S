.cpu cortex-m4
.syntax unified
.thumb
.text

/* Exported functions */
.global SVC_Handler
.global switch_to_user
.global __wait
.global __allow
.global __subscribe
.global __command
.global __safe_call
.global __fast_print_async
.global to_kernel
.thumb_func
SVC_Handler:
  cmp lr, #0xfffffff9
  bne to_kernel
  movw lr, #0xfffd
  movt lr, #0xffff
  bx lr
to_kernel:
  mrs r0, PSP /* PSP into r0 */
  str r0, [sp, #0] /* PSP into Master stack r0 */
  movw LR, #0xFFF9
  movt LR, #0xFFFF
  bx lr

.thumb_func
/* r0 is top of user stack, r1 is heap base */
switch_to_user:
    /* Load bottom of stack into Process Stack Pointer */
    msr psp, r0

    push {r4-r11}
    mov r9, r1
    svc 0xff
    pop {r4-r11}
    bx lr

.cpu cortex-m4
.syntax unified
.thumb
.text

.section .syscalls


.thumb_func
__wait:
    push {r4-r11,lr}
    svc 0
    pop {r4-r11,lr}
    bx lr

.thumb_func
__allow:
    push {r4-r11}
    svc 3
    pop {r4-r11}
    bx lr

.thumb_func
__subscribe:
    push {r4-r11}
    svc 1
    pop {r4-r11}
    bx lr

.thumb_func
__command:
    push {r4-r11}
    svc 2
    pop {r4-r11}
    bx lr

.thumb_func
__safe_call:
  push {r4-r11} //watch what goes on over here
  svc 5
  pop {r4-r11}
  bx lr
  
.thumb_func
__fast_print_async:
  push {r4 - r11} //watch
  svc 6
  pop {r4-r11}
  bx lr



  


  .syntax unified
  .cpu cortex-m4
  .fpu softvfp
  .eabi_attribute 20, 1
  .eabi_attribute 21, 1
  .eabi_attribute 23, 3
  .eabi_attribute 24, 1
  .eabi_attribute 25, 1
  .eabi_attribute 26, 1
  .eabi_attribute 30, 6
  .eabi_attribute 34, 1
  .eabi_attribute 18, 4
  .thumb
  .syntax unified
  .file "count_cycles.c"
  .global count
  .bss
  .align  2
  .type count, %object
  .size count, 4
count:
  .space  4
  .global DWT_CONTROL
  .data
  .align  2
  .type DWT_CONTROL, %object
  .size DWT_CONTROL, 4
DWT_CONTROL:
  .word -536866816
  .global DWT_CYCCNT
  .align  2
  .type DWT_CYCCNT, %object
  .size DWT_CYCCNT, 4
DWT_CYCCNT:
  .word -536866812
  .global DEMCR
  .align  2
  .type DEMCR, %object
  .size DEMCR, 4
DEMCR:
  .word -536809988
  .text
  .align  2
  .global __start_count
  .thumb
  .thumb_func
  .type __start_count, %function
__start_count:
  @ args = 0, pretend = 0, frame = 0
  @ frame_needed = 1, uses_anonymous_args = 0
  @ link register save eliminated.
  push  {r7}
  add r7, sp, #0
  ldr r3, .L2
  ldr r3, [r3]
  ldr r2, .L2
  ldr r2, [r2]
  ldr r2, [r2]
  orr r2, r2, #16777216
  str r2, [r3]
  ldr r3, .L2+4
  ldr r3, [r3]
  movs  r2, #0
  str r2, [r3]
  ldr r3, .L2+8
  ldr r3, [r3]
  ldr r2, .L2+8
  ldr r2, [r2]
  ldr r2, [r2]
  orr r2, r2, #1
  str r2, [r3]
  nop
  mov r0, r3
  mov sp, r7
  @ sp needed
  pop {r7}
  bx  lr
.L3:
  .align  2
.L2:
  .word DEMCR
  .word DWT_CYCCNT
  .word DWT_CONTROL
  .size __start_count, .-__start_count
  .align  2
  .global __end_count
  .thumb
  .thumb_func
  .type __end_count, %function
__end_count:
  @ args = 0, pretend = 0, frame = 0
  @ frame_needed = 1, uses_anonymous_args = 0
  @ link register save eliminated.
  push  {r7}
  add r7, sp, #0
  ldr r3, .L6
  ldr r3, [r3]
  ldr r3, [r3]
  ldr r2, .L6+4
  str r3, [r2]
  ldr r3, .L6+4
  ldr r3, [r3]
  mov r0, r3
  mov sp, r7
  @ sp needed
  pop {r7}
  bx  lr
.L7:
  .align  2
.L6:
  .word DWT_CYCCNT
  .word count
  .size __end_count, .-__end_count
  .ident  "GCC: (GNU Tools for ARM Embedded Processors) 5.2.1 20151202 (release) [ARM/embedded-5-branch revision 231848]"
