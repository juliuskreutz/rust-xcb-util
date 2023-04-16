use xcb::x;

use super::{ffi, EwmhConnection, EwmhRequest, RawEwmhRequest};

pub struct RequestFrameExtents {
    pub screen_nbr: i32,
    pub client_window: x::Window,
}

unsafe impl RawEwmhRequest for RequestFrameExtents {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_frame_extents(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.client_window),
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestFrameExtents {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}
