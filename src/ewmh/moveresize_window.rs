use xcb::x;

use super::{
    ffi, ClientSourceType, EwmhConnection, EwmhRequest, MoveresizeWindowOptFlags, RawEwmhRequest,
};

pub struct RequestMoveresizeWindow {
    pub screen_nbr: i32,
    pub moveresize_window: x::Window,
    pub gravity: x::Gravity,
    pub source_indication: ClientSourceType,
    pub flags: MoveresizeWindowOptFlags,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

unsafe impl RawEwmhRequest for RequestMoveresizeWindow {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_moveresize_window(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.moveresize_window),
                self.gravity as u32,
                self.source_indication as u32,
                self.flags.bits(),
                self.x,
                self.y,
                self.width,
                self.height,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestMoveresizeWindow {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}
