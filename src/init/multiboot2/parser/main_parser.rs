
/*use crate::{debug::dump_debug_log_to_file, init::{config::{p1_fb_table, p1_fb_table_2, p2_fb_table, p3_fb_table, p4_table}, font::{draw_text_psf, parse_psf}}, mem, vga::{
    buffer::Color, write::{newline, number, string}
} };*/
//Specific tags
use crate::init::multiboot2::{header,header::M2TagType as TagType, tags::BasicTag as BasicTag, tags::MemoryMapTag as MMapTag, tags::BootModules as BootModules, tags::CMDLine as CMDLineTag};
//Parser specifics
use crate::init::multiboot2::parser::{mmap, module, cmd};


 
pub unsafe fn parse_multiboot2_info(m2_ptr: *mut usize, m2_magic: u32) {
	if m2_magic != header::MULTIBOOT2_BOOTLOADER_MAGIC {
		debugln!("Issue");
		return; //return sysfail here
	};

	let mut m2_tag = ((m2_ptr as *mut u8).add(8) ) as *mut BasicTag;


    while (*m2_tag).typ != TagType::End {

        match (*m2_tag).typ {

            TagType::CmdLine => {
				let cmdline_tag = m2_tag as *mut CMDLineTag;
				cmd::cmdline_tag(cmdline_tag);
            }

            TagType::Module => { 
				let module_tag = m2_tag as *mut BootModules;
				module::module_tag(module_tag);

            }

            TagType::Mmap => {
				let mmap_tag = m2_tag as *mut MMapTag;
				mmap::memory_map_tag(mmap_tag);
            }

            TagType::Framebuffer => {
				debugn!((*m2_tag).typ);
				debugln!("Frame");

            }

            TagType::AcpiOLD => {
				debugn!((*m2_tag).typ);
				debugln!("AcpiOld");

            }

            _ => {
				debugn!((*m2_tag).typ);
				debugln!("Not yet implemented");
            }
		
        }
	//Could be cleaned up
	m2_tag = (((m2_tag as *mut u8).add((*m2_tag).size as usize + 7)) as usize & !(7)) as *mut BasicTag;

	}


 }



//stashed code for now!!!
/* 

pub unsafe fn framebuffer_tag() {
	debugln!("Framebuffer tag: ");
	/* 
    b_tag = &*(ptr as *const FramebufferTag);

    debug!("Framebuffer address: ");
    debugn!(fb_tag.addr as u64);
    debugln!("");

    debug!("(bpp + res): ");
    debugn!(fb_tag.bpp as u64);
    debug!(" + ");
    debugn!(fb_tag.width as u64);
    debug!("x");
    debugn!(fb_tag.height as u64);
    debugln!("");

    debug!("Pitch: ");
    debugn!(fb_tag.pitch);
    debugln!("");


                use core::ptr;
                use x86_64::registers::control::Cr3;

                unsafe {
                    if fb_tag.addr == 0xb8000 {
                        ptr += align_up(tag.size as usize, 8);
                        continue;
                    }

                    rprint!("Mapping framebuffer\n");
                    let virt_base = 0xffff_8000_0000_0000u64 + fb_tag.addr as u64;

                    //crate::mem::pmm::map_framebuffer(fb_tag.addr as u64, 0xffff_8000_0000_0000 + fb_tag.addr as u64);
                    //crate::mem::pmm::map_framebuffer(fb_tag.addr as u64, virt_base);
                    crate::mem::pmm::map_framebuffer(0xfd00_0000, 0xffff_8000_fd00_0000);

                    let fb_ptr = 0xffff_8000_fd00_0000 as *mut u64;

                    *fb_ptr = 0xFFFFFFFF; 

                    draw_rect(fb_ptr, 150, 150, 100, 100, 4096, 0x00ffffff);
                    draw_rect(fb_ptr, 250, 250, 100, 100, 4096, 0x00ff0000);
                    draw_rect(fb_ptr, 350, 350, 100, 100, 4096, 0x0000ff00);
                    draw_rect(fb_ptr, 450, 450, 100, 100, 4096, 0x000000ff);

                    if let Some(font) = parse_psf(super::font::PSF_FONT) {
                        draw_text_psf("[guest@rou2ex:/] > ", &font, 25, 30, 0x0000ff00, fb_ptr, fb_tag.pitch as usize, fb_tag.bpp as usize);
                        draw_text_psf("[guest@rou2ex:/] > ", &font, 25, 50, 0x00ffd700, fb_ptr, fb_tag.pitch as usize, fb_tag.bpp as usize);

                        //draw_char("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 35, 35, fb_ptr, fb_tag.pitch as usize, 0xdeadbeef, FONT_RAW);
                    }

                    //draw_test_char(35, 35, fb_ptr);
                    //draw_text_psf("ABCDEFGHIJKLMNOPQRSTUVWXYZ",&FONT_RAW, 35, 35, 0x00ff00, fb_ptr, fb_tag.pitch, fb_tag.bpp);
                }

                //dump_debug_log_to_file();
	*/
}
*/
//Could be used elsewhere
/* 
fn align_up(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
} */


/*pub unsafe fn draw_rect(ptr: *mut u64, x0: usize, y0: usize, w: usize, h: usize, pitch: usize, color: u32) {
    for y in y0..(y0 + h) {
        for x in x0..(x0 + w) {
            let offset = y * (pitch / 4) + x;

            ptr.add(offset).write_volatile(color as u64);
        }
    }
}

*/
