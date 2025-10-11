use crate::init::multiboot2::{tags::CMDLine as CMDLine};

/*
pub struct CMDLine<'a> {
	pub typ: u32,
	pub size: u32,
	pub string: &'a str
}

*/



pub unsafe fn cmdline_tag(cmdline_tag: *mut CMDLine) {
	debugln!("CmdLine Type");
	debugn!((*cmdline_tag).typ);
	debugln!("CmdLine Size");
	debugn!((*cmdline_tag).size);
	debugln!("CmdLine String");
	

	
}