use std::ptr;

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetDesktopGeometry {
    pub screen_nbr: i32,
    pub new_width: u32,
    pub new_height: u32,
}

unsafe impl RawEwmhRequest for SetDesktopGeometry {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_desktop_geometry_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.new_width,
                    self.new_height,
                )
            } else {
                ffi::xcb_ewmh_set_desktop_geometry(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.new_width,
                    self.new_height,
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetDesktopGeometry {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetDesktopGeometry {}

pub struct RequestChangeDesktopGeometry {
    pub screen_nbr: i32,
    pub new_width: u32,
    pub new_height: u32,
}

unsafe impl RawEwmhRequest for RequestChangeDesktopGeometry {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_desktop_geometry(
                ewmh.ewmh.get(),
                self.screen_nbr,
                self.new_width,
                self.new_height,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeDesktopGeometry {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetDesktopGeometryReply {
    raw: *const u8,
    width: u32,
    height: u32,
}

impl EwmhReply for GetDesktopGeometryReply {
    unsafe fn from_raw(raw: *const u8, _: &EwmhConnection) -> Self {
        let mut width = 0;
        let mut height = 0;

        ffi::xcb_ewmh_get_desktop_geometry_from_reply(
            &mut width,
            &mut height,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        Self { raw, width, height }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetDesktopGeometryReply {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

//TODO: Expose inner cookie
pub struct GetDesktopGeometryCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetDesktopGeometryCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetDesktopGeometryCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetDesktopGeometryCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetDesktopGeometryCookie {
    type Reply = GetDesktopGeometryReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut width = 0;
            let mut height = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_desktop_geometry_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut width,
                &mut height,
                &mut e,
            );

            Ok(Self::Reply { raw, width, height })
        }
    }
}

impl xcb::Cookie for GetDesktopGeometryCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetDesktopGeometryCookieUnchecked {
    type Reply = GetDesktopGeometryReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut width = 0;
            let mut height = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_desktop_geometry_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut width,
                &mut height,
                &mut e,
            );

            Ok(Some(Self::Reply { raw, width, height }))
        }
    }
}

pub struct GetDesktopGeometry {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetDesktopGeometry {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_desktop_geometry(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_desktop_geometry_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetDesktopGeometry {
    type Cookie = GetDesktopGeometryCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetDesktopGeometry {
    type Reply = GetDesktopGeometryReply;
    type Cookie = GetDesktopGeometryCookie;
    type CookieUnchecked = GetDesktopGeometryCookieUnchecked;
}
