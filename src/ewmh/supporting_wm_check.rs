use xcb::{x, Xid, XidNew};

use super::{
    ffi, EwmhConnection, EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetSupportingWmCheck {
    pub parent_window: x::Window,
    pub child_window: x::Window,
}

unsafe impl RawEwmhRequest for SetSupportingWmCheck {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_supporting_wm_check_checked(
                    ewmh.ewmh.get(),
                    self.parent_window.resource_id(),
                    self.child_window.resource_id(),
                )
            } else {
                ffi::xcb_ewmh_set_supporting_wm_check(
                    ewmh.ewmh.get(),
                    self.parent_window.resource_id(),
                    self.child_window.resource_id(),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for SetSupportingWmCheck {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl EwmhRequestWithoutReply for SetSupportingWmCheck {}

// TODO: Expose inner reply
pub struct GetSupportingWmCheckReply {
    raw: *const u8,
}

impl xcb::Reply for GetSupportingWmCheckReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetSupportingWmCheckReply {
    pub fn window(&self) -> x::Window {
        unsafe {
            let mut supporting_wm_check = 0;

            ffi::xcb_ewmh_get_supporting_wm_check_from_reply(
                &mut supporting_wm_check,
                self.raw as *mut ffi::xcb_get_property_reply_t,
            );

            x::Window::new(supporting_wm_check)
        }
    }
}

//TODO: Expose inner cookie
pub struct GetSupportingWmCheckCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetSupportingWmCheckCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetSupportingWmCheckCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetSupportingWmCheckCookie {}

unsafe impl xcb::CookieWithReplyChecked for GetSupportingWmCheckCookie {
    type Reply = GetSupportingWmCheckReply;
}

impl xcb::Cookie for GetSupportingWmCheckCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieWithReplyUnchecked for GetSupportingWmCheckCookieUnchecked {
    type Reply = GetSupportingWmCheckReply;
}

pub struct GetSupportingWmCheck {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetSupportingWmCheck {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_supporting_wm_check(ewmh.ewmh.get(), self.window.resource_id())
            } else {
                ffi::xcb_ewmh_get_supporting_wm_check_unchecked(
                    ewmh.ewmh.get(),
                    self.window.resource_id(),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetSupportingWmCheck {
    type Cookie = GetSupportingWmCheckCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetSupportingWmCheck {
    type Reply = GetSupportingWmCheckReply;
    type Cookie = GetSupportingWmCheckCookie;
    type CookieUnchecked = GetSupportingWmCheckCookieUnchecked;
}
