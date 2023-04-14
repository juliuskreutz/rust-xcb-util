use std::{mem, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetDesktopNames<'a> {
    pub screen_nbr: i32,
    pub strings: &'a [String],
}

unsafe impl<'a> RawEwmhRequest for SetDesktopNames<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            let mut strings = self.strings.join("\0");
            strings.push('\0');

            let strings = strings.as_bytes();

            if checked {
                ffi::xcb_ewmh_set_desktop_names_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    strings.len() as u32,
                    strings.as_ptr() as *const i8,
                )
            } else {
                ffi::xcb_ewmh_set_desktop_names(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    strings.len() as u32,
                    strings.as_ptr() as *const i8,
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetDesktopNames<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetDesktopNames<'a> {}

//TODO: Expose inner reply
pub struct GetDesktopNamesReply {
    raw: *const u8,
}

impl xcb::Reply for GetDesktopNamesReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetDesktopNamesReply {
    pub fn strings(&self, ewmh: &EwmhConnection) -> Vec<String> {
        unsafe {
            let mut names = mem::zeroed();

            ffi::xcb_ewmh_get_desktop_names_from_reply(
                ewmh.ewmh.get(),
                &mut names,
                self.raw as *mut ffi::xcb_get_property_reply_t,
            );

            let strings = std::str::from_utf8_unchecked(slice::from_raw_parts(
                names.strings as *mut u8,
                names.strings_len as usize - 1,
            ))
            .split('\0')
            .map(str::to_string)
            .collect::<Vec<_>>();

            ffi::xcb_ewmh_get_utf8_strings_reply_wipe(&mut names);

            strings
        }
    }
}

//TODO: Expose inner cookie
pub struct GetDesktopNamesCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetDesktopNamesCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetDesktopNamesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetDesktopNamesCookie {}

unsafe impl xcb::CookieWithReplyChecked for GetDesktopNamesCookie {
    type Reply = GetDesktopNamesReply;
}

impl xcb::Cookie for GetDesktopNamesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetDesktopNamesCookieUnchecked {
    type Reply = GetDesktopNamesReply;
}

pub struct GetDesktopNames {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetDesktopNames {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_desktop_names(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_desktop_names_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetDesktopNames {
    type Cookie = GetDesktopNamesCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetDesktopNames {
    type Reply = GetDesktopNamesReply;
    type Cookie = GetDesktopNamesCookie;
    type CookieUnchecked = GetDesktopNamesCookieUnchecked;
}
