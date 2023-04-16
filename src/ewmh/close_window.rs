use xcb::x;

use super::{ffi, ClientSourceType, EwmhConnection, EwmhRequest, RawEwmhRequest};

pub struct RequestCloseWindow {
    pub screen_nbr: i32,
    pub window_to_close: x::Window,
    pub timestamp: x::Timestamp,
    pub source_indication: ClientSourceType,
}

unsafe impl RawEwmhRequest for RequestCloseWindow {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_close_window(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.window_to_close),
                self.timestamp,
                self.source_indication as u32,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestCloseWindow {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}
