use std::{mem, ptr, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetWmIconName<'a> {
    pub window: x::Window,
    pub strings: &'a [&'a str],
}

unsafe impl<'a> RawEwmhRequest for SetWmIconName<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            let mut strings = self.strings.join("\0");
            strings.push('\0');

            let strings = strings.as_bytes();

            if checked {
                ffi::xcb_ewmh_set_wm_icon_name_checked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    strings.len() as u32,
                    strings.as_ptr() as *const i8,
                )
            } else {
                ffi::xcb_ewmh_set_wm_icon_name(
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

impl<'a> EwmhRequest for SetWmIconName<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetWmIconName<'a> {}

//TODO: Expose inner reply
pub struct GetWmIconNameReply {
    raw: *const u8,
    strings: Vec<String>,
}

impl EwmhReply for GetWmIconNameReply {
    unsafe fn from_raw(raw: *const u8, ewmh: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut names = mem::zeroed();

        ffi::xcb_ewmh_get_wm_icon_name_from_reply(
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

impl GetWmIconNameReply {
    pub fn strings(&self) -> &[String] {
        &self.strings
    }
}

//TODO: Expose inner cookie
pub struct GetWmIconNameCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetWmIconNameCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetWmIconNameCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetWmIconNameCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetWmIconNameCookie {
    type Reply = GetWmIconNameReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut names = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_wm_icon_name_reply(ewmh.ewmh.get(), cookie, &mut names, &mut e);

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

impl xcb::Cookie for GetWmIconNameCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetWmIconNameCookieUnchecked {
    type Reply = GetWmIconNameReply;

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

            let raw =
                &ffi::xcb_ewmh_get_wm_icon_name_reply(ewmh.ewmh.get(), cookie, &mut names, &mut e);

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

pub struct GetWmIconName {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetWmIconName {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_wm_icon_name(ewmh.ewmh.get(), xcb::Xid::resource_id(&self.window))
            } else {
                ffi::xcb_ewmh_get_wm_icon_name_unchecked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetWmIconName {
    type Cookie = GetWmIconNameCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetWmIconName {
    type Reply = GetWmIconNameReply;
    type Cookie = GetWmIconNameCookie;
    type CookieUnchecked = GetWmIconNameCookieUnchecked;
}
