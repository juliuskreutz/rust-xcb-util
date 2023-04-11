use std::{ffi, fmt, ptr};

use xcb::{x, Xid, XidNew};
use xcb_util_sys::*;

pub enum Cursor {
    XCursor,
    Arrow,
    BaseArrowDown,
    BasedArrowUp,
    Boat,
    Bogosity,
    BottomLeftCorner,
    BottomRightCorner,
    BottomSide,
    BottomTee,
    BoxSpiral,
    CenterPtr,
    Circle,
    Clock,
    CoffeeMug,
    Cross,
    CrossReverse,
    Crosshair,
    DiamongCross,
    Dot,
    Dotbox,
    DoubleArrow,
    DraftLarge,
    DrawftSmall,
    DrapedBox,
    Exchange,
    Fleur,
    Gobbler,
    Gumby,
    Hand1,
    Hand2,
    Heart,
    Icon,
    IronCross,
    LeftPtr,
    LeftSide,
    LeftTee,
    Leftbutton,
    LlAngle,
    LrAngle,
    Man,
    Middlebutton,
    Mouse,
    Pencil,
    Pirate,
    Plus,
    QuestionArrow,
    RightPtr,
    RightSide,
    RightTee,
    Rightbutton,
    RtlLogo,
    Sailboat,
    SbDownArrow,
    SbHDoubleArrow,
    SbLeftArrow,
    SbRightArrow,
    SbUpArrow,
    SbVDoubleArrow,
    Shuttle,
    Sizing,
    Spider,
    Spraycan,
    Star,
    Target,
    Tcross,
    TopLeftArrow,
    TopLeftCorner,
    TopRightCorner,
    TopSide,
    TopTee,
    Trek,
    UlAngle,
    Umbrella,
    UrAngle,
    Watch,
    Xterm,
    Custom(String),
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cursor::XCursor => "X_cursor",
            Cursor::Arrow => "arrow",
            Cursor::BaseArrowDown => "based_arrow_down",
            Cursor::BasedArrowUp => "based_arrow_up",
            Cursor::Boat => "boat",
            Cursor::Bogosity => "bogosity",
            Cursor::BottomLeftCorner => "bottom_left_corner",
            Cursor::BottomRightCorner => "bottom_right_corner",
            Cursor::BottomSide => "bottom_side",
            Cursor::BottomTee => "bottom_tee",
            Cursor::BoxSpiral => "box_spiral",
            Cursor::CenterPtr => "center_ptr",
            Cursor::Circle => "circle",
            Cursor::Clock => "clock",
            Cursor::CoffeeMug => "coffee_mug",
            Cursor::Cross => "cross",
            Cursor::CrossReverse => "cross_reverse",
            Cursor::Crosshair => "crosshair",
            Cursor::DiamongCross => "diamond_cross",
            Cursor::Dot => "dot",
            Cursor::Dotbox => "dotbox",
            Cursor::DoubleArrow => "double_arrow",
            Cursor::DraftLarge => "draft_large",
            Cursor::DrawftSmall => "draft_small",
            Cursor::DrapedBox => "draped_box",
            Cursor::Exchange => "exchange",
            Cursor::Fleur => "fleur",
            Cursor::Gobbler => "gobbler",
            Cursor::Gumby => "gumby",
            Cursor::Hand1 => "hand1",
            Cursor::Hand2 => "hand2",
            Cursor::Heart => "heart",
            Cursor::Icon => "icon",
            Cursor::IronCross => "iron_cross",
            Cursor::LeftPtr => "left_ptr",
            Cursor::LeftSide => "left_side",
            Cursor::LeftTee => "left_tee",
            Cursor::Leftbutton => "leftbutton",
            Cursor::LlAngle => "ll_angle",
            Cursor::LrAngle => "lr_angle",
            Cursor::Man => "man",
            Cursor::Middlebutton => "middlebutton",
            Cursor::Mouse => "mouse",
            Cursor::Pencil => "pencil",
            Cursor::Pirate => "pirate",
            Cursor::Plus => "plus",
            Cursor::QuestionArrow => "question_arrow",
            Cursor::RightPtr => "right_ptr",
            Cursor::RightSide => "right_side",
            Cursor::RightTee => "right_tee",
            Cursor::Rightbutton => "rightbutton",
            Cursor::RtlLogo => "rtl_logo",
            Cursor::Sailboat => "sailboat",
            Cursor::SbDownArrow => "sb_down_arrow",
            Cursor::SbHDoubleArrow => "sb_h_double_arrow",
            Cursor::SbLeftArrow => "sb_left_arrow",
            Cursor::SbRightArrow => "sb_right_arrow",
            Cursor::SbUpArrow => "sb_up_arrow",
            Cursor::SbVDoubleArrow => "sb_v_double_arrow",
            Cursor::Shuttle => "shuttle",
            Cursor::Sizing => "sizing",
            Cursor::Spider => "spider",
            Cursor::Spraycan => "spraycan",
            Cursor::Star => "star",
            Cursor::Target => "target",
            Cursor::Tcross => "tcross",
            Cursor::TopLeftArrow => "top_left_arrow",
            Cursor::TopLeftCorner => "top_left_corner",
            Cursor::TopRightCorner => "top_right_corner",
            Cursor::TopSide => "top_side",
            Cursor::TopTee => "top_tee",
            Cursor::Trek => "trek",
            Cursor::UlAngle => "ul_angle",
            Cursor::Umbrella => "umbrella",
            Cursor::UrAngle => "ur_angle",
            Cursor::Watch => "watch",
            Cursor::Xterm => "xterm",
            Cursor::Custom(s) => s,
        };

        write!(f, "{s}")
    }
}

pub struct CursorContext {
    inner: ptr::NonNull<xcb_cursor_context_t>,
}

impl CursorContext {
    pub fn new(connection: &xcb::Connection, screen: &x::Screen) -> Option<Self> {
        let mut screen = xcb_screen_t {
            root: screen.root().resource_id(),
            default_colormap: screen.default_colormap().resource_id(),
            white_pixel: screen.white_pixel(),
            black_pixel: screen.black_pixel(),
            current_input_masks: screen.current_input_masks().bits(),
            width_in_pixels: screen.width_in_pixels(),
            height_in_pixels: screen.height_in_pixels(),
            width_in_millimeters: screen.width_in_millimeters(),
            height_in_millimeters: screen.height_in_millimeters(),
            min_installed_maps: screen.min_installed_maps(),
            max_installed_maps: screen.max_installed_maps(),
            root_visual: screen.root_visual(),
            backing_stores: screen.backing_stores() as u8,
            save_unders: screen.save_unders() as u8,
            root_depth: screen.root_depth(),
            allowed_depths_len: screen.allowed_depths().count() as u8,
        };

        let mut ctx = ptr::null_mut();

        unsafe {
            xcb_cursor_context_new(
                connection.get_raw_conn() as *mut xcb_connection_t,
                &mut screen,
                &mut ctx,
            )
        };

        ptr::NonNull::new(ctx).map(|ctx| Self { inner: ctx })
    }

    pub fn load_cursor(&self, cursor: Cursor) -> x::Cursor {
        let c_str = ffi::CString::new(cursor.to_string()).unwrap();

        unsafe {
            let cursor = xcb_cursor_load_cursor(self.inner.as_ptr(), c_str.as_ptr());
            x::Cursor::new(cursor)
        }
    }
}

impl Drop for CursorContext {
    fn drop(&mut self) {
        unsafe {
            xcb_cursor_context_free(self.inner.as_ptr());
        }
    }
}
