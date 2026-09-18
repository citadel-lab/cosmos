#![no_std]
#![no_main]

use core::panic::PanicInfo;

use limine::{
    RequestsEndMarker,
    RequestsStartMarker,
};
use limine::request::FramebufferRequest;

#[used]
#[unsafe(link_section = ".requests")]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let response = FRAMEBUFFER_REQUEST
        .response()
        .expect("Framebuffer request failed");

    let framebuffer = response.framebuffers()
        .first()
        .expect("No framebuffer available");

    let buffer = unsafe { framebuffer.as_slice_mut() };

    for byte in buffer.iter_mut() {
        *byte = 0;
    }

    loop {}
}