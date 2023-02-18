use console::Console;
use error::Result;
use screen::ScreenBuffer;

pub mod console;
pub mod error;
pub mod matcher;
pub mod screen;

pub trait ConAutomation {
    fn run(&mut self, screen: &mut ScreenBuffer, input: &Console) -> Result<()>;
}
