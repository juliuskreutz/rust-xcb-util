use std::ptr;

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
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
                    xcb::Xid::resource_id(&self.parent_window),
                    xcb::Xid::resource_id(&self.child_window),
                )
            } else {
                ffi::xcb_ewmh_set_supporting_wm_check(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.parent_window),
                    xcb::Xid::resource_id(&self.child_window),
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
    window: x::Window,
}

impl EwmhReply for GetSupportingWmCheckReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut supporting_wm_check = 0;

        ffi::xcb_ewmh_get_supporting_wm_check_from_reply(
            &mut supporting_wm_check,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let window = <x::Window as xcb::XidNew>::new(supporting_wm_check);

        Self { raw, window }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetSupportingWmCheckReply {
    pub fn window(&self) -> x::Window {
        self.window
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

unsafe impl EwmhCookieWithReplyChecked for GetSupportingWmCheckCookie {
    type Reply = GetSupportingWmCheckReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut window = 0;
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_supporting_wm_check_reply(
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

impl xcb::Cookie for GetSupportingWmCheckCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetSupportingWmCheckCookieUnchecked {
    type Reply = GetSupportingWmCheckReply;

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

            let raw = &ffi::xcb_ewmh_get_supporting_wm_check_reply(
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

pub struct GetSupportingWmCheck {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetSupportingWmCheck {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_supporting_wm_check(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            } else {
                ffi::xcb_ewmh_get_supporting_wm_check_unchecked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
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
