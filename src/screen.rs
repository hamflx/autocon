use std::time::{Duration, SystemTime};

use windows::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleScreenBufferInfo, GetStdHandle, ReadConsoleOutputCharacterA,
        CONSOLE_SCREEN_BUFFER_INFO, COORD, STD_OUTPUT_HANDLE,
    },
};

use crate::{
    error::{last_os_error, AutoConError, Result},
    matcher::MatchScreenBuffer,
};

pub struct ScreenBuffer {
    std_out: HANDLE,
    buffer: Vec<u8>,
    content: String,
    y: usize,
}

impl ScreenBuffer {
    pub fn new() -> Result<Self> {
        let std_out = unsafe { GetStdHandle(STD_OUTPUT_HANDLE).unwrap() };
        Ok(Self {
            std_out,
            buffer: Vec::new(),
            content: String::new(),
            y: get_screen_info(std_out)?.dwCursorPosition.Y as _,
        })
    }

    pub fn reset_current_pos(&mut self) -> Result<()> {
        self.y = get_screen_info(self.std_out)?.dwCursorPosition.Y as _;
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        let screen = get_screen_info(self.std_out)?;
        let start_point = COORD {
            X: 0,
            Y: self.y as _,
        };
        let length = (screen.dwSize.X as usize + 2) * (screen.dwSize.Y as usize + 2);
        if self.buffer.len() < length {
            self.buffer.resize(length, 0);
        }
        let mut read_bytes = 0;
        let result = unsafe {
            ReadConsoleOutputCharacterA(
                self.std_out,
                &mut self.buffer,
                start_point,
                &mut read_bytes,
            )
        };
        if !result.as_bool() {
            return Err(last_os_error("Failed to read console output"));
        };
        let content: String = self.buffer[..read_bytes as _]
            .chunks(screen.dwSize.X as _)
            .map(|line| {
                Into::<String>::into(String::from_utf8_lossy(line))
                    .trim()
                    .to_owned()
                    + "\n"
            })
            .collect();
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
