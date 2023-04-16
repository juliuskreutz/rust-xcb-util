use xcb::x;

use super::{ffi, EwmhConnection, EwmhRequest, RawEwmhRequest};

pub struct RequestRestackWindow {
    pub screen_nbr: i32,
    pub window_to_restack: x::Window,
    pub sibling_window: x::Window,
    pub detail: x::StackMode,
}

unsafe impl RawEwmhRequest for RequestRestackWindow {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_restack_window(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.window_to_restack),
                xcb::Xid::resource_id(&self.sibling_window),
                self.detail as u32,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestRestackWindow {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}
