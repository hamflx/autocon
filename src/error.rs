#[derive(thiserror::Error, Debug)]
pub enum AutoConError {
    #[error("Win32 Error")]
    Win32Error(&'static str, std::io::Error),
    #[error("Timeout")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, AutoConError>;

pub fn last_os_error(ctx: &'static str) -> AutoConError {
    AutoConError::Win32Error(ctx, std::io::Error::last_os_error())
}
