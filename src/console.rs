use std::str::Chars;

use windows::Win32::{
    Foundation::{CHAR, HANDLE},
    System::Console::{
        GetStdHandle, WriteConsoleInputA, INPUT_RECORD, INPUT_RECORD_0, KEY_EVENT,
        KEY_EVENT_RECORD, KEY_EVENT_RECORD_0, STD_INPUT_HANDLE,
    },
};

pub struct Console(HANDLE);

impl Console {
    pub fn stdin() -> Self {
        Self(unsafe { GetStdHandle(STD_INPUT_HANDLE).unwrap() })
    }

    pub fn type_string(&self, text: ConsoleInputChars) -> usize {
        let events: Vec<_> = text.collect();
        let mut written = 0;
        unsafe { WriteConsoleInputA(self.0, &events, &mut written) };
        written as _
    }
}

pub struct ConsoleInputChars<'a>(Chars<'a>);

impl<'a> ConsoleInputChars<'a> {
    pub fn new(text: &'a str) -> Self {
        Self(text.chars())
    }
}

impl<'a> Iterator for ConsoleInputChars<'a> {
    type Item = INPUT_RECORD;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|ch| INPUT_RECORD {
            EventType: KEY_EVENT as _,
            Event: INPUT_RECORD_0 {
                KeyEvent: KEY_EVENT_RECORD {
                    bKeyDown: true.into(),
                    dwControlKeyState: 0,
                    uChar: KEY_EVENT_RECORD_0 {
                        AsciiChar: CHAR(u8::try_from(ch).unwrap()),
                    },
                    wRepeatCount: 1,
                    wVirtualKeyCode: 0,
                    wVirtualScanCode: 0,
                },
            },
        })
    }
}
