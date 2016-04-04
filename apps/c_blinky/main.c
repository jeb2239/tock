#include <firestorm.h>
#include <gpio.h>






CB_TYPE timer_cb(int, int, int, void*);

void main(void) {
	volatile uint32_t count = 0;

// addresses of registers
volatile uint32_t *DWT_CONTROL = (uint32_t *)0xE0001000;
volatile uint32_t *DWT_CYCCNT = (uint32_t *)0xE0001004; 
volatile uint32_t *DEMCR = (uint32_t *)0xE000EDFC; 

// enable the use DWT
*DEMCR = *DEMCR | 0x01000000;

// Reset cycle counter
*DWT_CYCCNT = 0; 

// enable cycle counter
*DWT_CONTROL = *DWT_CONTROL | 1 ; 

// some code here
// .....

// number of cycles stored in count variable

gpio_enable_output(LED_0);
//timer_repeating_subscribe(timer_cb, NULL);

count = *DWT_CYCCNT;
   
}

CB_TYPE timer_cb(int arg0, int arg2, int arg3, void* userdata) {
  gpio_toggle(LED_0);
  return 0;
}

