use std::{fmt, marker::PhantomData, num::NonZeroU32, slice};

use super::{
    authority::{authority, Authority},
    path::Path,
};

#[derive(Clone)]
pub struct Uri<'a> {
    pub ptr: *const u8,
    pub len: u32,
    pub scheme_end: Option<NonZeroU32>,
    pub host: Option<(NonZeroU32, u32, HostInternal)>,
    pub path: (u32, u32),
    pub query_end: Option<NonZeroU32>,
    pub fragment_start: Option<NonZeroU32>,
    pub _marker: PhantomData<&'a [u8]>,
}

pub struct Scheme<'a>(&'a str);

impl<'a> Scheme<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a> Uri<'a> {
    fn scheme(&self) -> Option<Scheme<'_>> {
        self.scheme_end
            .map(|i| Scheme(unsafe { self.slice(0, i.get() - 1) }))
    }
    pub unsafe fn slice(&self, start: u32, end: u32) -> &'a str {
        let bytes =
            unsafe { slice::from_raw_parts(self.ptr.add(start as usize), (end - start) as usize) };

        unsafe { core::str::from_utf8_unchecked(bytes) }
    }

    fn path(&self) -> Path<'_> {
        Path(unsafe { self.slice(self.path.0, self.path.1) })
    }

    #[inline]
    pub fn authority(&self) -> Option<Authority<'_>> {
        if self.host.is_some() {
            authority(self)
        } else {
            None
        }
    }
    pub fn query(&self) -> Option<&str> {
        self.query_end
            .map(|i| unsafe { self.slice(self.path.1 + 1, i.get()) })
    }
}

impl<'a> fmt::Debug for Uri<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Uri")
            .field("scheme", &self.scheme().map(|s| s.as_str()))
            .field("authority", &self.authority())
            .field("path", &self.path().as_str())
            .field("query", &self.query())
            .finish()
    }
}

impl<'a> Default for Uri<'a> {
    fn default() -> Self {
        Uri {
            ptr: "".as_ptr(),
            len: 0,
            scheme_end: None,
            path: (0, 0),
            query_end: None,
            host: None,
            fragment_start: None,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub enum HostInternal {
    RegName,
}

#[test]
fn name() {
    let m = "asdfghjk";
    let c = m.as_ptr();
    let m = unsafe { c.add(2) };
    let l = unsafe { slice::from_raw_parts(m, 3) };
    let c = unsafe { core::str::from_utf8(l) };
    println!("{:?}", c);
}
