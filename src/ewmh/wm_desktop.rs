use std::ptr;

use xcb::x;

use super::{
    ffi, ClientSourceType, EwmhConnection, EwmhCookieWithReplyChecked,
    EwmhCookieWithReplyUnchecked, EwmhReply, EwmhRequest, EwmhRequestWithReply,
    EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetWmDesktop {
    pub window: x::Window,
    pub desktop: u32,
}

unsafe impl RawEwmhRequest for SetWmDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_wm_desktop_checked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    self.desktop,
                )
            } else {
                ffi::xcb_ewmh_set_wm_desktop(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    self.desktop,
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetWmDesktop {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetWmDesktop {}

pub struct RequestChangeWmDesktop {
    pub screen_nbr: i32,
    pub client_window: x::Window,
    pub new_desktop: u32,
    pub source_indication: ClientSourceType,
}

unsafe impl RawEwmhRequest for RequestChangeWmDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_wm_desktop(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.client_window),
                self.new_desktop,
                self.source_indication as u32,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeWmDesktop {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetWmDesktopReply {
    raw: *const u8,
    window: x::Window,
}

impl EwmhReply for GetWmDesktopReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut window = 0;

        ffi::xcb_ewmh_get_wm_desktop_from_reply(
            &mut window,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let window = <x::Window as xcb::XidNew>::new(window);

        Self { raw, window }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetWmDesktopReply {
    pub fn window(&self) -> x::Window {
        self.window
    }
}

//TODO: Expose inner cookie
pub struct GetWmDesktopCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetWmDesktopCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetWmDesktopCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetWmDesktopCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetWmDesktopCookie {
    type Reply = GetWmDesktopReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut window = 0;
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_wm_desktop_reply(ewmh.ewmh.get(), cookie, &mut window, &mut e);

            let window = <x::Window as xcb::XidNew>::new(window);

            Ok(Self::Reply { raw, window })
        }
    }
}

impl xcb::Cookie for GetWmDesktopCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetWmDesktopCookieUnchecked {
    type Reply = GetWmDesktopReply;

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

            let raw =
                &ffi::xcb_ewmh_get_wm_desktop_reply(ewmh.ewmh.get(), cookie, &mut window, &mut e);

            let window = <x::Window as xcb::XidNew>::new(window);

            Ok(Some(Self::Reply { raw, window }))
        }
    }
}

pub struct GetWmDesktop {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetWmDesktop {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_wm_desktop(ewmh.ewmh.get(), xcb::Xid::resource_id(&self.window))
            } else {
                ffi::xcb_ewmh_get_wm_desktop_unchecked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetWmDesktop {
    type Cookie = GetWmDesktopCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetWmDesktop {
    type Reply = GetWmDesktopReply;
    type Cookie = GetWmDesktopCookie;
    type CookieUnchecked = GetWmDesktopCookieUnchecked;
}
