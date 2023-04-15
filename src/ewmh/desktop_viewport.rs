use std::{mem, ptr, slice};

use xcb::x;

use super::{
    ffi, Coordinates, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked,
    EwmhReply, EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetDesktopViewport<'a> {
    pub screen_nbr: i32,
    pub coordinates: &'a [Coordinates],
}

unsafe impl<'a> RawEwmhRequest for SetDesktopViewport<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_desktop_viewport_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.coordinates.len() as u32,
                    self.coordinates.as_ptr() as *mut ffi::xcb_ewmh_coordinates_t,
                )
            } else {
                ffi::xcb_ewmh_set_desktop_viewport(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.coordinates.len() as u32,
                    self.coordinates.as_ptr() as *mut ffi::xcb_ewmh_coordinates_t,
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetDesktopViewport<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetDesktopViewport<'a> {}

pub struct RequestChangeDesktopViewport {
    pub screen_nbr: i32,
    pub x: u32,
    pub y: u32,
}

unsafe impl RawEwmhRequest for RequestChangeDesktopViewport {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_desktop_viewport(
                ewmh.ewmh.get(),
                self.screen_nbr,
                self.x,
                self.y,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeDesktopViewport {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

// TODO: Expose inner reply
pub struct GetDesktopViewportReply {
    raw: *const u8,
    coordinates: Vec<Coordinates>,
}

impl EwmhReply for GetDesktopViewportReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut vp = mem::zeroed();

        ffi::xcb_ewmh_get_desktop_viewport_from_reply(
            &mut vp,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let coordinates = slice::from_raw_parts(
            vp.desktop_viewport as *mut Coordinates,
            vp.desktop_viewport_len as usize,
        )
        .to_vec();

        ffi::xcb_ewmh_get_desktop_viewport_reply_wipe(&mut vp);

        Self { raw, coordinates }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetDesktopViewportReply {
    pub fn coordiantes(&self) -> &[Coordinates] {
        &self.coordinates
    }
}

//TODO: Expose inner cookie
pub struct GetDesktopViewportCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetDesktopViewportCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetDesktopViewportCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetDesktopViewportCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetDesktopViewportCookie {
    type Reply = GetDesktopViewportReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut vp = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_desktop_viewport_reply(ewmh.ewmh.get(), cookie, &mut vp, &mut e);

            let coordinates = slice::from_raw_parts(
                vp.desktop_viewport as *mut Coordinates,
                vp.desktop_viewport_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_desktop_viewport_reply_wipe(&mut vp);

            Ok(Self::Reply { raw, coordinates })
        }
    }
}

impl xcb::Cookie for GetDesktopViewportCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetDesktopViewportCookieUnchecked {
    type Reply = GetDesktopViewportReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut vp = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_desktop_viewport_reply(ewmh.ewmh.get(), cookie, &mut vp, &mut e);

            let coordinates = slice::from_raw_parts(
                vp.desktop_viewport as *mut Coordinates,
                vp.desktop_viewport_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_desktop_viewport_reply_wipe(&mut vp);

            Ok(Some(Self::Reply { raw, coordinates }))
        }
    }
}

pub struct GetDesktopViewport {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetDesktopViewport {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_desktop_viewport(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_desktop_viewport_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetDesktopViewport {
    type Cookie = GetDesktopViewportCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetDesktopViewport {
    type Reply = GetDesktopViewportReply;
    type Cookie = GetDesktopViewportCookie;
    type CookieUnchecked = GetDesktopViewportCookieUnchecked;
}
