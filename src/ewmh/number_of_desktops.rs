use std::ptr;

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetNumberOfDesktops {
    pub screen_nbr: i32,
    pub number_of_desktops: u32,
}

unsafe impl RawEwmhRequest for SetNumberOfDesktops {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_number_of_desktops_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.number_of_desktops,
                )
            } else {
                ffi::xcb_ewmh_set_number_of_desktops(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.number_of_desktops,
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetNumberOfDesktops {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetNumberOfDesktops {}

pub struct RequestChangeNumberOfDesktops {
    pub screen_nbr: i32,
    pub number_of_desktops: u32,
}

unsafe impl RawEwmhRequest for RequestChangeNumberOfDesktops {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_number_of_desktops(
                ewmh.ewmh.get(),
                self.screen_nbr,
                self.number_of_desktops,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeNumberOfDesktops {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetNumberOfDesktopsReply {
    raw: *const u8,
    number: u32,
}

impl EwmhReply for GetNumberOfDesktopsReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut number = 0;

        ffi::xcb_ewmh_get_number_of_desktops_from_reply(
            &mut number,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        Self { raw, number }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetNumberOfDesktopsReply {
    pub fn number(&self) -> u32 {
        self.number
    }
}

//TODO: Expose inner cookie
pub struct GetNumberOfDesktopsCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetNumberOfDesktopsCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetNumberOfDesktopsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetNumberOfDesktopsCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetNumberOfDesktopsCookie {
    type Reply = GetNumberOfDesktopsReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut number = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_number_of_desktops_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut number,
                &mut e,
            );

            Ok(Self::Reply { raw, number })
        }
    }
}

impl xcb::Cookie for GetNumberOfDesktopsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetNumberOfDesktopsCookieUnchecked {
    type Reply = GetNumberOfDesktopsReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut number = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_number_of_desktops_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut number,
                &mut e,
            );

            Ok(Some(Self::Reply { raw, number }))
        }
    }
}

pub struct GetNumberOfDesktops {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetNumberOfDesktops {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_number_of_desktops(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_number_of_desktops_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetNumberOfDesktops {
    type Cookie = GetNumberOfDesktopsCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetNumberOfDesktops {
    type Reply = GetNumberOfDesktopsReply;
    type Cookie = GetNumberOfDesktopsCookie;
    type CookieUnchecked = GetNumberOfDesktopsCookieUnchecked;
}
