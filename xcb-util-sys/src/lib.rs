#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "cursor")]
pub mod cursor {
    pub use xcb::ffi::xcb_connection_t;

    include!(concat!(env!("OUT_DIR"), "/cursor.rs"));
}

#[cfg(feature = "ewmh")]
pub mod ewmh {
    pub use xcb::ffi::xcb_connection_t;

    include!(concat!(env!("OUT_DIR"), "/ewmh.rs"));
}

#[cfg(feature = "icccm")]
pub mod icccm {
    pub use xcb::ffi::xcb_connection_t;

    include!(concat!(env!("OUT_DIR"), "/icccm.rs"));
}
