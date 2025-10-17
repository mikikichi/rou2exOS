
use crate::debug::dump_debug_log_to_file;
use crate::init::multiboot2::parser::{main_parser as parser};
use crate::init::{cpu, idt, heap,pit, fs, color, ascii};
use crate::video::{vga};
use crate::video::sysprint::{Result};


pub fn init(m2_ptr: *mut usize, m2_magic: u32) {
	vga::init_writer();
	clear_screen!();
	//result!("Kernel Loaded", Result::Passed);
	
	unsafe {
	//result!("Reading Multiboot2 Tags", parser::parse_multiboot2_info(m2_ptr, m2_magic));
	print!("Test");
	parser::parse_multiboot2_info(m2_ptr, m2_magic);
	}
	/* 
	result!("Reloading IDT and ISRs", idt::idt_isrs_init());
	
	result!("Initializing heap allocation", heap::pmm_heap_init());

	//result!("Initializing video", );

	result!("Starting PIC timer", pit::pic_pit_init());

	//result!("Checking floppy drive", fs::floppy_check_init());
	color::color_demo();
	ascii::ascii_art(); */
}
