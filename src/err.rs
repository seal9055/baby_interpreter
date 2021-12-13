#[derive(Debug, Clone)]
pub struct Error {
    pub line: u32,
    pub err: String,
}

impl Error {
    pub fn new(s: String, l: u32) -> Error {
        Error {
            line: l,
            err: s,
        }
    }
}
