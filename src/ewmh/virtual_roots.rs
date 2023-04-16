use std::{mem, ptr, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
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
    windows: Vec<x::Window>,
}

impl EwmhReply for GetVirtualRootsReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut clients = mem::zeroed();

        ffi::xcb_ewmh_get_virtual_roots_from_reply(
            &mut clients,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let windows = slice::from_raw_parts(
            clients.windows as *mut x::Window,
            clients.windows_len as usize,
        )
        .to_vec();

        ffi::xcb_ewmh_get_windows_reply_wipe(&mut clients);

        Self { raw, windows }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetVirtualRootsReply {
    pub fn windows(&self) -> &[x::Window] {
        &self.windows
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

unsafe impl EwmhCookieWithReplyChecked for GetVirtualRootsCookie {
    type Reply = GetVirtualRootsReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut clients = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_virtual_roots_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut clients,
                &mut e,
            );

            let windows = slice::from_raw_parts(
                clients.windows as *mut x::Window,
                clients.windows_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_windows_reply_wipe(&mut clients);

            Ok(Self::Reply { raw, windows })
        }
    }
}

impl xcb::Cookie for GetVirtualRootsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetVirtualRootsCookieUnchecked {
    type Reply = GetVirtualRootsReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut clients = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_virtual_roots_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut clients,
                &mut e,
            );

            let windows = slice::from_raw_parts(
                clients.windows as *mut x::Window,
                clients.windows_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_windows_reply_wipe(&mut clients);

            Ok(Some(Self::Reply { raw, windows }))
        }
    }
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