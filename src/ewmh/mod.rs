use std::{cell::UnsafeCell, mem, ops::Deref, ptr};

pub use xcb_util_sys::ewmh as ffi;

mod active_window;
mod client_list;
mod client_list_stacking;
mod current_desktop;
mod desktop_geometry;
// mod desktop_layout;
mod desktop_names;
mod desktop_viewport;
mod number_of_desktops;
mod supported;
mod supporting_wm_check;
mod virtual_roots;
mod workarea;

pub use self::active_window::*;
pub use self::client_list::*;
pub use self::client_list_stacking::*;
pub use self::current_desktop::*;
pub use self::desktop_geometry::*;
// pub use self::desktop_layout::*;
pub use self::desktop_names::*;
pub use self::desktop_viewport::*;
pub use self::number_of_desktops::*;
pub use self::supported::*;
pub use self::supporting_wm_check::*;
pub use self::virtual_roots::*;
pub use self::workarea::*;

pub trait EwmhReply {
    /// # Safety
    /// `raw` must be a pointer to a valid wire representation of `Self`, allocated with [`libc::malloc`].
    unsafe fn from_raw(raw: *const u8, ewmh: &EwmhConnection) -> Self;

    /// # Safety
    /// The returned pointer must be freed with [`libc::free`] to avoid any memory leak, or be used
    /// to build another reply.
    unsafe fn into_raw(self) -> *const u8;
}

impl<T: xcb::Reply> EwmhReply for T {
    unsafe fn from_raw(raw: *const u8, _: &EwmhConnection) -> Self {
        <T as xcb::Reply>::from_raw(raw)
    }

    unsafe fn into_raw(self) -> *const u8 {
        <T as xcb::Reply>::into_raw(self)
    }
}

/// # Safety
/// Cookies implementing this trait acknowledge that their error is checked when the reply is fetched from the server.
/// This is the default cookie type for requests with reply.
pub unsafe trait EwmhCookieWithReplyChecked: xcb::CookieChecked {
    type Reply: EwmhReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply>;
}

unsafe impl<T: xcb::CookieWithReplyChecked> EwmhCookieWithReplyChecked for T {
    type Reply = <T as xcb::CookieWithReplyChecked>::Reply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        ewmh.connection.wait_for_reply(self)
    }
}

/// # Safety
/// Cookies implementing this trait acknowledge that their error is checked when the reply is fetched from the server.
/// This is the default cookie type for requests with reply.
pub unsafe trait EwmhCookieWithReplyUnchecked: xcb::Cookie {
    type Reply: EwmhReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>>;
}

unsafe impl<T: xcb::CookieWithReplyUnchecked> EwmhCookieWithReplyUnchecked for T {
    type Reply = <T as xcb::CookieWithReplyUnchecked>::Reply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        ewmh.connection.wait_for_reply_unchecked(self)
    }
}

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
    type Reply: EwmhReply;
    type Cookie: EwmhCookieWithReplyChecked<Reply = Self::Reply>;
    type CookieUnchecked: EwmhCookieWithReplyUnchecked<Reply = Self::Reply>;
}

impl<T: xcb::RequestWithReply> EwmhRequestWithReply for T {
    type Reply = <T as xcb::RequestWithReply>::Reply;
    type Cookie = <T as xcb::RequestWithReply>::Cookie;
    type CookieUnchecked = <T as xcb::RequestWithReply>::CookieUnchecked;
}

pub struct EwmhConnection<'a> {
    ewmh: UnsafeCell<ffi::xcb_ewmh_connection_t>,
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
        unsafe {
            let ewmh = UnsafeCell::new(mem::zeroed());

            let ewmh_cookies = ffi::xcb_ewmh_init_atoms(connection.get_raw_conn(), ewmh.get());

            let mut e = ptr::null_mut();

            if ffi::xcb_ewmh_init_atoms_replies(ewmh.get(), ewmh_cookies, &mut e) == 1 {
                Some(Self { ewmh, connection })
            } else {
                None
            }
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

    pub fn wait_for_reply<C>(&self, cookie: C) -> xcb::Result<C::Reply>
    where
        C: EwmhCookieWithReplyChecked,
    {
        cookie.wait_for_reply(self)
    }

    pub fn wait_for_reply_unchecked<C>(&self, cookie: C) -> xcb::ConnResult<Option<C::Reply>>
    where
        C: EwmhCookieWithReplyUnchecked,
    {
        cookie.wait_for_reply_unchecked(self)
    }
}

impl<'a> Drop for EwmhConnection<'a> {
    fn drop(&mut self) {
        unsafe { ffi::xcb_ewmh_connection_wipe(self.ewmh.get()) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Geometry {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClientSourceType {
    None,
    Normal,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DesktopLayoutOrientation {
    Horz,
    Vert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DesktopLayoutStartingCorner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}
