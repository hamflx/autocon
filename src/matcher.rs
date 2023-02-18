pub trait MatchScreenBuffer {
    type MatchResult;

    fn match_screen_buffer(&self, content: &str) -> Option<Self::MatchResult>;
}

pub struct MatchAuthPrompt {
    only_match_password: bool,
}

impl MatchAuthPrompt {
    pub fn new(only_match_password: bool) -> Self {
        Self {
            only_match_password,
        }
    }
}

impl MatchScreenBuffer for MatchAuthPrompt {
    type MatchResult = MatchAuthPromptResult;

    fn match_screen_buffer(&self, content: &str) -> Option<Self::MatchResult> {
        let mut iter = content
            .lines()
            .rev()
            .skip_while(|line| line.trim().is_empty());
        match iter.next() {
            Some(line)
                if !self.only_match_password
                    && line.contains("Are you sure you want to continue") =>
            {
                Some(MatchAuthPromptResult::Confirm)
            }
            Some(line) if line.to_lowercase().ends_with("password:") => {
                Some(MatchAuthPromptResult::Password)
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MatchAuthPromptResult {
    Confirm,
    Password,
}
