use super::uri::Uri;

struct Parser<'a> {
    out: Uri<'a>,
    pos: u32,
    mark: u32,
}
