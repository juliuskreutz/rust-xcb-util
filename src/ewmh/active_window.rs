use std::ptr;

use num_traits::ToPrimitive;
use xcb::x;

use super::{
    ffi, ClientSourceType, EwmhConnection, EwmhCookieWithReplyChecked,
    EwmhCookieWithReplyUnchecked, EwmhReply, EwmhRequest, EwmhRequestWithReply,
    EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetActiveWindow {
    pub screen_nbr: i32,
    pub new_active_window: x::Window,
}

unsafe impl RawEwmhRequest for SetActiveWindow {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_active_window_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    xcb::Xid::resource_id(&self.new_active_window),
                )
            } else {
                ffi::xcb_ewmh_set_active_window(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    xcb::Xid::resource_id(&self.new_active_window),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetActiveWindow {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetActiveWindow {}

pub struct RequestChangeActiveWindow {
    pub screen_nbr: i32,
    pub window_to_activate: x::Window,
    pub timestamp: x::Timestamp,
    pub source_indication: ClientSourceType,
    pub current_active_window: x::Window,
}

unsafe impl RawEwmhRequest for RequestChangeActiveWindow {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_active_window(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.window_to_activate),
                self.source_indication.to_u32().unwrap(),
                self.timestamp,
                xcb::Xid::resource_id(&self.current_active_window),
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeActiveWindow {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetActiveWindowReply {
    raw: *const u8,
    window: x::Window,
}

impl EwmhReply for GetActiveWindowReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut window = 0;

        ffi::xcb_ewmh_get_active_window_from_reply(
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

impl GetActiveWindowReply {
    pub fn window(&self) -> x::Window {
        self.window
    }
}

//TODO: Expose inner cookie
pub struct GetActiveWindowCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetActiveWindowCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetActiveWindowCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetActiveWindowCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetActiveWindowCookie {
    type Reply = GetActiveWindowReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut window = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_active_window_reply(
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

impl xcb::Cookie for GetActiveWindowCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetActiveWindowCookieUnchecked {
    type Reply = GetActiveWindowReply;

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

            let raw = &ffi::xcb_ewmh_get_active_window_reply(
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

pub struct GetActiveWindow {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetActiveWindow {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_active_window(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_active_window_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetActiveWindow {
    type Cookie = GetActiveWindowCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetActiveWindow {
    type Reply = GetActiveWindowReply;
    type Cookie = GetActiveWindowCookie;
    type CookieUnchecked = GetActiveWindowCookieUnchecked;
}
