#![allow(non_camel_case_types, non_snake_case)]

extern crate xcb;
extern crate libc;

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