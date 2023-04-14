use std::{mem, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetVirtualRoots<'a> {
    pub screen_nbr: i32,
    pub windows: &'a [x::Window],
}

unsafe impl<'a> RawEwmhRequest for SetVirtualRoots<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_virtual_roots_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.windows.len() as u32,
                    self.windows.as_ptr() as *mut ffi::xcb_window_t,
                )
            } else {
                ffi::xcb_ewmh_set_virtual_roots(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.windows.len() as u32,
                    self.windows.as_ptr() as *mut ffi::xcb_window_t,
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetVirtualRoots<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetVirtualRoots<'a> {}

// TODO: Expose inner reply
pub struct GetVirtualRootsReply {
    raw: *const u8,
}

impl xcb::Reply for GetVirtualRootsReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetVirtualRootsReply {
    pub fn windows(&self) -> Vec<x::Window> {
        unsafe {
            let mut clients = mem::zeroed();

            ffi::xcb_ewmh_get_virtual_roots_from_reply(
                &mut clients,
                self.raw as *mut ffi::xcb_get_property_reply_t,
            );

            let windows = slice::from_raw_parts(
                clients.windows as *mut x::Window,
                clients.windows_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_windows_reply_wipe(&mut clients);

            windows
        }
    }
}

//TODO: Expose inner cookie
pub struct GetVirtualRootsCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetVirtualRootsCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetVirtualRootsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetVirtualRootsCookie {}

unsafe impl xcb::CookieWithReplyChecked for GetVirtualRootsCookie {
    type Reply = GetVirtualRootsReply;
}

impl xcb::Cookie for GetVirtualRootsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetVirtualRootsCookieUnchecked {
    type Reply = GetVirtualRootsReply;
}

pub struct GetVirtualRoots {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetVirtualRoots {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_virtual_roots(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_virtual_roots_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetVirtualRoots {
    type Cookie = GetVirtualRootsCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetVirtualRoots {
    type Reply = GetVirtualRootsReply;
    type Cookie = GetVirtualRootsCookie;
    type CookieUnchecked = GetVirtualRootsCookieUnchecked;
}
