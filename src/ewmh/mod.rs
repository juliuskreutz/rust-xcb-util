use std::{cell::UnsafeCell, mem, ops::Deref, ptr};

pub use xcb_util_sys::ewmh as ffi;

mod active_window;
mod client_list;
mod client_list_stacking;
mod close_window;
mod current_desktop;
mod desktop_geometry;
mod desktop_layout;
mod desktop_names;
mod desktop_viewport;
mod frame_extents;
mod moveresize_window;
mod number_of_desktops;
mod restack_window;
mod showing_desktop;
mod supported;
mod supporting_wm_check;
mod virtual_roots;
mod wm_desktop;
mod wm_icon_name;
mod wm_moveresize;
mod wm_name;
mod wm_state;
mod wm_visible_icon_name;
mod wm_visible_name;
mod wm_window_type;
mod workarea;

pub use self::active_window::*;
pub use self::client_list::*;
pub use self::client_list_stacking::*;
pub use self::close_window::*;
pub use self::current_desktop::*;
pub use self::desktop_geometry::*;
pub use self::desktop_layout::*;
pub use self::desktop_names::*;
pub use self::desktop_viewport::*;
pub use self::frame_extents::*;
pub use self::moveresize_window::*;
pub use self::number_of_desktops::*;
pub use self::restack_window::*;
pub use self::showing_desktop::*;
pub use self::supported::*;
pub use self::supporting_wm_check::*;
pub use self::virtual_roots::*;
pub use self::wm_desktop::*;
pub use self::wm_icon_name::*;
pub use self::wm_moveresize::*;
pub use self::wm_name::*;
pub use self::wm_state::*;
pub use self::wm_visible_icon_name::*;
pub use self::wm_visible_name::*;
pub use self::wm_window_type::*;
pub use self::workarea::*;

pub trait EwmhReply {
    /// # Safety
    /// `raw` must be a pointer to a valid wire representation of `Self`, allocated with [`libc::malloc`].
    unsafe fn from_raw(raw: *const u8, ewmh: *mut ffi::xcb_ewmh_connection_t) -> Self;

    /// # Safety
    /// The returned pointer must be freed with [`libc::free`] to avoid any memory leak, or be used
    /// to build another reply.
    unsafe fn into_raw(self) -> *const u8;
}

impl<T: xcb::Reply> EwmhReply for T {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
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
#[repr(u32)]
pub enum ClientSourceType {
    None = ffi::XCB_EWMH_CLIENT_SOURCE_TYPE_NONE,
    Normal = ffi::XCB_EWMH_CLIENT_SOURCE_TYPE_NORMAL,
    Other = ffi::XCB_EWMH_CLIENT_SOURCE_TYPE_OTHER,
}

impl From<u32> for ClientSourceType {
    fn from(value: u32) -> Self {
        match value {
            ffi::XCB_EWMH_CLIENT_SOURCE_TYPE_NONE => Self::None,
            ffi::XCB_EWMH_CLIENT_SOURCE_TYPE_NORMAL => Self::Normal,
            ffi::XCB_EWMH_CLIENT_SOURCE_TYPE_OTHER => Self::Other,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum DesktopLayoutOrientation {
    Horz = ffi::XCB_EWMH_WM_ORIENTATION_HORZ,
    Vert = ffi::XCB_EWMH_WM_ORIENTATION_VERT,
}

impl From<u32> for DesktopLayoutOrientation {
    fn from(value: u32) -> Self {
        match value {
            ffi::XCB_EWMH_WM_ORIENTATION_HORZ => Self::Horz,
            ffi::XCB_EWMH_WM_ORIENTATION_VERT => Self::Vert,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum DesktopLayoutStartingCorner {
    TopLeft = ffi::XCB_EWMH_WM_TOPLEFT,
    TopRight = ffi::XCB_EWMH_WM_TOPRIGHT,
    BottomRight = ffi::XCB_EWMH_WM_BOTTOMRIGHT,
    BottomLeft = ffi::XCB_EWMH_WM_BOTTOMLEFT,
}

impl From<u32> for DesktopLayoutStartingCorner {
    fn from(value: u32) -> Self {
        match value {
            ffi::XCB_EWMH_WM_TOPLEFT => Self::TopLeft,
            ffi::XCB_EWMH_WM_TOPRIGHT => Self::TopRight,
            ffi::XCB_EWMH_WM_BOTTOMRIGHT => Self::BottomRight,
            ffi::XCB_EWMH_WM_BOTTOMLEFT => Self::BottomLeft,
            _ => todo!(),
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MoveresizeWindowOptFlags: u32 {
        const X = ffi::XCB_EWMH_MOVERESIZE_WINDOW_X;
        const Y = ffi::XCB_EWMH_MOVERESIZE_WINDOW_Y;
        const Width = ffi::XCB_EWMH_MOVERESIZE_WINDOW_WIDTH;
        const Height = ffi::XCB_EWMH_MOVERESIZE_WINDOW_HEIGHT;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum MoveresizeDirection {
    SizeTopLeft = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_TOPLEFT,
    SizeTop = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_TOP,
    SizeTopRight = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_TOPRIGHT,
    SizeRight = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_RIGHT,
    SizeBottomRight = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOMRIGHT,
    SizeBottom = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOM,
    SizeBottomLeft = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOMLEFT,
    SizeLeft = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_LEFT,
    Move = ffi::XCB_EWMH_WM_MOVERESIZE_MOVE,
    SizeKeyboard = ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_KEYBOARD,
    MoveKeyboard = ffi::XCB_EWMH_WM_MOVERESIZE_MOVE_KEYBOARD,
    Cancel = ffi::XCB_EWMH_WM_MOVERESIZE_CANCEL,
}

impl From<u32> for MoveresizeDirection {
    fn from(value: u32) -> Self {
        match value {
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_TOPLEFT => Self::SizeTopLeft,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_TOP => Self::SizeTop,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_TOPRIGHT => Self::SizeTopRight,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_RIGHT => Self::SizeRight,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOMRIGHT => Self::SizeBottomRight,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOM => Self::SizeBottom,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOMLEFT => Self::SizeBottomLeft,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_LEFT => Self::SizeLeft,
            ffi::XCB_EWMH_WM_MOVERESIZE_MOVE => Self::Move,
            ffi::XCB_EWMH_WM_MOVERESIZE_SIZE_KEYBOARD => Self::SizeKeyboard,
            ffi::XCB_EWMH_WM_MOVERESIZE_MOVE_KEYBOARD => Self::MoveKeyboard,
            ffi::XCB_EWMH_WM_MOVERESIZE_CANCEL => Self::Cancel,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum WmStateAction {
    Remove = ffi::XCB_EWMH_WM_STATE_REMOVE,
    Add = ffi::XCB_EWMH_WM_STATE_ADD,
    Toggle = ffi::XCB_EWMH_WM_STATE_TOGGLE,
}

impl From<u32> for WmStateAction {
    fn from(value: u32) -> Self {
        match value {
            ffi::XCB_EWMH_WM_STATE_REMOVE => Self::Remove,
            ffi::XCB_EWMH_WM_STATE_ADD => Self::Add,
            ffi::XCB_EWMH_WM_STATE_TOGGLE => Self::Toggle,
            _ => todo!(),
        }
    }
}
