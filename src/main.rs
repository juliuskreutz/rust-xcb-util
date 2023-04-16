extern crate xcb;
extern crate xcb_util;

use xcb::{x, XidNew};
use xcb_util::ewmh::{
    ClientSourceType, EwmhConnection, GetActiveWindow, GetClientList, GetDesktopNames,
    GetSupported, GetWmName, GetWmVisibleName, RequestCloseWindow, SetDesktopNames, SetWmName,
    SetWmVisibleName,
};

fn main() {
    let (c, _) = xcb::Connection::connect(None).unwrap();

    let e = EwmhConnection::new(&c).unwrap();

    let strings = &vec!["Hello", "World"];

    e.send_request(&SetWmVisibleName {
        window: unsafe { x::Window::new(0x6f8) },
        strings,
    });

    let cookie = e.send_request(&GetWmVisibleName {
        window: unsafe { x::Window::new(0x6f8) },
    });
    let reply = e.wait_for_reply(cookie).unwrap();

    println!("{:?}", reply.strings());
}
