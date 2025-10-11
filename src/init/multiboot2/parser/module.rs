use crate::init::multiboot2::{tags::BootModules as BootModules};
/* 
    let str_ptr = ptr + 16;
    let str_len = tag.size as usize - 16;
    let raw_bytes = core::slice::from_raw_parts(str_ptr as *const u8, str_len);

    let cmdline = core::str::from_utf8_unchecked(raw_bytes);

*/


pub unsafe fn module_tag(module_tag: *mut BootModules) {
	debugln!("Module end");
	debugn!((*module_tag).mod_end);
	debugln!("Module start");
	debugn!((*module_tag).mod_start);
}
