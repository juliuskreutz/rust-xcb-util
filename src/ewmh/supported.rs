use std::{mem, ptr, slice};

use xcb::{x, Cookie};

use super::{
    ffi, EwmhConnection, EwmhCookieWithReplyChecked, EwmhCookieWithReplyUnchecked, EwmhReply,
    EwmhRequest, EwmhRequestWithReply, EwmhRequestWithoutReply, RawEwmhRequest,
};

pub struct SetSupported<'a> {
    pub screen_nbr: i32,
    pub atoms: &'a [x::Atom],
}

unsafe impl<'a> RawEwmhRequest for SetSupported<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_supported_checked(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.atoms.len() as u32,
                    self.atoms.as_ptr() as *mut ffi::xcb_atom_t,
                )
            } else {
                ffi::xcb_ewmh_set_supported(
                    ewmh.ewmh.get(),
                    self.screen_nbr,
                    self.atoms.len() as u32,
                    self.atoms.as_ptr() as *mut ffi::xcb_atom_t,
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

//TODO: Expose inner reply
pub struct GetSupportedReply {
    raw: *const u8,
    atoms: Vec<x::Atom>,
}

impl EwmhReply for GetSupportedReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut supported = mem::zeroed();

        ffi::xcb_ewmh_get_supported_from_reply(
            &mut supported,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let atoms = slice::from_raw_parts(
            supported.atoms as *mut x::Atom,
            supported.atoms_len as usize,
        )
        .to_vec();

        ffi::xcb_ewmh_get_atoms_reply_wipe(&mut supported);

        Self { raw, atoms }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetSupportedReply {
    pub fn atoms(&self) -> &[x::Atom] {
        &self.atoms
    }
}

//TODO: Expose inner cookie
pub struct GetSupportedCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetSupportedCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetSupportedCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetSupportedCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetSupportedCookie {
    type Reply = GetSupportedReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: self.sequence() as u32,
            };
            let mut supported = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_supported_reply(ewmh.ewmh.get(), cookie, &mut supported, &mut e);

            let atoms = slice::from_raw_parts(
                supported.atoms as *mut x::Atom,
                supported.atoms_len as usize,
            )
            .to_vec();

            ffi::xcb_ewmh_get_atoms_reply_wipe(&mut supported);

            Ok(Self::Reply { raw, atoms })
        }
    }
}

impl xcb::Cookie for GetSupportedCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetSupportedCookieUnchecked {
    type Reply = GetSupportedReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: self.sequence() as u32,
            };
            let mut supported = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_supported_reply(ewmh.ewmh.get(), cookie, &mut supported, &mut e);

            let atoms = slice::from_raw_parts(
                supported.atoms as *mut x::Atom,
                supported.atoms_len as usize,
            )
            .to_vec();

            Ok(Some(Self::Reply { raw, atoms }))
        }
    }
}

pub struct GetSupported {
    pub screen_nbr: i32,
}

unsafe impl RawEwmhRequest for GetSupported {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_supported(ewmh.ewmh.get(), self.screen_nbr)
            } else {
                ffi::xcb_ewmh_get_supported_unchecked(ewmh.ewmh.get(), self.screen_nbr)
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetSupported {
    type Cookie = GetSupportedCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetSupported {
    type Reply = GetSupportedReply;
    type Cookie = GetSupportedCookie;
    type CookieUnchecked = GetSupportedCookieUnchecked;
}
