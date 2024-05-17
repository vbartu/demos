#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const MULTIBOOT2_MAGIC: u32          = 0xE85250D6;
const MULTIBOOT2_ARCH_I386: u32      = 0;
const MULTIBOOT2_TAG_END_TYPE: u16   = 0;
const MULTIBOOT2_TAG_END_FLAGS: u16  = 0;
const MULTIBOOT2_TAG_END_LENGTH: u32 = 8;

#[repr(C, packed)]
pub struct Multiboot2Header {
    magic: u32,
    arch: u32,
    header_length: u32,
    checksum: i32,
    // Include the mandatory End Tag
    tag_type: u16,
    tag_flags: u16,
    tag_length: u32,
}

impl Multiboot2Header {
    const fn new() -> Self {
        let mut new = Self {
            magic: MULTIBOOT2_MAGIC,
            arch: MULTIBOOT2_ARCH_I386,
            header_length: core::mem::size_of::<Self>() as u32,
            checksum: 0,
            tag_type: MULTIBOOT2_TAG_END_TYPE,
            tag_flags: MULTIBOOT2_TAG_END_FLAGS,
            tag_length: MULTIBOOT2_TAG_END_LENGTH,
        };
        new.checksum = -((new.magic + new.arch + new.header_length) as i32);
        new
    }
}

#[no_mangle]
#[link_section = ".multiboot"]
pub static MULTIBOOT_HEADER: Multiboot2Header = Multiboot2Header::new();


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_ptr: *mut u32 = 0xb8000 as *mut u32;
    unsafe {
        *vga_ptr = 0x2f4b2f4f;
    }
    loop {}
}
