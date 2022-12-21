#![no_main]
#![no_std]

use panic_halt as _;

use stm32f1xx_hal as hal;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
#[entry]
fn main() -> ! {
    loop {
        wfi();
    }
}
