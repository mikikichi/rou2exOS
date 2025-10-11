use crate::init::multiboot2::tags::{tags::CmdLine as CmdLine};
use spin::Mutex;

/*
pub struct CMDLine<'a> {
	pub typ: u32,
	pub size: u32,
	pub string: &'a str
}

*/


pub static BootCommandLine:CMDLine = Mutex::new(CmdLine{typ: 0, size: 0, string: 0});