use std::time::{Duration, SystemTime};

use windows::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleScreenBufferInfo, GetStdHandle, ReadConsoleOutputA, CHAR_INFO,
        CONSOLE_SCREEN_BUFFER_INFO, COORD, STD_OUTPUT_HANDLE,
    },
};

use crate::{
    error::{last_os_error, AutoConError, Result},
    matcher::MatchScreenBuffer,
};

pub struct ScreenBuffer {
    std_out: HANDLE,
    buffer: Vec<CHAR_INFO>,
    content: String,
}

impl ScreenBuffer {
    pub fn new() -> Result<Self> {
        let std_out = unsafe { GetStdHandle(STD_OUTPUT_HANDLE).unwrap() };
        Ok(Self {
            std_out,
            buffer: Vec::new(),
            content: String::new(),
        })
    }

    pub fn refresh(&mut self) -> Result<()> {
        let screen = get_screen_info(self.std_out)?;
        let length = screen.dwSize.X as usize * screen.dwSize.Y as usize;
        if self.buffer.len() < length {
            self.buffer.resize(length, Default::default());
        }
        let mut window = screen.srWindow;
        let result = unsafe {
            ReadConsoleOutputA(
                self.std_out,
                self.buffer.as_mut_ptr(),
                screen.dwSize,
                COORD { X: 0, Y: 0 },
                &mut window,
            )
        };
        if !result.as_bool() {
            return Err(last_os_error("Failed to read console output"));
        };
        let content = self
            .buffer
            .chunks(screen.dwSize.X as _)
            .map(|line| {
                let line = line
                    .iter()
                    .map(|c| unsafe { c.Char.AsciiChar.0 })
                    .skip_while(|c| *c == 0)
                    .take_while(|c| *c != 0)
                    .collect::<Vec<_>>();
                String::from_utf8_lossy(&line).trim().to_owned() + "\n"
            })
            .collect::<String>()
            .trim()
            .to_string();
        self.content = content.trim().to_string();

        Ok(())
    }

    pub fn wait_for<M>(&mut self, timeout: Duration, matcher: M) -> Result<M::MatchResult>
    where
        M: MatchScreenBuffer,
    {
        let begin = SystemTime::now();
        let mut last_error = None;

        loop {
            match self.refresh() {
                Ok(_) => {
                    if let Some(val) = matcher.match_screen_buffer(&self.content) {
                        break Ok(val);
                    }
                }
                Err(err) => {
                    last_error = Some(err);
                }
            }

            if let Ok(elapsed) = begin.elapsed() {
                if elapsed > timeout {
                    break Err(last_error.unwrap_or(AutoConError::Timeout));
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

fn get_screen_info(handle: HANDLE) -> Result<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut screen = Default::default();
    let get_success = unsafe { GetConsoleScreenBufferInfo(handle, &mut screen) };
    match get_success.as_bool() {
        true => Ok(screen),
        false => Err(last_os_error("Failed to retrieve screen buffer info")),
    }
}
