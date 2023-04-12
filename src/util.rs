macro_rules! define {
    (cookie $cookie:ident for $inner:ident => $reply:ident) => {
        pub struct $cookie(
            x::GetPropertyCookie,
            unsafe extern "C" fn(
                *mut xcb_connection_t,
                xcb_get_property_cookie_t,
                *mut $inner,
                *mut *mut xcb_generic_error_t,
            ) -> u8,
        );

        #[cfg(feature = "thread")]
        unsafe impl Send for $cookie {}
        #[cfg(feature = "thread")]
        unsafe impl Sync for $cookie {}

        impl $cookie {
            pub fn get_reply(&self) -> Option<$reply> {
                unsafe {
                    if self.0.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res = self.1(
                            self.0.conn.get_raw_conn(),
                            self.0.cookie,
                            &mut reply,
                            &mut err,
                        );

                        if err.is_null() && res != 0 {
                            Some($reply(reply))
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        self.1(
                            self.0.conn.get_raw_conn(),
                            self.0.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some($reply(reply))
                    }
                }
            }
        }
    };

    (cookie $cookie:ident with $func:ident => $reply:ident) => {
        pub struct $cookie(x::GetPropertyCookie);

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl $cookie {
            pub fn get_reply(&self) -> Option<$reply> {
                unsafe {
                    if self.0.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res = $func(
                            self.0.conn.get_raw_conn(),
                            self.0.cookie,
                            &mut reply,
                            &mut err,
                        );

                        if err.is_null() && res != 0 {
                            Some($reply(reply))
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.0.conn.get_raw_conn(),
                            self.0.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some($reply(reply))
                    }
                }
            }
        }
    };

    (cookie $cookie:ident through $conn:ident with $func:ident => $reply:ident) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: xcb_get_property_cookie_t,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<$reply> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res =
                            $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

                        if err.is_null() && res != 0 {
                            Some($reply(reply))
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some($reply(reply))
                    }
                }
            }
        }
    };

    (cookie $cookie:ident through $conn:ident with $func:ident as ($first:path, $second:path)) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: xcb_get_property_cookie_t,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<($first, $second)> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut first = mem::zeroed();
                        let mut second = mem::zeroed();
                        let res = $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut first,
                            &mut second,
                            &mut err,
                        );

                        if err.is_null() && res != 0 {
                            Some((first, second))
                        } else {
                            None
                        }
                    } else {
                        let mut first = mem::zeroed();
                        let mut second = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut first,
                            &mut second,
                            ptr::null_mut(),
                        );

                        Some((first, second))
                    }
                }
            }
        }
    };

    (cookie $cookie:ident through $conn:ident with $func:ident as u32) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: xcb_get_property_cookie_t,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<u32> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res =
                            $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

                        if err.is_null() && res != 0 {
                            Some(reply)
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some(reply)
                    }
                }
            }
        }
    };

    (cookie $cookie:ident through $conn:ident with $func:ident as u64) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: xcb_get_property_cookie_t,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<u64> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res =
                            $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

                        if err.is_null() && res != 0 {
                            Some(reply)
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some(reply)
                    }
                }
            }
        }
    };

    (cookie $cookie:ident through $conn:ident with $func:ident as x::Window) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: xcb_get_property_cookie_t,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<x::Window> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res =
                            $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

                        if err.is_null() && res != 0 {
                            Some(x::Window::new(reply))
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some(x::Window::new(reply))
                    }
                }
            }
        }
    };

    (cookie $cookie:ident through $conn:ident with $func:ident as $reply:path) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: xcb_get_property_cookie_t,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<$reply> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res =
                            $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

                        if err.is_null() && res != 0 {
                            Some($reply(reply))
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some($reply(reply))
                    }
                }
            }
        }
    };

    (cookie $cookie:ident($inner:path) through $conn:ident with $func:ident as $reply:path) => {
        pub struct $cookie<'a> {
            conn: &'a $conn,
            cookie: $inner,
            checked: bool,
        }

        #[cfg(feature = "thread")]
        unsafe impl<'a> Send for $cookie<'a> {}
        #[cfg(feature = "thread")]
        unsafe impl<'a> Sync for $cookie<'a> {}

        impl<'a> $cookie<'a> {
            pub fn get_reply(&self) -> Option<$reply> {
                unsafe {
                    if self.checked {
                        let mut err: *mut xcb_generic_error_t = ptr::null_mut();
                        let mut reply = mem::zeroed();
                        let res =
                            $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

                        if err.is_null() && res != 0 {
                            Some(x::Window::new(reply))
                        } else {
                            None
                        }
                    } else {
                        let mut reply = mem::zeroed();
                        $func(
                            self.conn.get_raw_conn(),
                            self.cookie,
                            &mut reply,
                            ptr::null_mut(),
                        );

                        Some(x::Window::new(reply))
                    }
                }
            }
        }
    };

    (reply $reply:ident for $inner:ident with $wipe:ident) => {
        pub struct $reply($inner);

        impl Drop for $reply {
            fn drop(&mut self) {
                unsafe {
                    $wipe(&mut self.0);
                }
            }
        }
    };

    (reply $reply:ident for $inner:ident) => {
        pub struct $reply($inner);
    };
}

macro_rules! void {
    (checked, $cookie:expr) => {
        unsafe { xcb::VoidCookieChecked::from_sequence($cookie.sequence as u64) }
    };

    (unchecked, $cookie:expr) => {
        unsafe { xcb::VoidCookie::from_sequence($cookie.sequence as u64) }
    };
}

macro_rules! property {
    (checked $name:ident -> $conn:expr, $cookie:expr) => {
        unsafe {
            $name {
                conn: $conn,
                cookie: $cookie,
                checked: true,
            }
        }
    };

    (unchecked $name:ident -> $conn:expr, $cookie:expr) => {
        unsafe {
            $name {
                conn: $conn,
                cookie: $cookie,
                checked: false,
            }
        }
    };
}

pub mod utf8 {
    use std::ffi::c_char;
    use std::slice;
    use std::str;

    pub fn into<'a>(data: *const c_char, length: u32) -> Vec<&'a str> {
        if length == 0 {
            return Vec::new();
        }

        unsafe {
            let mut result =
                str::from_utf8_unchecked(slice::from_raw_parts(data as *mut u8, length as usize))
                    .split('\0')
                    .collect::<Vec<_>>();

            // Data is sometimes NULL-terminated and sometimes not. If there is a
            // NULL terminator, then our call to .split() will result in an extra
            // empty-string element at the end, so pop it.
            if let Some(&"") = result.last() {
                result.pop();
            }

            result
        }
    }

    pub fn from<'a, T: IntoIterator<Item = &'a str>>(data: T) -> Vec<u8> {
        let mut result = String::new();

        for item in data {
            result.push_str(item);
            result.push('\0');
        }

        result.into_bytes()
    }
}
