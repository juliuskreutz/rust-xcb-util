#![allow(non_camel_case_types, non_snake_case)]

#[macro_use]
extern crate bitflags;
extern crate xcb;
extern crate xcb_util_sys;

pub mod ffi;

#[macro_use]
mod util;

#[cfg(feature = "icccm")]
pub mod icccm;

#[cfg(feature = "ewmh")]
pub mod ewmh;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "cursor")]
pub mod cursor;

#[cfg(feature = "keysyms")]
pub mod keysyms;

#[cfg(feature = "misc")]
pub mod misc;

#[cfg(feature = "render")]
pub mod render;
