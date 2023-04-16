use std::{mem, ptr, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetWmVisibleIconName<'a> {
    pub window: x::Window,
    pub strings: &'a [&'a str],
}

unsafe impl<'a> RawEwmhRequest for SetWmVisibleIconName<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            let mut strings = self.strings.join("\0");
            strings.push('\0');

            let strings = strings.as_bytes();

            if checked {
                ffi::xcb_ewmh_set_wm_visible_icon_name_checked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    strings.len() as u32,
                    strings.as_ptr() as *const i8,
                )
            } else {
                ffi::xcb_ewmh_set_wm_visible_icon_name(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    strings.len() as u32,
                    strings.as_ptr() as *const i8,
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetWmVisibleIconName<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetWmVisibleIconName<'a> {}

//TODO: Expose inner reply
pub struct GetWmVisibleIconNameReply {
    raw: *const u8,
    strings: Vec<String>,
}

impl EwmhReply for GetWmVisibleIconNameReply {
    unsafe fn from_raw(raw: *const u8, ewmh: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut names = mem::zeroed();

        ffi::xcb_ewmh_get_wm_visible_icon_name_from_reply(
            ewmh,
            &mut names,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let strings = std::str::from_utf8_unchecked(slice::from_raw_parts(
            names.strings as *mut u8,
            names.strings_len as usize - 1,
        ))
        .split('\0')
        .map(str::to_string)
        .collect::<Vec<_>>();

        ffi::xcb_ewmh_get_utf8_strings_reply_wipe(&mut names);

        Self { raw, strings }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetWmVisibleIconNameReply {
    pub fn strings(&self) -> &[String] {
        &self.strings
    }
}

//TODO: Expose inner cookie
pub struct GetWmVisibleIconNameCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetWmVisibleIconNameCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetWmVisibleIconNameCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetWmVisibleIconNameCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetWmVisibleIconNameCookie {
    type Reply = GetWmVisibleIconNameReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut names = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_wm_visible_icon_name_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut names,
                &mut e,
            );

            let strings = std::str::from_utf8_unchecked(slice::from_raw_parts(
                names.strings as *mut u8,
                names.strings_len as usize - 1,
            ))
            .split('\0')
            .map(str::to_string)
            .collect::<Vec<_>>();

            ffi::xcb_ewmh_get_utf8_strings_reply_wipe(&mut names);

            Ok(Self::Reply { raw, strings })
        }
    }
}

impl xcb::Cookie for GetWmVisibleIconNameCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetWmVisibleIconNameCookieUnchecked {
    type Reply = GetWmVisibleIconNameReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut names = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_wm_visible_icon_name_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut names,
                &mut e,
            );

            let strings = std::str::from_utf8_unchecked(slice::from_raw_parts(
                names.strings as *mut u8,
                names.strings_len as usize - 1,
            ))
            .split('\0')
            .map(str::to_string)
            .collect::<Vec<_>>();

            ffi::xcb_ewmh_get_utf8_strings_reply_wipe(&mut names);

            Ok(Some(Self::Reply { raw, strings }))
        }
    }
}

pub struct GetWmVisibleIconName {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetWmVisibleIconName {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_wm_visible_icon_name(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            } else {
                ffi::xcb_ewmh_get_wm_visible_icon_name_unchecked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetWmVisibleIconName {
    type Cookie = GetWmVisibleIconNameCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetWmVisibleIconName {
    type Reply = GetWmVisibleIconNameReply;
    type Cookie = GetWmVisibleIconNameCookie;
    type CookieUnchecked = GetWmVisibleIconNameCookieUnchecked;
}
