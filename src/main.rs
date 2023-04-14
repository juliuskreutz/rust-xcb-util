extern crate xcb;
extern crate xcb_util;

use xcb_util::ewmh::{EwmhConnection, GetActiveWindow, GetClientList, GetSupported};

fn main() {
    let (c, _) = xcb::Connection::connect(None).unwrap();

    let e = EwmhConnection::new(&c).unwrap();

    let cookie = e.send_request(&GetClientList { screen_nbr: 0 });

    let reply = e.wait_for_reply(cookie).unwrap();

    println!("{:?}", reply.windows())
}
