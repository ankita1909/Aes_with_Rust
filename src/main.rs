#![no_std]
#![no_main]
//#![feature(allocator_api)]
//#[global_allocator]
//static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use core::alloc;
use core::mem;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use psoc6_01_pac as psoc;
use cortex_m::peripheral::{DWT, SYST, Peripherals};


//mod aes_rustway;
mod aes_using_1d_array;
//mod aes_rust_2;
//mod command_execution;
//mod dynamic_array_vec;

#[link_section=".cm0boot"]
#[no_mangle]
static CM0_BOOT: [u8; 6224]=*(include_bytes!("psoc6_02_cm0p_sleep.bin"));
/*
pub use cortex_m::volatile_register::{RW, RO};

pub struct SystemTimer {
    p: &'static mut RegisterBlock
}

#[repr(C)]
struct RegisterBlock {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

impl SystemTimer {
    pub fn new() -> SystemTimer {
        SystemTimer {
            p: unsafe { &mut *(0x1600 as *mut RegisterBlock) }
        }
    }

    pub fn get_time(&self) -> u32 {
        self.p.cvr.read()
    }

    pub fn set_reload(&mut self, reload_value: u32) {
        unsafe { self.p.rvr.write(reload_value) }
    }
}

pub fn example_usage() {
    let mut st = SystemTimer::new();
    st.set_reload(0x40000147);
    st.get_time();
}
*/

#[entry]
fn main() -> ! {
hprintln!("Hello World!");
let p=psoc::Peripherals::take().unwrap();
/*
let ctrl=&p.CPUSS.systick_ctl;
let a = ctrl.write(|w| unsafe{w.noref().set_bit()});
*/
//let ctrl = &p.TCPWM0.cnt;

{
let mut peripherals = Peripherals::take().unwrap();
//peripherals.DWT.enable_cycle_counter();

//*using the systick counter in syst.rs* 
peripherals.SYST.set_reload(0x00FFFFFF);
peripherals.SYST.clear_current();
peripherals.SYST.enable_counter();
peripherals.SYST.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);

}
//let cyccnt = DWT::get_cycle_count();
while SYST::get_current() != 0
{
    
}
let sys_cnt:u32 = SYST::get_current();

//aes_rustway::test_aes();
//aes_rust_2::test_aes();
aes_using_1d_array::test_aes();

//let cyccnt_1 = DWT::get_cycle_count();

let sys_cnt_after:u32 = SYST::get_current();

hprintln!("tick before {}",sys_cnt).unwrap(); 
hprintln!("tick after {}",sys_cnt_after).unwrap(); 
hprintln!("After call using Systick {}",sys_cnt - sys_cnt_after).unwrap(); 
//hprintln!("After call using Systick {}",sys_cnt_after - sys_cnt).unwrap(); 


loop
   {

   }
}


/*

#d
volatile let test = 0x44
test = check_pass()

if (test == 0x55)
{

}

else if(test = 0x66)

{

}

else
{
   sec_res()
}
*/