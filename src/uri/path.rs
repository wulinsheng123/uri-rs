#[derive(Debug, Clone, Copy)]
pub struct Path<'a>(pub &'a str);

impl<'a> Path<'a> {
   pub  fn as_str(&self) -> &'a str {
        self.0
    }
}
