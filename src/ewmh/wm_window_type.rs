use std::{mem, ptr, slice};

use xcb::x;

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetWmWindowType<'a> {
    pub window: x::Window,
    pub atoms: &'a [x::Atom],
}

unsafe impl<'a> RawEwmhRequest for SetWmWindowType<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_wm_window_type_checked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    self.atoms.len() as u32,
                    self.atoms.as_ptr() as *mut ffi::xcb_atom_t,
                )
            } else {
                ffi::xcb_ewmh_set_wm_window_type(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    self.atoms.len() as u32,
                    self.atoms.as_ptr() as *mut ffi::xcb_atom_t,
                )
            }
            .sequence as u64
        }
    }
}

impl<'a> EwmhRequest for SetWmWindowType<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetWmWindowType<'a> {}

//TODO: Expose inner reply
pub struct GetWmWindowTypeReply {
    raw: *const u8,
    atoms: Vec<x::Atom>,
}

impl EwmhReply for GetWmWindowTypeReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut wm_window_type = mem::zeroed();

        ffi::xcb_ewmh_get_wm_window_type_from_reply(
            &mut wm_window_type,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let atoms = slice::from_raw_parts(
            wm_window_type.atoms as *mut x::Atom,
            wm_window_type.atoms_len as usize,
        )
        .to_vec();

        ffi::xcb_ewmh_get_atoms_reply_wipe(&mut wm_window_type);

        Self { raw, atoms }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetWmWindowTypeReply {
    pub fn atoms(&self) -> &[x::Atom] {
        &self.atoms
    }
}

//TODO: Expose inner cookie
pub struct GetWmWindowTypeCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetWmWindowTypeCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetWmWindowTypeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetWmWindowTypeCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetWmWindowTypeCookie {
    type Reply = GetWmWindowTypeReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut wm_window_type = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_wm_window_type_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut wm_window_type,
                &mut e,
            );

            let atoms = slice::from_raw_parts(
                wm_window_type.atoms as *mut x::Atom,
                wm_window_type.atoms_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_atoms_reply_wipe(&mut wm_window_type);

            Ok(Self::Reply { raw, atoms })
        }
    }
}

impl xcb::Cookie for GetWmWindowTypeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetWmWindowTypeCookieUnchecked {
    type Reply = GetWmWindowTypeReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut wm_window_type = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw = &ffi::xcb_ewmh_get_wm_window_type_reply(
                ewmh.ewmh.get(),
                cookie,
                &mut wm_window_type,
                &mut e,
            );

            let atoms = slice::from_raw_parts(
                wm_window_type.atoms as *mut x::Atom,
                wm_window_type.atoms_len as usize,
            )
            .to_vec();

            Ok(Some(Self::Reply { raw, atoms }))
        }
    }
}

pub struct GetWmWindowType {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetWmWindowType {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_wm_window_type(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            } else {
                ffi::xcb_ewmh_get_wm_window_type_unchecked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetWmWindowType {
    type Cookie = GetWmWindowTypeCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetWmWindowType {
    type Reply = GetWmWindowTypeReply;
    type Cookie = GetWmWindowTypeCookie;
    type CookieUnchecked = GetWmWindowTypeCookieUnchecked;
}
