///! tables 字符 根据 RFC 3986    
///!
///! 这个模块预定义了值为 0 或者 1 的 tables
/// table主要是决定，字符串里 bytes 的方式, 并且保证 bytes 是 ASCII

#[derive(Debug, Copy, Clone)]
pub struct Table {
    arr: [u8; 256],
    allow_enc: bool,
}

impl Table {
    /// 生成一个 bytes 的 table
    pub const fn gen(mut bytes: &[u8]) -> Table {
        let mut arr = [0; 256];
        while let [cur, rem @ ..] = bytes {
            arr[*cur as usize] = 1;
            bytes = rem
        }
        Table {
            arr,
            allow_enc: false,
        }
    }
    /// 标记这个 table 是一个 percent-encoded
    pub const fn enc(mut self) -> Table {
        self.allow_enc = true;
        self
    }

    /// 从 table 里返回指定的值
    pub const fn get(&self, x: u8) -> u8 {
        self.arr[x as usize]
    }
    /// 如果 table 中的 bytes 不等于 0 就返回 true
    pub const fn contains(&self, x: u8) -> bool {
        self.get(x) != 0
    }

    /// 左位移 table 的值
    pub const fn shl(mut self, n: u8) -> Table {
        let mut i = 0;
        while i < 128 {
            self.arr[i] <<= n;
            i += 1;
        }
        self
    }

    /// 整合 2 个 tables 为 1 个 table
    pub const fn or(mut self, t: &Table) -> Table {
        let mut i = 0;
        while i < 128 {
            self.arr[i] |= t.arr[i];
            i += 1
        }
        self.allow_enc |= t.allow_enc;
        self
    }

    /// 如果 table 是 percent-encoded 就返回 true
    pub const fn allow_enc(&self) -> bool {
        self.allow_enc
    }
}

const fn gen(bytes: &[u8]) -> Table {
    Table::gen(bytes)
}

/// ALPHA = A-Z / a-z
pub static ALPHA: &Table = &gen(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");

/// DIGIT = 0-9
pub static DIGIT: &Table = &gen(b"0123456789");

/// HEXDIG = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
///                / "a" / "b" / "c" / "d" / "e" / "f"
pub static HEXDIG: &Table = &DIGIT.or(&gen(b"ABCDEFabcdef"));

/// reserved = gen-delims / sub-delims
pub static RESERVED: &Table = &GEN_DELIMS.or(SUB_DELIMS);

/// gen-delims = ":" / "/" / "?" / "#" / "[" / "]" / "@"
pub static GEN_DELIMS: &Table = &gen(b":/?#[]@");

/// sub-delims = "!" / "$" / "&" / "'" / "(" / ")"
///            / "*" / "+" / "," / ";" / "="
pub static SUB_DELIMS: &Table = &gen(b"!$&'()*+,;=");

/// unreserved = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub static UNRESERVED: &Table = &ALPHA.or(DIGIT).or(&gen(b"-._~"));

/// pchar = unreserved / pct-encoded / sub-delims / ":" / "@"
pub static PCHAR: &Table = &UNRESERVED.or(SUB_DELIMS).or(&gen(b":@")).enc();

/// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
pub static SEGMENT_NC: &Table = &UNRESERVED.or(SUB_DELIMS).or(&gen(b"@")).enc();

/// scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
pub static SCHEME: &Table = &ALPHA.or(DIGIT).or(&gen(b"+-."));

/// userinfo = *( unreserved / pct-encoded / sub-delims / ":" )
pub static USERINFO: &Table = &UNRESERVED.or(SUB_DELIMS).or(&gen(b":")).enc();

/// IPvFuture = "v" 1\*HEXDIG "." 1\*( unreserved / sub-delims / ":" )
pub static IPV_FUTURE: &Table = &UNRESERVED.or(SUB_DELIMS).or(&gen(b":"));

/// reg-name = *( unreserved / pct-encoded / sub-delims )
pub static REG_NAME: &Table = &UNRESERVED.or(SUB_DELIMS).enc();

/// path = *( pchar / "/" )
pub static PATH: &Table = &PCHAR.or(&gen(b"/"));

/// query = fragment = *( pchar / "/" / "?" )
pub static QUERY_FRAGMENT: &Table = &PCHAR.or(&gen(b"/?"));

/// RFC 6874: ZoneID = 1*( unreserved / pct-encoded )
pub(crate) static ZONE_ID: &Table = &UNRESERVED.enc();
