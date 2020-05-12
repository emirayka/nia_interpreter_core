#[derive(Clone, Debug)]
pub struct NiaExecuteCodeCommand {
    code: String,
}

impl NiaExecuteCodeCommand {
    pub fn new<S>(code: S) -> NiaExecuteCodeCommand
    where
        S: Into<String>,
    {
        NiaExecuteCodeCommand { code: code.into() }
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }
}
