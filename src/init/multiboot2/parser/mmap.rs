use crate::mem::memorymap::{MEMORYMAP as MEMORYMAP};
use crate::init::multiboot2::{tags::MemoryMapTag as MMapTag, tags::MemoryMapEntry as MapEntry};


pub unsafe fn memory_map_tag(mmap_tag: *mut MMapTag) {
	let mut entries = &mut (*mmap_tag).entries as *mut MapEntry;

	let end = (mmap_tag as *mut u8).add((*mmap_tag).size as usize) as *mut MapEntry;
	let mut copied_map = MEMORYMAP.lock();
	let mut entrycount: usize = 0;
	let entrymax = 9;

	while entries < end && entrycount < entrymax{

		copied_map[entrycount].base_addr = (*entries).base_addr;
		copied_map[entrycount].length = (*entries).length;
		copied_map[entrycount].typ = (*entries).typ;
		copied_map[entrycount].reserved = (*entries).reserved;

		entries = ((entries as *mut u8).add((*mmap_tag).entry_size as usize)) as *mut MapEntry;
		entrycount += 1;

	}


}