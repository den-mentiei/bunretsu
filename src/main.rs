#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
	loop {
		asm::nop();
	}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {
		atomic::compiler_fence(Ordering::SeqCst);
	}
}
