use core::cell::Cell;
use hil::{Driver,Callback};
use hil::gpio::{GPIOPin,InputMode,InterruptMode,Client};


pub struct TypedGPIO<'a ,G: GPIOPin + 'a>  {
	pins: &'a [&'a G],
	callback: Cell<Option<Callback>>
}

impl<'a, G:GPIOPin> TypedGPIO<'a,G> {
    // add code here
    pub fn new(pins: &'a [&'a G]) -> GPIO<'a, G> {
        GPIO {
            pins: pins,
            callback: Cell::new(None),
        }
        //collasped a pattern match 
        fn configure_input_pin(&self, pin_num: usize, config: InputMode) -> isize {
        let pins = self.pins.as_ref();
        pins[pin_num].enable_input(config);
    	}

    fn configure_interrupt(&self, pin_num: usize, config: InterruptMode) -> isize {
        let pins = self.pins.as_ref();
        pins[pin_num].enable_interrupt(pin_num, config);

        }
    }
    //ideally we would want the application to make the callback struct right?
    //don't need a subscribe number
    fn subscribe(&self, callback: Callback) -> isize {
       			
                self.callback.set(Some(callback));
                0
         
        }

    //these are more specialized functions to replicate commands
    fn enable_output(&self,index:usize) -> isize {
    	let pins =  self.pins.as_ref();
    	if data >= pins.len() {
                    -1
                }else {
                    pins[data].enable_output();
                     0
                }
    }

    fn set_pin(&self,index:usize) -> isize {   //is the idea that these get called directly from the app code
    	let pins = self.pins.as_ref();
    	if data >= pins.len() {
                    -1
                  
                } else {
                    pins[data].set();
                    0
                }
    }

    fn clear_pin(&self,index:usize) -> isize {
    	let pins = self.pins.as_ref(); 
    	if data >= pins.len() {  //is there away to check this at compile time?
    		-1
    	}else {
    		pins[data].clear();
    		0
    	}
    }

    fn toggle_pin(&self,index:usize) -> isize {

    	let pins = self.pins.as_ref();
    	if data >= pins.len() {
    		-1
    	}else {
    		pins[data].toggle();
    		0
    	}


    }

    fn enable_input(&self,index:usize,pin_config:InputMode) -> isize {

                if index >= pins.len() {
                    -1
                } else {
                   let err_code = self.configure_input_pin(index, pin_config);
                   err_code
                }
    }

    fn read_input(index: usize) -> isize {
         if data >= pins.len() {
                    -1
                } else {
                    let pin_state = pins[data].read()
                    pin_state as isize
                }
    }

    fn enable_interrupt(index: usize,pin_config:usize,irq_config:usize) -> isize {
        
                //let pin_config = (data >>  8) & 0xFF;
               // let irq_config = (data >> 16) & 0xFF;
                if index >= pins.len() {
                    -1
                } else {
                    let mut err_code = self.configure_input_pin(index, pin_config);
                    if err_code == 0 {
                        err_code = self.configure_interrupt(index, irq_config);
                    }
                    err_code
                }
    }

    fn disable_interrupts(index: usize) -> isize {
        if index >= pins.len() {    
                    -1
                } else {
                    pins[index].disable_interrupt();
                    pins[index].disable();
                    0
                }
    }

    fn disable(index:usize) -> isize {
        if data >= pins.len() {
                    -1
                } else {
                    pins[data].disable();
                    0
                }
    }
    //there is also no possibility of returning negative one




}


    impl<'a, G: GPIOPin> Client for GPIO<'a, G> {
    fn fired(&self, pin_num: usize) {
        // read the value of the pin
        let pins = self.pins.as_ref();
        let pin_state = pins[pin_num].read();

        // schedule callback with the pin number and value
        if self.callback.get().is_some() {
            self.callback.get().unwrap().schedule(pin_num, pin_state as usize, 0);
        }
    }




}



