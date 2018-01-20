#![no_std]
#![feature(lang_items)]

#![feature(alloc)]
#![feature(fnbox)]

#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[inline(never)]
extern fn panic_fmt(msg: core::fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
	use core::fmt;
	use core::fmt::Write;
	use alloc::string::*;

	debug_print("Panicked!");
	
	{
		let mut s = String::new();
		s.write_fmt(msg);
		debug_print(&s);
	}
    
	exit_test(98);
	loop {}
}


#[macro_use]
extern crate alloc;

extern crate freertos_rs;

#[macro_use]
extern crate lazy_static;

extern {
	fn testbed_println(line: *const u8, line_len: u16);
	fn testbed_start_kernel();
	fn testbed_return(return_code: i8);
	fn testbed_allocated_memory() -> u32;
	fn testbed_init_timer4_50ms_isr();
}

pub fn debug_print(s: &str) {
	let s = s.as_bytes();	
	unsafe {
		testbed_println(s.as_ptr(), s.len() as u16);
	}
}

pub fn start_kernel() {
	unsafe {
		testbed_start_kernel();
	}
}

pub fn exit_test(return_code: i8) {
	unsafe {
		testbed_return(return_code);
	}
}

pub fn heap_allocated_memory() -> u32 {
	unsafe {
		testbed_allocated_memory()
	}
}

pub fn start_timer4_50ms() {
	unsafe {
		testbed_init_timer4_50ms_isr();
	}
}


pub mod freertos_alloc;
mod prelude;
mod utils;

pub mod test_basics;
pub mod test_delay;
pub mod test_mutex;
pub mod test_mem_leaks1;
pub mod test_isr_timer4_queue;
pub mod test_isr_timer4_notify;
pub mod test_sample1;
pub mod test_stats;
pub mod test_processor;
pub mod test_timers;
pub mod test_box_queue1;
pub mod test_box_queue2;



