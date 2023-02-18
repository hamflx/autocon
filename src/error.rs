#[derive(thiserror::Error, Debug)]
pub enum AutoConError {
    #[error("{0} (Win32 Error): {1}")]
    Win32Error(&'static str, std::io::Error),
    #[error("Timeout")]
    Timeout,
    #[error("User defined error: {0}")]
    UserDefinedError(String),
}

pub type Result<T> = std::result::Result<T, AutoConError>;

pub fn last_os_error(ctx: &'static str) -> AutoConError {
    AutoConError::Win32Error(ctx, std::io::Error::last_os_error())
}
