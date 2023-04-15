use std::ptr;

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetCurrentDesktop {
    pub screen_nbr: i32,
    pub new_current_desktop: x::Window,
}

unsafe impl RawEwmhRequest for SetCurrentDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_current_desktop_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    xcb::Xid::resource_id(&self.new_current_desktop),
                )
            } else {
                ffi::xcb_ewmh_set_current_desktop(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    xcb::Xid::resource_id(&self.new_current_desktop),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetCurrentDesktop {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetCurrentDesktop {}

pub struct RequestChangeCurrentDesktop {
    pub screen_nbr: i32,
    pub new_desktop: x::Window,
    pub timestamp: x::Timestamp,
}

unsafe impl RawEwmhRequest for RequestChangeCurrentDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_current_desktop(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.new_desktop),
                self.timestamp,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeCurrentDesktop {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetCurrentDesktopReply {
    raw: *const u8,
    window: x::Window,
}

impl EwmhReply for GetCurrentDesktopReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut current_desktop = 0;

        ffi::xcb_ewmh_get_current_desktop_from_reply(
            &mut current_desktop,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let window = <x::Window as xcb::XidNew>::new(current_desktop);

        Self { raw, window }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetCurrentDesktopReply {
    pub fn window(&self) -> x::Window {
        self.window
    }
}

//TODO: Expose inner cookie
pub struct GetCurrentDesktopCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetCurrentDesktopCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetCurrentDesktopCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetCurrentDesktopCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetCurrentDesktopCookie {
    type Reply = GetCurrentDesktopReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut window = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_current_desktop_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut window,
                &mut e,
            );

            let window = <x::Window as xcb::XidNew>::new(window);

            Ok(Self::Reply { raw, window })
        }
    }
}

impl xcb::Cookie for GetCurrentDesktopCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetCurrentDesktopCookieUnchecked {
    type Reply = GetCurrentDesktopReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut window = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_current_desktop_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut window,
                &mut e,
            );

            let window = <x::Window as xcb::XidNew>::new(window);

            Ok(Some(Self::Reply { raw, window }))
        }
    }
}

pub struct GetCurrentDesktop {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetCurrentDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_current_desktop(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_current_desktop_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetCurrentDesktop {
    type Cookie = GetCurrentDesktopCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetCurrentDesktop {
    type Reply = GetCurrentDesktopReply;
    type Cookie = GetCurrentDesktopCookie;
    type CookieUnchecked = GetCurrentDesktopCookieUnchecked;
}
