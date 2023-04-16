use std::{mem, ptr, slice};

use xcb::x;

use super::{
    ffi, ClientSourceType, EwmhConnection, EwmhCookieWithReplyChecked,
    EwmhCookieWithReplyUnchecked, EwmhReply, EwmhRequest, EwmhRequestWithReply,
    EwmhRequestWithoutReply, RawEwmhRequest, WmStateAction,
};

pub struct SetWmState<'a> {
    pub window: x::Window,
    pub atoms: &'a [x::Atom],
}

unsafe impl<'a> RawEwmhRequest for SetWmState<'a> {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_set_wm_state_checked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                    self.atoms.len() as u32,
                    self.atoms.as_ptr() as *mut ffi::xcb_atom_t,
                )
            } else {
                ffi::xcb_ewmh_set_wm_state(
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

impl<'a> EwmhRequest for SetWmState<'a> {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

impl<'a> EwmhRequestWithoutReply for SetWmState<'a> {}

pub struct RequestChangeWmState {
    pub screen_nbr: i32,
    pub window: x::Window,
    pub action: WmStateAction,
    pub first_property: x::Atom,
    pub second_property: x::Atom,
    pub source_indication: ClientSourceType,
}

unsafe impl RawEwmhRequest for RequestChangeWmState {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, _: bool) -> u64 {
        unsafe {
            ffi::xcb_ewmh_request_change_wm_state(
                ewmh.ewmh.get(),
                self.screen_nbr,
                xcb::Xid::resource_id(&self.window),
                self.action as u32,
                xcb::Xid::resource_id(&self.first_property),
                xcb::Xid::resource_id(&self.second_property),
                self.source_indication as u32,
            )
            .sequence as u64
        }
    }
}

impl EwmhRequest for RequestChangeWmState {
    type Cookie = xcb::VoidCookie;

    const IS_VOID: bool = true;
}

//TODO: Expose inner reply
pub struct GetWmStateReply {
    raw: *const u8,
    atoms: Vec<x::Atom>,
}

impl EwmhReply for GetWmStateReply {
    unsafe fn from_raw(raw: *const u8, _: *mut ffi::xcb_ewmh_connection_t) -> Self {
        let mut wm_state = mem::zeroed();

        ffi::xcb_ewmh_get_wm_state_from_reply(
            &mut wm_state,
            raw as *mut ffi::xcb_get_property_reply_t,
        );

        let atoms =
            slice::from_raw_parts(wm_state.atoms as *mut x::Atom, wm_state.atoms_len as usize)
                .to_vec();

        ffi::xcb_ewmh_get_atoms_reply_wipe(&mut wm_state);

        Self { raw, atoms }
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.raw
    }
}

impl GetWmStateReply {
    pub fn atoms(&self) -> &[x::Atom] {
        &self.atoms
    }
}

//TODO: Expose inner cookie
pub struct GetWmStateCookie(x::GetPropertyCookie);

//TODO: Expose inner cookie
pub struct GetWmStateCookieUnchecked(x::GetPropertyCookieUnchecked);

impl xcb::Cookie for GetWmStateCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookie::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl xcb::CookieChecked for GetWmStateCookie {}

unsafe impl EwmhCookieWithReplyChecked for GetWmStateCookie {
    type Reply = GetWmStateReply;

    fn wait_for_reply(self, ewmh: &EwmhConnection) -> xcb::Result<Self::Reply> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut wm_state = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_wm_state_reply(ewmh.ewmh.get(), cookie, &mut wm_state, &mut e);

            let atoms =
                slice::from_raw_parts(wm_state.atoms as *mut x::Atom, wm_state.atoms_len as usize)
                    .to_vec();

            ffi::xcb_ewmh_get_atoms_reply_wipe(&mut wm_state);

            Ok(Self::Reply { raw, atoms })
        }
    }
}

impl xcb::Cookie for GetWmStateCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        Self(x::GetPropertyCookieUnchecked::from_sequence(seq))
    }

    fn sequence(&self) -> u64 {
        self.0.sequence()
    }
}

unsafe impl EwmhCookieWithReplyUnchecked for GetWmStateCookieUnchecked {
    type Reply = GetWmStateReply;

    fn wait_for_reply_unchecked(
        self,
        ewmh: &EwmhConnection,
    ) -> xcb::ConnResult<Option<Self::Reply>> {
        unsafe {
            let cookie = ffi::xcb_get_property_cookie_t {
                sequence: xcb::Cookie::sequence(&self) as u32,
            };
            let mut wm_state = mem::zeroed();
            let mut e = ptr::null_mut();

            let raw =
                &ffi::xcb_ewmh_get_wm_state_reply(ewmh.ewmh.get(), cookie, &mut wm_state, &mut e);

            let atoms =
                slice::from_raw_parts(wm_state.atoms as *mut x::Atom, wm_state.atoms_len as usize)
                    .to_vec();

            Ok(Some(Self::Reply { raw, atoms }))
        }
    }
}

pub struct GetWmState {
    pub window: x::Window,
}

unsafe impl RawEwmhRequest for GetWmState {
    fn raw_ewmh_request(&self, ewmh: &EwmhConnection, checked: bool) -> u64 {
        unsafe {
            if checked {
                ffi::xcb_ewmh_get_wm_state(ewmh.ewmh.get(), xcb::Xid::resource_id(&self.window))
            } else {
                ffi::xcb_ewmh_get_wm_state_unchecked(
                    ewmh.ewmh.get(),
                    xcb::Xid::resource_id(&self.window),
                )
            }
            .sequence as u64
        }
    }
}

impl EwmhRequest for GetWmState {
    type Cookie = GetWmStateCookie;

    const IS_VOID: bool = false;
}

impl EwmhRequestWithReply for GetWmState {
    type Reply = GetWmStateReply;
    type Cookie = GetWmStateCookie;
    type CookieUnchecked = GetWmStateCookieUnchecked;
}
