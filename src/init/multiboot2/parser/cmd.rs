use crate::init::multiboot2::{tags::CMDLineTag as CMDLineTag};
use core::ffi::c_str;
use core::ffi::c_str::CStr;
use core::ffi::c_char;
use crate::debug::dump_debug_log_to_file;
/* 
    let raw_bytes = core::slice::from_raw_parts(str_ptr as *const u8, str_len);

	pub struct CMDLineTag {
	pub typ: u32,
	pub size: u32,
	pub string: *const i8
}

    let cmdline = core::str::from_utf8_unchecked(raw_bytes);


	let test1 = CStr::from_ptr((*cmdline_tag).string);
    let test2 = test1.to_str().unwrap();

*/

//create a slice and use CStr::from_bytes_with_nul
//print panic info
#[inline(never)]
#[no_mangle]
pub unsafe fn cmdline_tag(cmdline_tag: *mut CMDLineTag) {

	let test1var = (*cmdline_tag).string;
	dump_debug_log_to_file();
	

}