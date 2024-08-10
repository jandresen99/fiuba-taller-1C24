/// Error type for the project
#[derive(Debug)]
pub struct Error {
    _message: String,
}

impl Error {
    /// Creates a new error
    pub fn new(message: String) -> Self {
        Self { _message: message }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            _message: format!("IO error: {}", error),
        }
    }
}
