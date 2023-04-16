use std::ptr;

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetShowingDesktop {
    pub screen_nbr: i32,
    pub desktop: u32,
}

unsafe impl RawEwmhRequest for SetShowingDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_showing_desktop_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.desktop,
                )
            } else {
                ffi::xcb_ewmh_set_showing_desktop(ewmh.ewmh.get(), self.screen_nbr, self.desktop)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetShowingDesktop {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetShowingDesktop {}

pub struct RequestChangeShowingDesktop {
    pub screen_nbr: i32,
    pub enter: u32,
}

unsafe impl RawEwmhRequest for RequestChangeShowingDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_showing_desktop(
                ewmh.ewmh.get(),
                self.screen_nbr,
                self.enter,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeShowingDesktop {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetShowingDesktopReply {
    raw: *const u8,
    desktop: u32,
}

impl EwmhReply for GetShowingDesktopReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut desktop = 0;

        ffi::xcb_ewmh_get_showing_desktop_from_reply(
            &mut desktop,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        Self { raw, desktop }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetShowingDesktopReply {
    pub fn desktop(&self) -> u32 {
        self.desktop
    }
}

//TODO: Expose inner cookie
pub struct GetShowingDesktopCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetShowingDesktopCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetShowingDesktopCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetShowingDesktopCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetShowingDesktopCookie {
    type Reply = GetShowingDesktopReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut desktop = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_showing_desktop_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut desktop,
                &mut e,
            );

            Ok(Self::Reply { raw, desktop })
        }
    }
}

impl xcb::Cookie for GetShowingDesktopCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetShowingDesktopCookieUnchecked {
    type Reply = GetShowingDesktopReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut desktop = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_showing_desktop_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut desktop,
                &mut e,
            );

            Ok(Some(Self::Reply { raw, desktop }))
        }
    }
}

pub struct GetShowingDesktop {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetShowingDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_showing_desktop(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_showing_desktop_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetShowingDesktop {
    type Cookie = GetShowingDesktopCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetShowingDesktop {
    type Reply = GetShowingDesktopReply;
    type Cookie = GetShowingDesktopCookie;
    type CookieUnchecked = GetShowingDesktopCookieUnchecked;
}
