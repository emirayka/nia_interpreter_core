#[derive(Clone, Copy, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn from(kind: ErrorKind) -> Error {
        Error {
            kind,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Definition,
    Lookup,
}