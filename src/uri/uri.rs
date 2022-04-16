use std::{fmt, marker::PhantomData, num::NonZeroU32};

#[derive(Debug, Clone)]
pub struct Uri<'a> {
    ptr: *const u8,
    len: u32,
    scheme_end: Option<NonZeroU32>,
    path: (u32, u32),
    query_end: Option<NonZeroU32>,
    fragment_start: Option<NonZeroU32>,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> Uri<'a> {}

// impl<'a> fmt::Debug for Uri<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // f.debug_struct("Uri").field(name, value)
//     }
// }

impl<'a> Default for Uri<'a> {
    fn default() -> Self {
        Uri {
            ptr: "".as_ptr(),
            len: 0,
            scheme_end: None,
            path: (0, 0),
            query_end: None,
            fragment_start: None,
            _marker: PhantomData,
        }
    }
}
