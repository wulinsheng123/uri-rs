use super::uri::Uri;
use std::{fmt, slice};

#[derive(Clone, Copy)]
pub struct Authority<'a>(&'a Uri<'a>);

impl<'a> Authority<'a> {
    fn start(&self) -> u32 {
        self.0.scheme_end.map(|x| x.get()).unwrap_or(0) + 2
    }

    fn host_bounds(&self) -> (u32, u32) {
        let host = unsafe { self.0.host.as_ref().unwrap_unchecked() };
        (host.0.get(), host.1)
    }

    fn userinfo(&self) -> Option<&str> {
        let start = self.start();
        let host_start = self.host_bounds().0;
        (start != host_start).then(|| unsafe { self.0.slice(start, host_start - 1) })
    }

    pub fn host_raw(&self) -> &str {
        let bounds = self.host_bounds();
        unsafe { self.0.slice(bounds.0, bounds.1) }
    }
    pub fn port_raw(&self) -> Option<&str> {
        let host_end = self.host_bounds().1;
        (host_end != self.0.path.0).then(|| unsafe { self.0.slice(host_end + 1, self.0.path.0) })
    }
}

pub fn authority<'a>(uri: &'a Uri<'a>) -> Option<Authority<'a>> {
    Some(Authority(uri))
}

impl<'a> fmt::Debug for Authority<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Authority")
            .field("userinfo", &self.userinfo())
            .field("host", &self.host_raw())
            .field("port", &self.port_raw())
            .finish()
    }
}
