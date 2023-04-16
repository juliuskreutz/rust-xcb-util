use std::{mem, ptr};

use xcb::x;

use super::{
    ffi, DesktopLayoutOrientation, DesktopLayoutStartingCorner, EwmhConnection,
    EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply, EwmhRequest,
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
                    self.orientation as u32,
                    self.columns,
                    self.rows,
                    self.starting_corner as ffi::xcb_ewmh_desktop_layout_starting_corner_t,
                )
            } else {
                ffi::xcb_ewmh_set_desktop_layout(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.orientation as u32,
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
    orientation: DesktopLayoutOrientation,
    columns: u32,
    rows: u32,
    starting_corner: DesktopLayoutStartingCorner,
}

impl EwmhReply for GetDesktopLayoutReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut desktop_layout = mem::zeroed();

        ffi::xcb_ewmh_get_desktop_layout_from_reply(
            &mut desktop_layout,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let orientation = DesktopLayoutOrientation::from(desktop_layout.orientation);
        let columns = desktop_layout.columns;
        let rows = desktop_layout.rows;
        let starting_corner = DesktopLayoutStartingCorner::from(desktop_layout.starting_corner);

        Self {
            raw,
            orientation,
            columns,
            rows,
            starting_corner,
        }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetDesktopLayoutReply {
    pub fn orientation(&self) -> DesktopLayoutOrientation {
        self.orientation
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn starting_corner(&self) -> DesktopLayoutStartingCorner {
        self.starting_corner
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

unsafe impl EwmhCookieWithReplyChecked for GetDesktopLayoutCookie {
    type Reply = GetDesktopLayoutReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut desktop_layout = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_desktop_layout_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut desktop_layout,
                &mut e,
            );

            let orientation = DesktopLayoutOrientation::from(desktop_layout.orientation);
            let columns = desktop_layout.columns;
            let rows = desktop_layout.rows;
            let starting_corner = DesktopLayoutStartingCorner::from(desktop_layout.starting_corner);

            Ok(Self::Reply {
                raw,
                orientation,
                columns,
                rows,
                starting_corner,
            })
        }
    }
}

impl xcb::Cookie for GetDesktopLayoutCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetDesktopLayoutCookieUnchecked {
    type Reply = GetDesktopLayoutReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut desktop_layout = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_desktop_layout_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut desktop_layout,
                &mut e,
            );

            let orientation = DesktopLayoutOrientation::from(desktop_layout.orientation);
            let columns = desktop_layout.columns;
            let rows = desktop_layout.rows;
            let starting_corner = DesktopLayoutStartingCorner::from(desktop_layout.starting_corner);

            Ok(Some(Self::Reply {
                raw,
                orientation,
                columns,
                rows,
                starting_corner,
            }))
        }
    }
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
