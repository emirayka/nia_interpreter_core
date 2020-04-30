#[derive(Debug, Clone)]
pub enum ParseError {
    TrailingInput(String),
    NomError((String, nom::error::ErrorKind)),
    NomFailure((String, nom::error::ErrorKind)),
    NomIncomplete(),
}