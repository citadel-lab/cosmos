#![no_std]
#![no_main]

use core::panic::PanicInfo;

use limine::{
    RequestsEndMarker,
    RequestsStartMarker,
};
use limine::request::FramebufferRequest;

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests_end")]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    loop {}
}