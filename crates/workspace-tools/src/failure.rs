#[derive(Debug)]
pub struct Failure {
    pub message: String,
    pub code: u8,
}

impl Failure {
    pub fn new(code: u8, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}
