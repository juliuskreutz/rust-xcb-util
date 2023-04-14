use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
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
}

impl xcb::Reply for GetNumberOfDesktopsReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetNumberOfDesktopsReply {
    pub fn number(&self) -> u32 {
        unsafe {
            let mut number = 0;

            ffi::xcb_ewmh_get_number_of_desktops_from_reply(
                &mut number,
                self.raw as *mut ffi::xcb_get_property_reply_t,
            );

            number
        }
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

unsafe impl xcb::CookieWithReplyChecked for GetNumberOfDesktopsCookie {
    type Reply = GetNumberOfDesktopsReply;
}

impl xcb::Cookie for GetNumberOfDesktopsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetNumberOfDesktopsCookieUnchecked {
    type Reply = GetNumberOfDesktopsReply;
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
