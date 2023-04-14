use std::mem;

use xcb::{x, Xid, XidNew};

use super::{
    ffi, DesktopLayoutOrientation, DesktopLayoutStartingCorner, EwmhConnection, EwmhRequest,
    EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetDesktopLayout {
    pub screen_nbr: i32,
    pub orientation: DesktopLayoutOrientation,
    pub columns: u32,
    pub rows: u32,
    pub starting_corner: DesktopLayoutStartingCorner,
}

unsafe impl RawEwmhRequest for SetDesktopLayout {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_desktop_layout_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.orientation as ffi::xcb_ewmh_desktop_layout_orientation_t,
                    self.columns,
                    self.rows,
                    self.starting_corner as ffi::xcb_ewmh_desktop_layout_starting_corner_t,
                )
            } else {
                ffi::xcb_ewmh_set_desktop_layout(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.orientation as ffi::xcb_ewmh_desktop_layout_orientation_t,
                    self.columns,
                    self.rows,
                    self.starting_corner as ffi::xcb_ewmh_desktop_layout_starting_corner_t,
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetDesktopLayout {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetDesktopLayout {}

// TODO: Expose inner reply
pub struct GetDesktopLayoutReply {
    raw: *const u8,
}

impl xcb::Reply for GetDesktopLayoutReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetDesktopLayoutReply {
    pub fn window(&self) -> x::Window {
        unsafe {
            let mut desktop_layout = mem::zeroed();

            ffi::xcb_ewmh_get_desktop_layout_from_reply(
                &mut desktop_layout,
                self.raw as *mut ffi::xcb_get_property_reply_t,
            );

            desktop_layout.
        }
    }
}

//TODO: Expose inner cookie
pub struct GetDesktopLayoutCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetDesktopLayoutCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetDesktopLayoutCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetDesktopLayoutCookie {}

unsafe impl xcb::CookieWithReplyChecked for GetDesktopLayoutCookie {
    type Reply = GetDesktopLayoutReply;
}

impl xcb::Cookie for GetDesktopLayoutCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetDesktopLayoutCookieUnchecked {
    type Reply = GetDesktopLayoutReply;
}

pub struct GetDesktopLayout {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetDesktopLayout {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_desktop_layout(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_desktop_layout_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetDesktopLayout {
    type Cookie = GetDesktopLayoutCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetDesktopLayout {
    type Reply = GetDesktopLayoutReply;
    type Cookie = GetDesktopLayoutCookie;
    type CookieUnchecked = GetDesktopLayoutCookieUnchecked;
}
