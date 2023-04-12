use std::{
    ffi::c_int,
    mem,
    ops::{Deref, DerefMut},
    ptr, slice, str,
};

use util::utf8;
use xcb::{x, Cookie, Xid, XidNew};
use xcb_util_sys::ewmh::*;

#[repr(transparent)]
pub struct Coordinates(xcb_ewmh_coordinates_t);

impl Coordinates {
    pub fn x(&self) -> u32 {
        self.0.x
    }

    pub fn y(&self) -> u32 {
        self.0.y
    }
}

#[repr(transparent)]
pub struct Geometry(xcb_ewmh_geometry_t);

impl Geometry {
    pub fn x(&self) -> u32 {
        self.0.x
    }

    pub fn y(&self) -> u32 {
        self.0.y
    }

    pub fn width(&self) -> u32 {
        self.0.width
    }

    pub fn height(&self) -> u32 {
        self.0.height
    }
}

#[repr(transparent)]
pub struct StrutPartial(xcb_ewmh_wm_strut_partial_t);

impl StrutPartial {
    pub fn left(&self) -> u32 {
        self.0.left
    }

    pub fn right(&self) -> u32 {
        self.0.right
    }

    pub fn top(&self) -> u32 {
        self.0.top
    }

    pub fn bottom(&self) -> u32 {
        self.0.bottom
    }

    pub fn left_start_y(&self) -> u32 {
        self.0.left_start_y
    }

    pub fn left_end_y(&self) -> u32 {
        self.0.left_end_y
    }

    pub fn right_start_y(&self) -> u32 {
        self.0.right_start_y
    }

    pub fn right_end_y(&self) -> u32 {
        self.0.right_end_y
    }

    pub fn top_start_x(&self) -> u32 {
        self.0.top_start_x
    }

    pub fn top_end_x(&self) -> u32 {
        self.0.top_end_x
    }

    pub fn bottom_start_x(&self) -> u32 {
        self.0.bottom_start_x
    }

    pub fn bottom_end_x(&self) -> u32 {
        self.0.bottom_end_x
    }
}

#[repr(transparent)]
pub struct Extents(xcb_ewmh_get_extents_reply_t);

impl Extents {
    pub fn top(&self) -> u32 {
        self.0.top
    }

    pub fn bottom(&self) -> u32 {
        self.0.bottom
    }

    pub fn left(&self) -> u32 {
        self.0.left
    }

    pub fn right(&self) -> u32 {
        self.0.right
    }
}

pub struct WmIcon {
    width: u32,
    height: u32,
    id: u32,
}

impl WmIcon {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[repr(transparent)]
pub struct WmIconIterator(xcb_ewmh_wm_icon_iterator_t);

impl Iterator for WmIconIterator {
    type Item = WmIcon;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.0.rem == 0 {
                None
            } else {
                let width = self.0.width;
                let height = self.0.height;
                let id = *self.0.data;

                xcb_ewmh_get_wm_icon_next(&mut self.0);

                Some(WmIcon { width, height, id })
            }
        }
    }
}

#[repr(transparent)]
pub struct WmFullScreenMonitors(xcb_ewmh_get_wm_fullscreen_monitors_reply_t);

impl WmFullScreenMonitors {
    pub fn top(&self) -> u32 {
        self.0.top
    }

    pub fn bottom(&self) -> u32 {
        self.0.bottom
    }

    pub fn left(&self) -> u32 {
        self.0.left
    }

    pub fn right(&self) -> u32 {
        self.0.right
    }
}

pub type ClientSourceType = xcb_ewmh_client_source_type_t;
pub const CLIENT_SOURCE_TYPE_NONE: ClientSourceType = 0;
pub const CLIENT_SOURCE_TYPE_NORMAL: ClientSourceType = 1;
pub const CLIENT_SOURCE_TYPE_OTHER: ClientSourceType = 2;

pub type DesktopLayoutOrientation = xcb_ewmh_desktop_layout_orientation_t;
pub const ORIENTATION_HORZ: DesktopLayoutOrientation = 0;
pub const ORIENTATION_VERT: DesktopLayoutOrientation = 1;

pub type DesktopLayoutStartingCorner = xcb_ewmh_desktop_layout_starting_corner_t;
pub const TOP_LEFT: DesktopLayoutStartingCorner = 0;
pub const TOP_RIGHT: DesktopLayoutStartingCorner = 1;
pub const BOTTOM_RIGHT: DesktopLayoutStartingCorner = 2;
pub const BOTTOM_LEFT: DesktopLayoutStartingCorner = 3;

pub type MoveResizeWindowFlags = xcb_ewmh_moveresize_window_opt_flags_t;
pub const MOVE_RESIZE_WINDOW_X: MoveResizeWindowFlags = 1 << 8;
pub const MOVE_RESIZE_WINDOW_Y: MoveResizeWindowFlags = 1 << 9;
pub const MOVE_RESIZE_WINDOW_WIDTH: MoveResizeWindowFlags = 1 << 10;
pub const MOVE_RESIZE_WINDOW_HEIGHT: MoveResizeWindowFlags = 1 << 11;

pub type MoveResizeDirection = xcb_ewmh_moveresize_direction_t;
pub const MOVE_RESIZE_SIZE_TOPLEFT: MoveResizeDirection = 0;
pub const MOVE_RESIZE_SIZE_TOP: MoveResizeDirection = 1;
pub const MOVE_RESIZE_SIZE_TOPRIGHT: MoveResizeDirection = 2;
pub const MOVE_RESIZE_SIZE_RIGHT: MoveResizeDirection = 3;
pub const MOVE_RESIZE_SIZE_BOTTOMRIGHT: MoveResizeDirection = 4;
pub const MOVE_RESIZE_SIZE_BOTTOM: MoveResizeDirection = 5;
pub const MOVE_RESIZE_SIZE_BOTTOMLEFT: MoveResizeDirection = 6;
pub const MOVE_RESIZE_SIZE_LEFT: MoveResizeDirection = 7;
pub const MOVE_RESIZE_MOVE: MoveResizeDirection = 8;
pub const MOVE_RESIZE_SIZE_KEYBOARD: MoveResizeDirection = 9;
pub const MOVE_RESIZE_MOVE_KEYBOARD: MoveResizeDirection = 10;
pub const MOVE_RESIZE_CANCEL: MoveResizeDirection = 11;

pub type StateAction = xcb_ewmh_wm_state_action_t;
pub const STATE_REMOVE: StateAction = 0;
pub const STATE_ADD: StateAction = 1;
pub const STATE_TOGGLE: StateAction = 2;

pub struct Connection {
    xcb: xcb::Connection,
    ewmh: xcb_ewmh_connection_t,
}

#[cfg(feature = "thread")]
unsafe impl Send for Connection {}
#[cfg(feature = "thread")]
unsafe impl Sync for Connection {}

impl Connection {
    pub fn connect(xcb: xcb::Connection) -> Option<Connection> {
        unsafe {
            let mut ewmh = mem::zeroed();
            let mut err = ptr::null_mut();

            let cookie =
                xcb_ewmh_init_atoms(xcb.get_raw_conn() as *mut xcb_connection_t, &mut ewmh);
            xcb_ewmh_init_atoms_replies(&mut ewmh, cookie, &mut err);

            if err.is_null() {
                Some(Connection { xcb, ewmh })
            } else {
                None
            }
        }
    }

    #[inline(always)]
    pub fn get_raw_conn(&self) -> *mut xcb_ewmh_connection_t {
        &self.ewmh as *const _ as *mut _
    }

    #[inline(always)]
    pub fn WM_CM(&self) -> &[x::Atom] {
        unsafe {
            slice::from_raw_parts(
                &x::Atom::new(*self.ewmh._NET_WM_CM_Sn),
                self.ewmh.nb_screens as usize,
            )
        }
    }

    #[inline(always)]
    pub fn SUPPORTED(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_SUPPORTED) }
    }

    #[inline(always)]
    pub fn CLIENT_LIST(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_CLIENT_LIST) }
    }

    #[inline(always)]
    pub fn CLIENT_LIST_STACKING(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_CLIENT_LIST_STACKING) }
    }

    #[inline(always)]
    pub fn NUMBER_OF_DESKTOPS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_NUMBER_OF_DESKTOPS) }
    }

    #[inline(always)]
    pub fn DESKTOP_GEOMETRY(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_DESKTOP_GEOMETRY) }
    }

    #[inline(always)]
    pub fn DESKTOP_VIEWPORT(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_DESKTOP_VIEWPORT) }
    }

    #[inline(always)]
    pub fn CURRENT_DESKTOP(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_CURRENT_DESKTOP) }
    }

    #[inline(always)]
    pub fn DESKTOP_NAMES(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_DESKTOP_NAMES) }
    }

    #[inline(always)]
    pub fn ACTIVE_WINDOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_ACTIVE_WINDOW) }
    }

    #[inline(always)]
    pub fn WORKAREA(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WORKAREA) }
    }

    #[inline(always)]
    pub fn SUPPORTING_WM_CHECK(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_SUPPORTING_WM_CHECK) }
    }

    #[inline(always)]
    pub fn VIRTUAL_ROOTS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_VIRTUAL_ROOTS) }
    }

    #[inline(always)]
    pub fn DESKTOP_LAYOUT(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_DESKTOP_LAYOUT) }
    }

    #[inline(always)]
    pub fn SHOWING_DESKTOP(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_SHOWING_DESKTOP) }
    }

    #[inline(always)]
    pub fn CLOSE_WINDOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_CLOSE_WINDOW) }
    }

    #[inline(always)]
    pub fn MOVERESIZE_WINDOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_MOVERESIZE_WINDOW) }
    }

    #[inline(always)]
    pub fn WM_MOVERESIZE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_MOVERESIZE) }
    }

    #[inline(always)]
    pub fn RESTACK_WINDOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_RESTACK_WINDOW) }
    }

    #[inline(always)]
    pub fn REQUEST_FRAME_EXTENTS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_REQUEST_FRAME_EXTENTS) }
    }

    #[inline(always)]
    pub fn WM_NAME(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_NAME) }
    }

    #[inline(always)]
    pub fn WM_VISIBLE_NAME(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_VISIBLE_NAME) }
    }

    #[inline(always)]
    pub fn WM_ICON_NAME(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ICON_NAME) }
    }

    #[inline(always)]
    pub fn WM_VISIBLE_ICON_NAME(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_VISIBLE_ICON_NAME) }
    }

    #[inline(always)]
    pub fn WM_DESKTOP(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_DESKTOP) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE) }
    }

    #[inline(always)]
    pub fn WM_STATE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE) }
    }

    #[inline(always)]
    pub fn WM_ALLOWED_ACTIONS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ALLOWED_ACTIONS) }
    }

    #[inline(always)]
    pub fn WM_STRUT(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STRUT) }
    }

    #[inline(always)]
    pub fn WM_STRUT_PARTIAL(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STRUT_PARTIAL) }
    }

    #[inline(always)]
    pub fn WM_ICON_GEOMETRY(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ICON_GEOMETRY) }
    }

    #[inline(always)]
    pub fn WM_ICON(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ICON) }
    }

    #[inline(always)]
    pub fn WM_PID(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_PID) }
    }

    #[inline(always)]
    pub fn WM_HANDLED_ICONS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_HANDLED_ICONS) }
    }

    #[inline(always)]
    pub fn WM_USER_TIME(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_USER_TIME) }
    }

    #[inline(always)]
    pub fn WM_USER_TIME_WINDOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_USER_TIME_WINDOW) }
    }

    #[inline(always)]
    pub fn FRAME_EXTENTS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_FRAME_EXTENTS) }
    }

    #[inline(always)]
    pub fn WM_PING(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_PING) }
    }

    #[inline(always)]
    pub fn WM_SYNC_REQUEST(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_SYNC_REQUEST) }
    }

    #[inline(always)]
    pub fn WM_SYNC_REQUEST_COUNTER(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_SYNC_REQUEST_COUNTER) }
    }

    #[inline(always)]
    pub fn WM_FULLSCREEN_MONITORS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_FULLSCREEN_MONITORS) }
    }

    #[inline(always)]
    pub fn WM_FULL_PLACEMENT(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_FULL_PLACEMENT) }
    }

    #[inline(always)]
    pub fn WM_PROTOCOLS(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh.WM_PROTOCOLS) }
    }

    #[inline(always)]
    pub fn MANAGER(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh.MANAGER) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_DESKTOP(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_DESKTOP) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_DOCK(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_DOCK) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_TOOLBAR(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_TOOLBAR) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_MENU(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_MENU) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_UTILITY(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_UTILITY) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_SPLASH(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_SPLASH) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_DIALOG(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_DIALOG) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_DROPDOWN_MENU(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_DROPDOWN_MENU) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_POPUP_MENU(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_POPUP_MENU) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_TOOLTIP(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_TOOLTIP) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_NOTIFICATION(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_NOTIFICATION) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_COMBO(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_COMBO) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_DND(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_DND) }
    }

    #[inline(always)]
    pub fn WM_WINDOW_TYPE_NORMAL(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_WINDOW_TYPE_NORMAL) }
    }

    #[inline(always)]
    pub fn WM_STATE_MODAL(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_MODAL) }
    }

    #[inline(always)]
    pub fn WM_STATE_STICKY(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_STICKY) }
    }

    #[inline(always)]
    pub fn WM_STATE_MAXIMIZED_VERT(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_MAXIMIZED_VERT) }
    }

    #[inline(always)]
    pub fn WM_STATE_MAXIMIZED_HORZ(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_MAXIMIZED_HORZ) }
    }

    #[inline(always)]
    pub fn WM_STATE_SHADED(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_SHADED) }
    }

    #[inline(always)]
    pub fn WM_STATE_SKIP_TASKBAR(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_SKIP_TASKBAR) }
    }

    #[inline(always)]
    pub fn WM_STATE_SKIP_PAGER(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_SKIP_PAGER) }
    }

    #[inline(always)]
    pub fn WM_STATE_HIDDEN(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_HIDDEN) }
    }

    #[inline(always)]
    pub fn WM_STATE_FULLSCREEN(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_FULLSCREEN) }
    }

    #[inline(always)]
    pub fn WM_STATE_ABOVE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_ABOVE) }
    }

    #[inline(always)]
    pub fn WM_STATE_BELOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_BELOW) }
    }

    #[inline(always)]
    pub fn WM_STATE_DEMANDS_ATTENTION(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_STATE_DEMANDS_ATTENTION) }
    }

    #[inline(always)]
    pub fn WM_ACTION_MOVE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_MOVE) }
    }

    #[inline(always)]
    pub fn WM_ACTION_RESIZE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_RESIZE) }
    }

    #[inline(always)]
    pub fn WM_ACTION_MINIMIZE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_MINIMIZE) }
    }

    #[inline(always)]
    pub fn WM_ACTION_SHADE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_SHADE) }
    }

    #[inline(always)]
    pub fn WM_ACTION_STICK(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_STICK) }
    }

    #[inline(always)]
    pub fn WM_ACTION_MAXIMIZE_HORZ(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_MAXIMIZE_HORZ) }
    }

    #[inline(always)]
    pub fn WM_ACTION_MAXIMIZE_VERT(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_MAXIMIZE_VERT) }
    }

    #[inline(always)]
    pub fn WM_ACTION_FULLSCREEN(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_FULLSCREEN) }
    }

    #[inline(always)]
    pub fn WM_ACTION_CHANGE_DESKTOP(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_CHANGE_DESKTOP) }
    }

    #[inline(always)]
    pub fn WM_ACTION_CLOSE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_CLOSE) }
    }

    #[inline(always)]
    pub fn WM_ACTION_ABOVE(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_ABOVE) }
    }

    #[inline(always)]
    pub fn WM_ACTION_BELOW(&self) -> x::Atom {
        unsafe { x::Atom::new(self.ewmh._NET_WM_ACTION_BELOW) }
    }
}

impl Deref for Connection {
    type Target = xcb::Connection;

    fn deref(&self) -> &Self::Target {
        &self.xcb
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.xcb
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { xcb_ewmh_connection_wipe(&mut self.ewmh) }
    }
}

pub fn send_client_message(
    c: &xcb::Connection,
    window: x::Window,
    dest: x::Window,
    atom: x::Atom,
    data: &[u32],
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_send_client_message(
            c.get_raw_conn() as *mut xcb_connection_t,
            window.resource_id(),
            dest.resource_id(),
            atom.resource_id(),
            data.len() as u32,
            data.as_ptr()
        )
    )
}

pub fn request_close_window(
    c: &Connection,
    screen: i32,
    window: x::Window,
    timestamp: x::Timestamp,
    source_indication: ClientSourceType,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_close_window(
            c.get_raw_conn(),
            screen as c_int,
            window.resource_id(),
            timestamp,
            source_indication
        )
    )
}

#[allow(clippy::too_many_arguments)]
pub fn request_move_resize_window(
    c: &Connection,
    screen: i32,
    window: x::Window,
    gravity: x::Gravity,
    source_indication: ClientSourceType,
    flags: MoveResizeWindowFlags,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_moveresize_window(
            c.get_raw_conn(),
            screen as c_int,
            window.resource_id(),
            gravity as u32,
            source_indication,
            flags,
            x,
            y,
            width,
            height
        )
    )
}

pub fn send_wm_ping(c: &Connection, window: x::Window, timestamp: x::Timestamp) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_send_wm_ping(c.get_raw_conn(), window.resource_id(), timestamp)
    )
}

define!(cookie GetSupportedCookie through Connection with xcb_ewmh_get_supported_reply => GetSupportedReply);
define!(reply GetSupportedReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetSupportedReply {
    pub fn atoms(&self) -> &[x::Atom] {
        unsafe { slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize) }
    }
}

pub fn set_supported(c: &Connection, screen: i32, list: &[x::Atom]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_supported(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_supported_checked(
    c: &Connection,
    screen: i32,
    list: &[x::Atom],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_supported_checked(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn get_supported(c: &Connection, screen: i32) -> GetSupportedCookie {
    property!(checked GetSupportedCookie -> c,
		xcb_ewmh_get_supported(c.get_raw_conn(), screen as c_int))
}

pub fn get_supported_unchecked(c: &Connection, screen: i32) -> GetSupportedCookie {
    property!(unchecked GetSupportedCookie -> c,
		xcb_ewmh_get_supported_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetClientListCookie through Connection with xcb_ewmh_get_client_list_reply => GetClientListReply);
define!(reply GetClientListReply for xcb_ewmh_get_windows_reply_t with xcb_ewmh_get_windows_reply_wipe);

impl GetClientListReply {
    pub fn windows(&self) -> &[x::Window] {
        unsafe { slice::from_raw_parts(self.0.windows as *mut _, self.0.windows_len as usize) }
    }
}

pub fn set_client_list(c: &Connection, screen: i32, list: &[x::Window]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_client_list(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_client_list_checked(
    c: &Connection,
    screen: i32,
    list: &[x::Window],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_client_list_checked(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn get_client_list(c: &Connection, screen: i32) -> GetClientListCookie {
    property!(checked GetClientListCookie -> c,
		xcb_ewmh_get_client_list(c.get_raw_conn(), screen as c_int))
}

pub fn get_client_list_unchecked(c: &Connection, screen: i32) -> GetClientListCookie {
    property!(unchecked GetClientListCookie -> c,
		xcb_ewmh_get_client_list(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetClientListStackingCookie through Connection with xcb_ewmh_get_client_list_stacking_reply => GetClientListStackingReply);
define!(reply GetClientListStackingReply for xcb_ewmh_get_windows_reply_t with xcb_ewmh_get_windows_reply_wipe);

impl GetClientListStackingReply {
    pub fn windows(&self) -> &[x::Window] {
        unsafe { slice::from_raw_parts(self.0.windows as *mut _, self.0.windows_len as usize) }
    }
}

pub fn set_client_list_stacking(
    c: &Connection,
    screen: i32,
    list: &[x::Window],
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_client_list_stacking(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_client_list_stacking_checked(
    c: &Connection,
    screen: i32,
    list: &[x::Window],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_client_list_stacking_checked(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn get_client_list_stacking(c: &Connection, screen: i32) -> GetClientListStackingCookie {
    property!(checked GetClientListStackingCookie -> c,
		xcb_ewmh_get_client_list_stacking(c.get_raw_conn(), screen as c_int))
}

pub fn get_client_list_stacking_unchecked(
    c: &Connection,
    screen: i32,
) -> GetClientListStackingCookie {
    property!(unchecked GetClientListStackingCookie -> c,
		xcb_ewmh_get_client_list_stacking(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetNumberOfDesktopsCookie through Connection with xcb_ewmh_get_number_of_desktops_reply as u32);

pub fn set_number_of_desktops(c: &Connection, screen: i32, number: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_number_of_desktops(c.get_raw_conn(), screen as c_int, number)
    )
}

pub fn set_number_of_desktops_checked(
    c: &Connection,
    screen: i32,
    number: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_number_of_desktops_checked(c.get_raw_conn(), screen as c_int, number)
    )
}

pub fn get_number_of_desktops(c: &Connection, screen: i32) -> GetNumberOfDesktopsCookie {
    property!(checked GetNumberOfDesktopsCookie -> c,
		xcb_ewmh_get_number_of_desktops(c.get_raw_conn(), screen as c_int))
}

pub fn get_number_of_desktops_unchecked(c: &Connection, screen: i32) -> GetNumberOfDesktopsCookie {
    property!(unchecked GetNumberOfDesktopsCookie -> c,
		xcb_ewmh_get_number_of_desktops_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopGeometryCookie through Connection with xcb_ewmh_get_desktop_geometry_reply as (u32, u32));

pub fn set_desktop_geometry(
    c: &Connection,
    screen: i32,
    width: u32,
    height: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_desktop_geometry(c.get_raw_conn(), screen as c_int, width, height)
    )
}

pub fn set_desktop_geometry_checked(
    c: &Connection,
    screen: i32,
    width: u32,
    height: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_desktop_geometry_checked(c.get_raw_conn(), screen as c_int, width, height)
    )
}

pub fn request_change_desktop_geometry(
    c: &Connection,
    screen: i32,
    width: u32,
    height: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_change_desktop_geometry(c.get_raw_conn(), screen as c_int, width, height)
    )
}

pub fn get_desktop_geometry(c: &Connection, screen: i32) -> GetDesktopGeometryCookie {
    property!(checked GetDesktopGeometryCookie -> c,
		xcb_ewmh_get_desktop_geometry(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_geometry_unchecked(c: &Connection, screen: i32) -> GetDesktopGeometryCookie {
    property!(unchecked GetDesktopGeometryCookie -> c,
		xcb_ewmh_get_desktop_geometry_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopViewportCookie through Connection with xcb_ewmh_get_desktop_viewport_reply => GetDesktopViewportReply);
define!(reply GetDesktopViewportReply for xcb_ewmh_get_desktop_viewport_reply_t with xcb_ewmh_get_desktop_viewport_reply_wipe);

impl GetDesktopViewportReply {
    pub fn desktop_viewports(&self) -> &[Coordinates] {
        unsafe {
            slice::from_raw_parts(
                self.0.desktop_viewport as *mut _,
                self.0.desktop_viewport_len as usize,
            )
        }
    }
}

pub fn set_desktop_viewport(c: &Connection, screen: i32, list: &[Coordinates]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_desktop_viewport(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.as_ptr() as *mut _
        )
    )
}

pub fn set_desktop_viewport_checked(
    c: &Connection,
    screen: i32,
    list: &[Coordinates],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_desktop_viewport_checked(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.as_ptr() as *mut _
        )
    )
}

pub fn get_desktop_viewport(c: &Connection, screen: i32) -> GetDesktopViewportCookie {
    property!(checked GetDesktopViewportCookie -> c,
		xcb_ewmh_get_desktop_viewport(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_viewport_unchecked(c: &Connection, screen: i32) -> GetDesktopViewportCookie {
    property!(unchecked GetDesktopViewportCookie -> c,
		xcb_ewmh_get_desktop_viewport_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetCurrentDesktopCookie through Connection with xcb_ewmh_get_current_desktop_reply as u32);

pub fn set_current_desktop(c: &Connection, screen: i32, current_desktop: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_current_desktop(c.get_raw_conn(), screen as c_int, current_desktop)
    )
}

pub fn set_current_desktop_checked(
    c: &Connection,
    screen: i32,
    current_desktop: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_current_desktop_checked(c.get_raw_conn(), screen as c_int, current_desktop)
    )
}

pub fn request_change_current_desktop(
    c: &Connection,
    screen: i32,
    current_desktop: u32,
    timestamp: x::Timestamp,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_change_current_desktop(
            c.get_raw_conn(),
            screen as c_int,
            current_desktop,
            timestamp
        )
    )
}

pub fn get_current_desktop(c: &Connection, screen: i32) -> GetCurrentDesktopCookie {
    property!(checked GetCurrentDesktopCookie -> c,
		xcb_ewmh_get_current_desktop(c.get_raw_conn(), screen as c_int))
}

pub fn get_current_desktop_unchecked(c: &Connection, screen: i32) -> GetCurrentDesktopCookie {
    property!(unchecked GetCurrentDesktopCookie -> c,
		xcb_ewmh_get_current_desktop_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopNamesCookie through Connection with xcb_ewmh_get_desktop_names_reply => GetDesktopNamesReply);
define!(reply GetDesktopNamesReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

impl GetDesktopNamesReply {
    pub fn strings(&self) -> Vec<&str> {
        utf8::into(self.0.strings, self.0.strings_len)
    }
}

pub fn set_desktop_names<'a, T: IntoIterator<Item = &'a str>>(
    c: &Connection,
    screen: i32,
    list: T,
) -> xcb::VoidCookie {
    let value = utf8::from(list);

    void!(
        unchecked,
        xcb_ewmh_set_desktop_names(
            c.get_raw_conn(),
            screen,
            value.len() as u32,
            value.as_ptr() as *const _
        )
    )
}

pub fn set_desktop_names_checked<'a, T: IntoIterator<Item = &'a str>>(
    c: &Connection,
    screen: i32,
    list: T,
) -> xcb::VoidCookieChecked {
    let value = utf8::from(list);

    void!(
        checked,
        xcb_ewmh_set_desktop_names_checked(
            c.get_raw_conn(),
            screen,
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn get_desktop_names(c: &Connection, screen: i32) -> GetDesktopNamesCookie {
    property!(checked GetDesktopNamesCookie -> c,
		xcb_ewmh_get_desktop_names(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_names_unchecked(c: &Connection, screen: i32) -> GetDesktopNamesCookie {
    property!(unchecked GetDesktopNamesCookie -> c,
		xcb_ewmh_get_desktop_names_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetActiveWindowCookie through Connection with xcb_ewmh_get_active_window_reply as x::Window);

pub fn set_active_window(c: &Connection, screen: i32, window: x::Window) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_active_window(c.get_raw_conn(), screen as c_int, window.resource_id())
    )
}

pub fn set_active_window_checked(
    c: &Connection,
    screen: i32,
    window: x::Window,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_active_window_checked(c.get_raw_conn(), screen as c_int, window.resource_id())
    )
}

pub fn request_change_active_window(
    c: &Connection,
    screen: i32,
    window: x::Window,
    source_indication: ClientSourceType,
    timestamp: x::Timestamp,
    current: x::Window,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_change_active_window(
            c.get_raw_conn(),
            screen as c_int,
            window.resource_id(),
            source_indication,
            timestamp,
            current.resource_id()
        )
    )
}

pub fn get_active_window(c: &Connection, screen: i32) -> GetActiveWindowCookie {
    property!(checked GetActiveWindowCookie -> c,
		xcb_ewmh_get_active_window(c.get_raw_conn(), screen as c_int))
}

pub fn get_active_window_unchecked(c: &Connection, screen: i32) -> GetActiveWindowCookie {
    property!(unchecked GetActiveWindowCookie -> c,
		xcb_ewmh_get_active_window_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetWorkAreaCookie through Connection with xcb_ewmh_get_workarea_reply => GetWorkAreaReply);
define!(reply GetWorkAreaReply for xcb_ewmh_get_workarea_reply_t with xcb_ewmh_get_workarea_reply_wipe);

impl GetWorkAreaReply {
    pub fn work_area(&self) -> &[Geometry] {
        unsafe { slice::from_raw_parts(self.0.workarea as *const _, self.0.workarea_len as usize) }
    }
}

pub fn set_work_area(c: &Connection, screen: i32, list: &[Geometry]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_workarea(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.as_ptr() as *mut _
        )
    )
}

pub fn set_work_area_checked(
    c: &Connection,
    screen: i32,
    list: &[Geometry],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_workarea_checked(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.as_ptr() as *mut _
        )
    )
}

pub fn get_work_area(c: &Connection, screen: i32) -> GetWorkAreaCookie {
    property!(checked GetWorkAreaCookie -> c,
		xcb_ewmh_get_workarea(c.get_raw_conn(), screen as c_int))
}

pub fn get_work_area_unchecked(c: &Connection, screen: i32) -> GetWorkAreaCookie {
    property!(unchecked GetWorkAreaCookie -> c,
		xcb_ewmh_get_workarea_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetSupportingWmCheckCookie through Connection with xcb_ewmh_get_supporting_wm_check_reply as x::Window);

pub fn set_supporting_wm_check(
    c: &Connection,
    parent: x::Window,
    child: x::Window,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_supporting_wm_check(
            c.get_raw_conn(),
            parent.resource_id(),
            child.resource_id()
        )
    )
}

pub fn set_supporting_wm_check_checked(
    c: &Connection,
    parent: x::Window,
    child: x::Window,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_supporting_wm_check_checked(
            c.get_raw_conn(),
            parent.resource_id(),
            child.resource_id()
        )
    )
}

pub fn get_supporting_wm_check(c: &Connection, window: x::Window) -> GetSupportingWmCheckCookie {
    property!(checked GetSupportingWmCheckCookie -> c,
		xcb_ewmh_get_supporting_wm_check(c.get_raw_conn(), window.resource_id()))
}

pub fn get_supporting_wm_check_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetSupportingWmCheckCookie {
    property!(unchecked GetSupportingWmCheckCookie -> c,
		xcb_ewmh_get_supporting_wm_check_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetVirtualRootsCookie through Connection with xcb_ewmh_get_virtual_roots_reply => GetVirtualRootsReply);
define!(reply GetVirtualRootsReply for xcb_ewmh_get_windows_reply_t with xcb_ewmh_get_windows_reply_wipe);

pub fn set_virtual_roots(c: &Connection, screen: i32, list: &[x::Window]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_virtual_roots(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_virtual_roots_checked(
    c: &Connection,
    screen: i32,
    list: &[x::Window],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_virtual_roots_checked(
            c.get_raw_conn(),
            screen as c_int,
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn get_virtual_roots(c: &Connection, screen: i32) -> GetVirtualRootsCookie {
    property!(checked GetVirtualRootsCookie -> c,
		xcb_ewmh_get_virtual_roots(c.get_raw_conn(), screen as c_int))
}

pub fn get_virtual_roots_unchecked(c: &Connection, screen: i32) -> GetVirtualRootsCookie {
    property!(unchecked GetVirtualRootsCookie -> c,
		xcb_ewmh_get_virtual_roots_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopLayoutCookie through Connection with xcb_ewmh_get_desktop_layout_reply => GetDesktopLayoutReply);
define!(reply GetDesktopLayoutReply for xcb_ewmh_get_desktop_layout_reply_t);

impl GetDesktopLayoutReply {
    pub fn orientation(&self) -> DesktopLayoutOrientation {
        self.0.orientation
    }

    pub fn columns(&self) -> u32 {
        self.0.columns
    }

    pub fn rows(&self) -> u32 {
        self.0.rows
    }

    pub fn starting_corner(&self) -> DesktopLayoutStartingCorner {
        self.0.starting_corner
    }
}

pub fn set_desktop_layout(
    c: &Connection,
    screen: i32,
    orientation: DesktopLayoutOrientation,
    columns: u32,
    rows: u32,
    starting_corner: DesktopLayoutStartingCorner,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_desktop_layout(
            c.get_raw_conn(),
            screen as c_int,
            orientation,
            columns,
            rows,
            starting_corner
        )
    )
}

pub fn set_desktop_layout_checked(
    c: &Connection,
    screen: i32,
    orientation: DesktopLayoutOrientation,
    columns: u32,
    rows: u32,
    starting_corner: DesktopLayoutStartingCorner,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_desktop_layout_checked(
            c.get_raw_conn(),
            screen as c_int,
            orientation,
            columns,
            rows,
            starting_corner
        )
    )
}

pub fn get_desktop_layout(c: &Connection, screen: i32) -> GetDesktopLayoutCookie {
    property!(checked GetDesktopLayoutCookie -> c,
		xcb_ewmh_get_desktop_layout(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_layout_unchecked(c: &Connection, screen: i32) -> GetDesktopLayoutCookie {
    property!(unchecked GetDesktopLayoutCookie -> c,
		xcb_ewmh_get_desktop_layout_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetShowingDesktopCookie through Connection with xcb_ewmh_get_showing_desktop_reply as u32);

pub fn set_showing_desktop(c: &Connection, screen: i32, desktop: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_showing_desktop(c.get_raw_conn(), screen as c_int, desktop)
    )
}

pub fn set_showing_desktop_checked(
    c: &Connection,
    screen: i32,
    desktop: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_showing_desktop_checked(c.get_raw_conn(), screen as c_int, desktop)
    )
}

pub fn get_showing_desktop(c: &Connection, screen: i32) -> GetShowingDesktopCookie {
    property!(checked GetShowingDesktopCookie -> c,
		xcb_ewmh_get_showing_desktop(c.get_raw_conn(), screen as c_int))
}

pub fn get_showing_desktop_unchecked(c: &Connection, screen: i32) -> GetShowingDesktopCookie {
    property!(unchecked GetShowingDesktopCookie -> c,
		xcb_ewmh_get_showing_desktop_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetWmNameCookie through Connection with xcb_ewmh_get_wm_name_reply => GetWmNameReply);
define!(reply GetWmNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

impl GetWmNameReply {
    pub fn string(&self) -> &str {
        utf8::into(self.0.strings, self.0.strings_len)
            .first()
            .unwrap_or(&"")
    }
}

pub fn set_wm_name<T: AsRef<str>>(c: &Connection, window: x::Window, name: T) -> xcb::VoidCookie {
    let value = name.as_ref();

    void!(
        unchecked,
        xcb_ewmh_set_wm_name(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn set_wm_name_checked<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookieChecked {
    let value = name.as_ref();

    void!(
        checked,
        xcb_ewmh_set_wm_name_checked(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn get_wm_name(c: &Connection, window: x::Window) -> GetWmNameCookie {
    property!(checked GetWmNameCookie -> c,
		xcb_ewmh_get_wm_name(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_name_unchecked(c: &Connection, window: x::Window) -> GetWmNameCookie {
    property!(unchecked GetWmNameCookie -> c,
		xcb_ewmh_get_wm_name_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmVisibleNameCookie through Connection with xcb_ewmh_get_wm_visible_name_reply => GetWmVisibleNameReply);
define!(reply GetWmVisibleNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

pub fn set_wm_visible_name<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookie {
    let value = utf8::from(vec![name.as_ref()]);

    void!(
        unchecked,
        xcb_ewmh_set_wm_visible_name(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn set_wm_visible_name_checked<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookieChecked {
    let value = utf8::from(vec![name.as_ref()]);

    void!(
        checked,
        xcb_ewmh_set_wm_visible_name_checked(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn get_wm_visible_name(c: &Connection, window: x::Window) -> GetWmVisibleNameCookie {
    property!(checked GetWmVisibleNameCookie -> c,
		xcb_ewmh_get_wm_visible_name(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_visible_name_unchecked(c: &Connection, window: x::Window) -> GetWmVisibleNameCookie {
    property!(unchecked GetWmVisibleNameCookie -> c,
		xcb_ewmh_get_wm_visible_name_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmIconNameCookie through Connection with xcb_ewmh_get_wm_icon_name_reply => GetWmIconNameReply);
define!(reply GetWmIconNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

pub fn set_wm_icon_name<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookie {
    let value = utf8::from(vec![name.as_ref()]);

    void!(
        unchecked,
        xcb_ewmh_set_wm_icon_name(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn set_wm_icon_name_checked<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookieChecked {
    let value = utf8::from(vec![name.as_ref()]);

    void!(
        checked,
        xcb_ewmh_set_wm_icon_name_checked(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn get_wm_icon_name(c: &Connection, window: x::Window) -> GetWmIconNameCookie {
    property!(checked GetWmIconNameCookie -> c,
		xcb_ewmh_get_wm_icon_name(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_icon_name_unchecked(c: &Connection, window: x::Window) -> GetWmIconNameCookie {
    property!(unchecked GetWmIconNameCookie -> c,
		xcb_ewmh_get_wm_icon_name_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmVisibleIconNameCookie through Connection with xcb_ewmh_get_wm_visible_icon_name_reply => GetWmVisibleIconNameReply);
define!(reply GetWmVisibleIconNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

pub fn set_wm_visible_icon_name<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookie {
    let value = utf8::from(vec![name.as_ref()]);

    void!(
        unchecked,
        xcb_ewmh_set_wm_visible_icon_name(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn set_wm_visible_icon_name_checked<T: AsRef<str>>(
    c: &Connection,
    window: x::Window,
    name: T,
) -> xcb::VoidCookieChecked {
    let value = utf8::from(vec![name.as_ref()]);

    void!(
        checked,
        xcb_ewmh_set_wm_visible_icon_name_checked(
            c.get_raw_conn(),
            window.resource_id(),
            value.len() as u32,
            value.as_ptr() as *mut _
        )
    )
}

pub fn get_wm_visible_icon_name(c: &Connection, window: x::Window) -> GetWmVisibleIconNameCookie {
    property!(checked GetWmVisibleIconNameCookie -> c,
		xcb_ewmh_get_wm_visible_icon_name(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_visible_icon_name_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmVisibleIconNameCookie {
    property!(unchecked GetWmVisibleIconNameCookie -> c,
		xcb_ewmh_get_wm_visible_icon_name_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmDesktopCookie through Connection with xcb_ewmh_get_wm_desktop_reply as u32);

pub fn set_wm_desktop(c: &Connection, window: x::Window, number: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_desktop(c.get_raw_conn(), window.resource_id(), number)
    )
}

pub fn set_wm_desktop_checked(
    c: &Connection,
    window: x::Window,
    number: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_desktop_checked(c.get_raw_conn(), window.resource_id(), number)
    )
}

pub fn request_change_wm_desktop(
    c: &Connection,
    screen: i32,
    window: x::Window,
    desktop: u32,
    source_indication: ClientSourceType,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_change_wm_desktop(
            c.get_raw_conn(),
            screen as c_int,
            window.resource_id(),
            desktop,
            source_indication
        )
    )
}

pub fn get_wm_desktop(c: &Connection, window: x::Window) -> GetWmDesktopCookie {
    property!(checked GetWmDesktopCookie -> c,
		xcb_ewmh_get_wm_desktop(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_desktop_unchecked(c: &Connection, window: x::Window) -> GetWmDesktopCookie {
    property!(unchecked GetWmDesktopCookie -> c,
		xcb_ewmh_get_wm_desktop_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmWindowTypeCookie through Connection with xcb_ewmh_get_wm_window_type_reply => GetWmWindowTypeReply);
define!(reply GetWmWindowTypeReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetWmWindowTypeReply {
    pub fn atoms(&self) -> &[x::Atom] {
        unsafe { slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize) }
    }
}

pub fn set_wm_window_type(c: &Connection, window: x::Window, list: &[x::Atom]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_window_type(
            c.get_raw_conn(),
            window.resource_id(),
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_wm_window_type_checked(
    c: &Connection,
    window: x::Window,
    list: &[x::Atom],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_window_type_checked(
            c.get_raw_conn(),
            window.resource_id(),
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn get_wm_window_type(c: &Connection, window: x::Window) -> GetWmWindowTypeCookie {
    property!(checked GetWmWindowTypeCookie -> c,
		xcb_ewmh_get_wm_window_type(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_window_type_unchecked(c: &Connection, window: x::Window) -> GetWmWindowTypeCookie {
    property!(unchecked GetWmWindowTypeCookie -> c,
		xcb_ewmh_get_wm_window_type_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmStateCookie through Connection with xcb_ewmh_get_wm_state_reply => GetWmStateReply);
define!(reply GetWmStateReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetWmStateReply {
    pub fn atoms(&self) -> &[x::Atom] {
        unsafe { slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize) }
    }
}

pub fn set_wm_state(c: &Connection, window: x::Window, list: &[x::Atom]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_state(
            c.get_raw_conn(),
            window.resource_id(),
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_wm_state_checked(
    c: &Connection,
    window: x::Window,
    list: &[x::Atom],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_state_checked(
            c.get_raw_conn(),
            window.resource_id(),
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn request_change_wm_state(
    c: &Connection,
    screen: i32,
    window: x::Window,
    action: StateAction,
    first: x::Atom,
    second: x::Atom,
    source_indication: ClientSourceType,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_change_wm_state(
            c.get_raw_conn(),
            screen as c_int,
            window.resource_id(),
            action,
            first.resource_id(),
            second.resource_id(),
            source_indication
        )
    )
}

pub fn get_wm_state(c: &Connection, window: x::Window) -> GetWmStateCookie {
    property!(checked GetWmStateCookie -> c,
		xcb_ewmh_get_wm_state(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_state_unchecked(c: &Connection, window: x::Window) -> GetWmStateCookie {
    property!(unchecked GetWmStateCookie -> c,
		xcb_ewmh_get_wm_state_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmAllowedActionsCookie through Connection with xcb_ewmh_get_wm_allowed_actions_reply => GetWmAllowedActionsReply);
define!(reply GetWmAllowedActionsReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetWmAllowedActionsReply {
    pub fn atoms(&self) -> &[x::Atom] {
        unsafe { slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize) }
    }
}

pub fn set_wm_allowed_actions(
    c: &Connection,
    window: x::Window,
    list: &[x::Atom],
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_allowed_actions(
            c.get_raw_conn(),
            window.resource_id(),
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn set_wm_allowed_actions_checked(
    c: &Connection,
    window: x::Window,
    list: &[x::Atom],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_allowed_actions_checked(
            c.get_raw_conn(),
            window.resource_id(),
            list.len() as u32,
            list.iter()
                .map(|a| a.resource_id())
                .collect::<Vec<_>>()
                .as_mut_ptr()
        )
    )
}

pub fn get_wm_allowed_actions(c: &Connection, window: x::Window) -> GetWmAllowedActionsCookie {
    property!(checked GetWmAllowedActionsCookie -> c,
		xcb_ewmh_get_wm_allowed_actions(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_allowed_actions_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmAllowedActionsCookie {
    property!(unchecked GetWmAllowedActionsCookie -> c,
		xcb_ewmh_get_wm_allowed_actions_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmStrutCookie through Connection with xcb_ewmh_get_wm_strut_reply as Extents);

pub fn set_wm_strut(
    c: &Connection,
    window: x::Window,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_strut(
            c.get_raw_conn(),
            window.resource_id(),
            left,
            right,
            top,
            bottom
        )
    )
}

pub fn set_wm_strut_checked(
    c: &Connection,
    window: x::Window,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_strut_checked(
            c.get_raw_conn(),
            window.resource_id(),
            left,
            right,
            top,
            bottom
        )
    )
}

pub fn get_wm_strut(c: &Connection, window: x::Window) -> GetWmStrutCookie {
    property!(checked GetWmStrutCookie -> c,
		xcb_ewmh_get_wm_strut(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_strut_unchecked(c: &Connection, window: x::Window) -> GetWmStrutCookie {
    property!(unchecked GetWmStrutCookie -> c,
		xcb_ewmh_get_wm_strut(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmStrutPartialCookie through Connection with xcb_ewmh_get_wm_strut_partial_reply as StrutPartial);

pub fn set_wm_strut_partial(
    c: &Connection,
    window: x::Window,
    partial: StrutPartial,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_strut_partial(c.get_raw_conn(), window.resource_id(), partial.0)
    )
}

pub fn set_wm_strut_partial_checked(
    c: &Connection,
    window: x::Window,
    partial: StrutPartial,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_strut_partial_checked(c.get_raw_conn(), window.resource_id(), partial.0)
    )
}

pub fn get_wm_strut_partial(c: &Connection, window: x::Window) -> GetWmStrutPartialCookie {
    property!(checked GetWmStrutPartialCookie -> c,
		xcb_ewmh_get_wm_strut_partial(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_strut_partial_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmStrutPartialCookie {
    property!(unchecked GetWmStrutPartialCookie -> c,
		xcb_ewmh_get_wm_strut_partial(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmIconGeometryCookie through Connection with xcb_ewmh_get_wm_icon_geometry_reply as Geometry);

pub fn set_wm_icon_geometry(
    c: &Connection,
    window: x::Window,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_icon_geometry(
            c.get_raw_conn(),
            window.resource_id(),
            left,
            right,
            top,
            bottom
        )
    )
}

pub fn set_wm_icon_geometry_checked(
    c: &Connection,
    window: x::Window,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_icon_geometry_checked(
            c.get_raw_conn(),
            window.resource_id(),
            left,
            right,
            top,
            bottom
        )
    )
}

pub fn get_wm_icon_geometry(c: &Connection, window: x::Window) -> GetWmIconGeometryCookie {
    property!(checked GetWmIconGeometryCookie -> c,
		xcb_ewmh_get_wm_icon_geometry(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_icon_geometry_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmIconGeometryCookie {
    property!(unchecked GetWmIconGeometryCookie -> c,
		xcb_ewmh_get_wm_icon_geometry_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmIconCookie through Connection with xcb_ewmh_get_wm_icon_reply => GetWmIconReply);
define!(reply GetWmIconReply for xcb_ewmh_get_wm_icon_reply_t with xcb_ewmh_get_wm_icon_reply_wipe);

impl GetWmIconReply {
    pub fn len(&self) -> usize {
        unsafe { xcb_ewmh_get_wm_icon_length(&self.0) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn icons(&self) -> WmIconIterator {
        WmIconIterator(unsafe { xcb_ewmh_get_wm_icon_iterator(&self.0) })
    }
}

pub fn set_wm_icon(c: &Connection, mode: u8, window: x::Window, data: &[u32]) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_icon(
            c.get_raw_conn(),
            mode,
            window.resource_id(),
            data.len() as u32,
            data.as_ptr() as *mut _
        )
    )
}

pub fn set_wm_icon_checked(
    c: &Connection,
    mode: u8,
    window: x::Window,
    data: &[u32],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_icon_checked(
            c.get_raw_conn(),
            mode,
            window.resource_id(),
            data.len() as u32,
            data.as_ptr() as *mut _
        )
    )
}

pub fn append_wm_icon(
    c: &Connection,
    window: x::Window,
    width: u32,
    height: u32,
    img: &[u32],
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_append_wm_icon(
            c.get_raw_conn(),
            window.resource_id(),
            width,
            height,
            img.len() as u32,
            img.as_ptr() as *mut _
        )
    )
}

pub fn append_wm_icon_checked(
    c: &Connection,
    window: x::Window,
    width: u32,
    height: u32,
    img: &[u32],
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_append_wm_icon_checked(
            c.get_raw_conn(),
            window.resource_id(),
            width,
            height,
            img.len() as u32,
            img.as_ptr() as *mut _
        )
    )
}

pub fn get_wm_icon(c: &Connection, window: x::Window) -> GetWmIconCookie {
    property!(checked GetWmIconCookie -> c,
		xcb_ewmh_get_wm_icon(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_icon_unchecked(c: &Connection, window: x::Window) -> GetWmIconCookie {
    property!(unchecked GetWmIconCookie -> c,
		xcb_ewmh_get_wm_icon_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmPidCookie through Connection with xcb_ewmh_get_wm_pid_reply as u32);

pub fn set_wm_pid(c: &Connection, window: x::Window, pid: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_pid(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn set_wm_pid_checked(c: &Connection, window: x::Window, pid: u32) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_pid_checked(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn get_wm_pid(c: &Connection, window: x::Window) -> GetWmPidCookie {
    property!(checked GetWmPidCookie -> c,
		xcb_ewmh_get_wm_pid(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_pid_unchecked(c: &Connection, window: x::Window) -> GetWmPidCookie {
    property!(checked GetWmPidCookie -> c,
		xcb_ewmh_get_wm_pid_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmHandledIconsCookie through Connection with xcb_ewmh_get_wm_handled_icons_reply as u32);

pub fn set_wm_handled_icons(c: &Connection, window: x::Window, pid: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_handled_icons(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn set_wm_handled_icons_checked(
    c: &Connection,
    window: x::Window,
    pid: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_handled_icons_checked(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn get_wm_handled_icons(c: &Connection, window: x::Window) -> GetWmHandledIconsCookie {
    property!(checked GetWmHandledIconsCookie -> c,
		xcb_ewmh_get_wm_handled_icons(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_handled_icons_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmHandledIconsCookie {
    property!(checked GetWmHandledIconsCookie -> c,
		xcb_ewmh_get_wm_handled_icons_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmUserTimeCookie through Connection with xcb_ewmh_get_wm_user_time_reply as u32);

pub fn set_wm_user_time(c: &Connection, window: x::Window, pid: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_user_time(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn set_wm_user_time_checked(
    c: &Connection,
    window: x::Window,
    pid: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_user_time_checked(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn get_wm_user_time(c: &Connection, window: x::Window) -> GetWmUserTimeCookie {
    property!(checked GetWmUserTimeCookie -> c,
		xcb_ewmh_get_wm_user_time(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_user_time_unchecked(c: &Connection, window: x::Window) -> GetWmUserTimeCookie {
    property!(checked GetWmUserTimeCookie -> c,
		xcb_ewmh_get_wm_user_time_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmUserTimeWindowCookie through Connection with xcb_ewmh_get_wm_user_time_window_reply as u32);

pub fn set_wm_user_time_window(c: &Connection, window: x::Window, pid: u32) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_user_time_window(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn set_wm_user_time_window_checked(
    c: &Connection,
    window: x::Window,
    pid: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_user_time_window_checked(c.get_raw_conn(), window.resource_id(), pid)
    )
}

pub fn get_wm_user_time_window(c: &Connection, window: x::Window) -> GetWmUserTimeWindowCookie {
    property!(checked GetWmUserTimeWindowCookie -> c,
		xcb_ewmh_get_wm_user_time_window(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_user_time_window_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmUserTimeWindowCookie {
    property!(checked GetWmUserTimeWindowCookie -> c,
		xcb_ewmh_get_wm_user_time_window_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetFrameExtentsCookie through Connection with xcb_ewmh_get_frame_extents_reply as Extents);

pub fn set_frame_extents(
    c: &Connection,
    window: x::Window,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_frame_extents(
            c.get_raw_conn(),
            window.resource_id(),
            left,
            right,
            top,
            bottom
        )
    )
}

pub fn set_frame_extents_checked(
    c: &Connection,
    window: x::Window,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_frame_extents_checked(
            c.get_raw_conn(),
            window.resource_id(),
            left,
            right,
            top,
            bottom
        )
    )
}

pub fn get_frame_extents(c: &Connection, window: x::Window) -> GetFrameExtentsCookie {
    property!(checked GetFrameExtentsCookie -> c,
		xcb_ewmh_get_frame_extents(c.get_raw_conn(), window.resource_id()))
}

pub fn get_frame_extents_unchecked(c: &Connection, window: x::Window) -> GetFrameExtentsCookie {
    property!(unchecked GetFrameExtentsCookie -> c,
		xcb_ewmh_get_frame_extents_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmSyncRequestCounterCookie through Connection with xcb_ewmh_get_wm_sync_request_counter_reply as u64);

pub fn set_wm_sync_request_counter(
    c: &Connection,
    window: x::Window,
    atom: x::Atom,
    low: u32,
    high: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_sync_request_counter(
            c.get_raw_conn(),
            window.resource_id(),
            atom.resource_id(),
            low,
            high
        )
    )
}

pub fn set_wm_sync_request_counter_checked(
    c: &Connection,
    window: x::Window,
    atom: x::Atom,
    low: u32,
    high: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_sync_request_counter_checked(
            c.get_raw_conn(),
            window.resource_id(),
            atom.resource_id(),
            low,
            high
        )
    )
}

pub fn send_wm_sync_request(
    c: &Connection,
    window: x::Window,
    wm_protocols: x::Atom,
    wm_sync_request: x::Atom,
    timestamp: x::Timestamp,
    counter: u64,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_send_wm_sync_request(
            c.get_raw_conn(),
            window.resource_id(),
            wm_protocols.resource_id(),
            wm_sync_request.resource_id(),
            timestamp,
            counter
        )
    )
}

pub fn get_wm_sync_request_counter(
    c: &Connection,
    window: x::Window,
) -> GetWmSyncRequestCounterCookie {
    property!(checked GetWmSyncRequestCounterCookie -> c,
		xcb_ewmh_get_wm_sync_request_counter(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_sync_request_counter_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmSyncRequestCounterCookie {
    property!(unchecked GetWmSyncRequestCounterCookie -> c,
		xcb_ewmh_get_wm_sync_request_counter_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmFullScreenMonitorsCookie through Connection with xcb_ewmh_get_wm_fullscreen_monitors_reply as WmFullScreenMonitors);

pub fn set_wm_full_screen_monitors(
    c: &Connection,
    window: x::Window,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_fullscreen_monitors(
            c.get_raw_conn(),
            window.resource_id(),
            top,
            bottom,
            left,
            right
        )
    )
}

pub fn set_wm_full_screen_monitors_checked(
    c: &Connection,
    window: x::Window,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_fullscreen_monitors_checked(
            c.get_raw_conn(),
            window.resource_id(),
            top,
            bottom,
            left,
            right
        )
    )
}

#[allow(clippy::too_many_arguments)]
pub fn request_change_wm_full_screen_monitors(
    c: &Connection,
    screen: i32,
    window: x::Window,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    source_indication: ClientSourceType,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_request_change_wm_fullscreen_monitors(
            c.get_raw_conn(),
            screen as c_int,
            window.resource_id(),
            top,
            bottom,
            left,
            right,
            source_indication
        )
    )
}

pub fn get_wm_full_screen_monitors(
    c: &Connection,
    window: x::Window,
) -> GetWmFullScreenMonitorsCookie {
    property!(checked GetWmFullScreenMonitorsCookie -> c,
		xcb_ewmh_get_wm_fullscreen_monitors(c.get_raw_conn(), window.resource_id()))
}

pub fn get_wm_full_screen_monitors_unchecked(
    c: &Connection,
    window: x::Window,
) -> GetWmFullScreenMonitorsCookie {
    property!(unchecked GetWmFullScreenMonitorsCookie -> c,
		xcb_ewmh_get_wm_fullscreen_monitors_unchecked(c.get_raw_conn(), window.resource_id()))
}

define!(cookie GetWmCmOwnerCookie(xcb_get_selection_owner_cookie_t) through Connection with xcb_ewmh_get_wm_cm_owner_reply as x::Window);

pub fn set_wm_cm_owner(
    c: &Connection,
    screen: i32,
    owner: x::Window,
    timestamp: x::Timestamp,
    first: u32,
    second: u32,
) -> xcb::VoidCookie {
    void!(
        unchecked,
        xcb_ewmh_set_wm_cm_owner(
            c.get_raw_conn(),
            screen as c_int,
            owner.resource_id(),
            timestamp,
            first,
            second
        )
    )
}

pub fn set_wm_cm_owner_checked(
    c: &Connection,
    screen: i32,
    owner: x::Window,
    timestamp: x::Timestamp,
    first: u32,
    second: u32,
) -> xcb::VoidCookieChecked {
    void!(
        checked,
        xcb_ewmh_set_wm_cm_owner_checked(
            c.get_raw_conn(),
            screen as c_int,
            owner.resource_id(),
            timestamp,
            first,
            second
        )
    )
}

pub fn get_wm_cm_owner(c: &Connection, screen: i32) -> GetWmCmOwnerCookie {
    unsafe {
        GetWmCmOwnerCookie {
            conn: c,
            cookie: xcb_ewmh_get_wm_cm_owner(c.get_raw_conn(), screen as c_int),
            checked: true,
        }
    }
}

pub fn get_wm_cm_owner_unchecked(c: &Connection, screen: i32) -> GetWmCmOwnerCookie {
    unsafe {
        GetWmCmOwnerCookie {
            conn: c,
            cookie: xcb_ewmh_get_wm_cm_owner_unchecked(c.get_raw_conn(), screen as c_int),
            checked: false,
        }
    }
}
