use ratatui::{style::Stylize as _, text::Span};

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

pub fn highlight<'a>(word: &'a str, previous: &'a str) -> Span<'a> {
    let word = match word {
        "fn" | "mut" | "if" | "else" | "while" | "let" | "use" | "mod" | "struct" | "impl"
        | "pub" | "self" => word.magenta(),
        s => match previous {
            "fn" => s.green(),
            "let" | "mut" | "struct" | "impl" => s.light_blue(),
            _ => s.into(),
        },
    };

    word
}
