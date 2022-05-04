use std::{num::NonZeroU32, str};

use crate::{
    encoding::table::{Table, PATH, QUERY_FRAGMENT, SCHEME, USERINFO},
    uri::uri::{HostInternal, Uri},
};

enum PathKind {
    General,
    AbEmpty,
    ContinuedNoScheme,
}

#[derive(Debug)]
struct Parser<'a> {
    out: Uri<'a>,
    pos: u32,
    mark: u32,
}

pub(crate) fn parse(s: &[u8]) -> Uri {
    println!("{:?}", s);
    let mut parser = Parser {
        out: Uri {
            ptr: s.as_ptr(),
            len: s.len() as u32,
            ..Default::default()
        },
        pos: 0,
        mark: 0,
    };
    parser.parse_from_scheme();
    parser.out
}

impl<'a> Parser<'a> {
    fn parse_from_scheme(&mut self) {
        self.scan(SCHEME);
        let m = self.peek(0);

        println!(">>>>{:?}", unsafe {
            core::str::from_utf8_unchecked(&[m.unwrap()])
        });
        if m == Some(b':') {
            // 组件 Scheme 开始
            if self.pos != 0 && self.get(0).is_ascii_alphabetic() {
                self.out.scheme_end = NonZeroU32::new(self.pos + 1);
            } else {
                // TODO error
            }

            self.skip(1);
            self.parse_from_authority();
        }
    }

    fn parse_from_authority(&mut self) {
        if !self.read_str("//") {}
        let host_out;
        static TABLE: &Table = &USERINFO.shl(1).or(&Table::gen(b":"));
        let mut colon_cnt = 0;

        self.mark();
        self.scan_enc(TABLE, |v| {
            colon_cnt += (v & 1) as u32;
        });

        if self.peek(0) == Some(b'@') {
            // host_out = (self.pos, self.pos, HostInternal::RegName);
        } else if self.marked_len() == 0 {
            // host_out = (self.pos, self.pos, HostInternal::RegName);
        } else {
            let host_end = match colon_cnt {
                0 => self.pos,
                1 => {
                    let mut i = self.pos - 1;
                    loop {
                        let x = unsafe { self.get_unchecked(i) };
                        if !x.is_ascii_digit() {
                            if x == b':' {
                                break;
                            } else {
                                // TODO
                            }
                        }
                        i -= 1;
                    }
                    i
                }
                _ => {
                    let mut i = self.mark;
                    loop {
                        let x = unsafe { self.get_unchecked(i) };
                        if x == b':' {
                            // TODO
                        }
                        i += 1;
                    }
                }
            };

            let state = (self.out.len, self.pos);
            self.out.len = host_end;
            self.pos = self.mark;
            host_out = (self.mark, host_end, HostInternal::RegName);
            (self.out.len, self.pos) = state;

            let host_start = unsafe { NonZeroU32::new_unchecked(host_out.0) };
            self.out.host = Some((host_start, host_out.1, host_out.2));
            self.parse_from_path(PathKind::AbEmpty);
        }
    }

    fn parse_from_path(&mut self, kind: PathKind) {
        self.out.path = match kind {
            PathKind::AbEmpty => {
                let start = self.pos;
                self.read(PATH);
                (start, self.pos)
            }
            _ => {
                panic!("错误")
            }
        };

        if self.read_str("?") {
            self.read(QUERY_FRAGMENT);
            self.out.query_end = NonZeroU32::new(self.pos);
        }
        if self.read_str("#") {
            self.out.fragment_start = NonZeroU32::new(self.pos);
            self.read(QUERY_FRAGMENT);
        }
    }

    fn read(&mut self, table: &Table) -> bool {
        let start = self.pos;
        self.scan(table);
        self.pos != start
    }

    fn scan_enc(&mut self, table: &Table, mut f: impl FnMut(u8)) {
        let mut i = self.pos;
        while i < self.out.len {
            let x = self.get(i);
            if x == b'%' {
                // TODO
            } else {
                let v = table.get(x);
                if v == 0 {
                    break;
                }
                f(v);
                i += 1;
            }
        }
        self.pos = i;
    }

    fn read_str(&mut self, s: &str) -> bool {
        let len = s.len() as u32;
        let res = self.pos + len <= self.out.len
            && (0..len)
                .all(|i| unsafe { self.get_unchecked(self.pos + i) } == s.as_bytes()[i as usize]);
        if res {
            self.skip(len);
        }
        res
    }

    fn scan(&mut self, table: &Table) {
        let mut i = self.pos;

        while i < self.out.len {
            if !table.contains(self.get(i)) {
                break;
            }
            i += 1;
        }
        self.pos = i;
    }

    fn get(&self, i: u32) -> u8 {
        unsafe { self.get_unchecked(i) }
    }

    unsafe fn get_unchecked(&self, i: u32) -> u8 {
        unsafe { *self.out.ptr.add(i as usize) }
    }

    fn peek(&mut self, i: u32) -> Option<u8> {
        (self.pos + 1 < self.out.len).then(|| self.get(self.pos + i))
    }

    fn mark(&mut self) {
        self.mark = self.pos;
    }

    fn marked_len(&self) -> u32 {
        self.pos - self.mark
    }

    fn skip(&mut self, n: u32) {
        self.pos += n;
    }
}

#[test]
fn name() {
    let u = parse(b"http://www.ietf.org/rfc/rfc2396.txt?name=2323");
    println!("{:?}", u);
}
