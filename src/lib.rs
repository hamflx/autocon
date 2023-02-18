use console::Console;
use error::Result;
use screen::ScreenBuffer;

pub mod console;
pub mod error;
pub mod matcher;
pub mod screen;

#[macro_export]
macro_rules! user_defined_error {
    ($($arg:tt)*) => {
        $crate::error::AutoConError::UserDefinedError(format!($($arg)*))
    };
}

pub trait ConAutomation {
    fn run(&mut self, screen: &mut ScreenBuffer, input: &Console) -> Result<()>;
}
