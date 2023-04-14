use std::{mem, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, Geometry,
    RawEwmhRequest,
};

pub struct SetWorkarea<'a> {
    pub screen_nbr: i32,
    pub geometries: &'a [Geometry],
}

unsafe impl<'a> RawEwmhRequest for SetWorkarea<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_workarea_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.geometries.len() as u32,
                    self.geometries.as_ptr() as *mut ffi::xcb_ewmh_geometry_t,
                )
            } else {
                ffi::xcb_ewmh_set_workarea(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.geometries.len() as u32,
                    self.geometries.as_ptr() as *mut ffi::xcb_ewmh_geometry_t,
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetWorkarea<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetWorkarea<'a> {}

// TODO: Expose inner reply
pub struct GetWorkareaReply {
    raw: *const u8,
}

impl xcb::Reply for GetWorkareaReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetWorkareaReply {
    pub fn geometries(&self) -> Vec<Geometry> {
        unsafe {
            let mut wa = mem::zeroed();

            ffi::xcb_ewmh_get_workarea_from_reply(
                &mut wa,
                self.raw as *mut ffi::xcb_get_property_reply_t,
            );

            let geometries =
                slice::from_raw_parts(wa.workarea as *mut Geometry, wa.workarea_len as usize)
                    .to_vec();

            ffi::xcb_ewmh_get_workarea_reply_wipe(&mut wa);

            geometries
        }
    }
}

//TODO: Expose inner cookie
pub struct GetWorkareaCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetWorkareaCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetWorkareaCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetWorkareaCookie {}

unsafe impl xcb::CookieWithReplyChecked for GetWorkareaCookie {
    type Reply = GetWorkareaReply;
}

impl xcb::Cookie for GetWorkareaCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetWorkareaCookieUnchecked {
    type Reply = GetWorkareaReply;
}

pub struct GetWorkarea {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetWorkarea {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_workarea(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_workarea_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetWorkarea {
    type Cookie = GetWorkareaCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetWorkarea {
    type Reply = GetWorkareaReply;
    type Cookie = GetWorkareaCookie;
    type CookieUnchecked = GetWorkareaCookieUnchecked;
}
