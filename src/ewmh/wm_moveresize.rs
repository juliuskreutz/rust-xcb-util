use xcb::x;

use super::{
    ffi, ClientSourceType, EwmhConnection, EwmhRequest, MoveresizeDirection, RawEwmhRequest,
};

pub struct RequestWmMoveresize {
    pub screen_nbr: i32,
    pub moveresize_window: x::Window,
    pub x_root: u32,
    pub y_root: u32,
    pub direction: MoveresizeDirection,
    pub button: x::ButtonIndex,
    pub source_indication: ClientSourceType,
}

unsafe impl RawEwmhRequest for RequestWmMoveresize {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_wm_moveresize(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.moveresize_window),
                self.x_root,
                self.y_root,
                self.direction as u32,
                self.button as u32,
                self.source_indication as u32,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestWmMoveresize {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}
