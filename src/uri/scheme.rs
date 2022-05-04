#[derive(Debug, Clone, Copy)]
pub struct Scheme<'a>(&'a str);
impl<'a> Scheme<'a> {
    pub fn as_str(self) -> &'a str {
        self.0
    }
    
}
