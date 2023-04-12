use std::{
    ffi::c_int,
    ops::Deref,
    ptr::{self, NonNull},
};

use xcb::{x, Xid};
use xcb_util_sys::ewmh::*;

/// # Safety
/// Types implementing this trait acknowledge that the returned value of `raw_ewmh_request` correspond
/// to a cookie for `Self` request and is checked or unchecked depending on the `checked` flag value.
pub unsafe trait RawEwmhRequest {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64;
}

unsafe impl<T: xcb::RawRequest> RawEwmhRequest for T {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        self.raw_request(ewmh, checked)
    }
}

pub trait EwmhRequest: RawEwmhRequest {
    type Cookie: xcb::Cookie;

    const IS_VOID: bool;
}

impl<T: xcb::Request> EwmhRequest for T {
    type Cookie = <T as xcb::Request>::Cookie;

    const IS_VOID: bool = <T as xcb::Request>::IS_VOID;
}

pub trait EwmhRequestWithoutReply: EwmhRequest {}

impl<T: xcb::RequestWithoutReply> EwmhRequestWithoutReply for T {}

pub trait EwmhRequestWithReply: EwmhRequest {
    type Reply: xcb::Reply;
    type Cookie: xcb::CookieWithReplyChecked<Reply = Self::Reply>;
    type CookieUnchecked: xcb::CookieWithReplyUnchecked<Reply = Self::Reply>;
}

impl<T: xcb::RequestWithReply> EwmhRequestWithReply for T {
    type Reply = <T as xcb::RequestWithReply>::Reply;

    type Cookie = <T as xcb::RequestWithReply>::Cookie;

    type CookieUnchecked = <T as xcb::RequestWithReply>::CookieUnchecked;
}

pub struct EwmhConnection<'a> {
    raw: NonNull<xcb_ewmh_connection_t>,
    connection: &'a xcb::Connection,
}

impl<'a> Deref for EwmhConnection<'a> {
    type Target = xcb::Connection;

    fn deref(&self) -> &Self::Target {
        self.connection
    }
}

impl<'a> EwmhConnection<'a> {
    pub fn new(connection: &'a xcb::Connection) -> Option<Self> {
        let ewmh = ptr::null_mut();

        let ewmh_cookies = unsafe { xcb_ewmh_init_atoms(connection.get_raw_conn(), ewmh) };

        let e = ptr::null_mut();

        if unsafe { xcb_ewmh_init_atoms_replies(ewmh, ewmh_cookies, e) } == 1 {
            ptr::NonNull::new(ewmh).map(|inner| Self {
                raw: inner,
                connection,
            })
        } else {
            None
        }
    }

    pub fn send_request<R>(&self, req: &R) -> R::Cookie
    where
        R: EwmhRequest,
    {
        unsafe {
            <R::Cookie as xcb::Cookie>::from_sequence(req.raw_ewmh_request(self, !R::IS_VOID))
        }
    }

    pub fn send_request_checked<R>(&self, req: &R) -> xcb::VoidCookieChecked
    where
        R: EwmhRequestWithoutReply,
    {
        unsafe {
            <xcb::VoidCookieChecked as xcb::Cookie>::from_sequence(req.raw_ewmh_request(self, true))
        }
    }

    pub fn send_request_unchecked<R>(&self, req: &R) -> R::CookieUnchecked
    where
        R: EwmhRequestWithReply,
    {
        unsafe {
            <R::CookieUnchecked as xcb::Cookie>::from_sequence(req.raw_ewmh_request(self, false))
        }
    }

    pub fn send_and_check_request<R>(&self, req: &R) -> xcb::ProtocolResult<()>
    where
        R: EwmhRequestWithoutReply,
    {
        self.check_request(self.send_request_checked(req))
    }
}

impl<'a> Drop for EwmhConnection<'a> {
    fn drop(&mut self) {
        unsafe { xcb_ewmh_connection_wipe(self.raw.as_ptr()) }
    }
}

pub struct SetSupported<'a> {
    pub screen_nbr: c_int,
    pub atoms: &'a [x::Atom],
}

unsafe impl<'a> RawEwmhRequest for SetSupported<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                xcb_ewmh_set_supported_checked(
                    ewmh.raw.as_ptr(),
                    self.screen_nbr,
                    self.atoms.len() as u32,
                    self.atoms
                        .iter()
                        .map(|a| a.resource_id())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                )
            } else {
                xcb_ewmh_set_supported(
                    ewmh.raw.as_ptr(),
                    self.screen_nbr,
                    self.atoms.len() as u32,
                    self.atoms
                        .iter()
                        .map(|a| a.resource_id())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetSupported<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetSupported<'a> {}

pub struct GetPropertyReply {
    raw: *const xcb_get_property_reply_t,
}

impl GetPropertyReply {
    //TODO: Impl
}

impl xcb::Reply for GetPropertyReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self {
            raw: raw as *const xcb_get_property_reply_t,
        }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw as *const u8
    }
}

pub struct GetSupported {
    pub screen_nbr: c_int,
}

unsafe impl RawEwmhRequest for GetSupported {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                xcb_ewmh_get_supported(ewmh.raw.as_ptr(), self.screen_nbr)
            } else {
                xcb_ewmh_get_supported_unchecked(ewmh.raw.as_ptr(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

pub struct GetPropertyCookie {
    seq: u64,
}

pub struct GetPropertyCookieUnchecked {
    seq: u64,
}

impl xcb::Cookie for GetPropertyCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPropertyCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl xcb::CookieChecked for GetPropertyCookie {}

unsafe impl xcb::CookieWithReplyChecked for GetPropertyCookie {
    type Reply = GetPropertyReply;
}

impl xcb::Cookie for GetPropertyCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPropertyCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetPropertyCookieUnchecked {
    type Reply = GetPropertyReply;
}

impl EwmhRequest for GetSupported {
    type Cookie = GetPropertyCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetSupported {
    type Reply = GetPropertyReply;
    type Cookie = GetPropertyCookie;
    type CookieUnchecked = GetPropertyCookieUnchecked;
}
