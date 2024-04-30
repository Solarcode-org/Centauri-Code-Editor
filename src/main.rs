use std::{fs::read_to_string, io};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize as _,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
    Frame,
};

mod tui;

/// The Centauri code editor.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the file.
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut terminal = tui::init()?;
    let app_result = App {
        contents: read_to_string(&args.file)?,
        file: args.file,
        exit: false,
        insert_mode: false,
        row: 0,
        mov_speed: 1,
    }
    .run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    file: String,
    exit: bool,
    contents: String,
    insert_mode: bool,
    row: usize,
    mov_speed: usize,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.modifiers {
            KeyModifiers::ALT => self.mov_speed = 5,
            _ => self.mov_speed = 1,
        }

        match key_event.code {
            KeyCode::Char('q') => {
                if !self.insert_mode {
                    self.exit();
                }
            }
            KeyCode::Char('i') => {
                if !self.insert_mode {
                    self.insert_mode = true
                }
            }
            KeyCode::Down => self.row += self.mov_speed,
            KeyCode::Up => {
                if self.row > self.mov_speed - 1 {
                    self.row -= self.mov_speed
                }
            }
            KeyCode::Esc => self.insert_mode = false,
            _ => {}
        }
    }
}

fn highlight<'a>(word: &'a str, previous: &'a str) -> Span<'a> {
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(format!(" Centauri Code Editor – {} ", self.file).bold());
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Insert ".into(),
            "<I> ".blue().bold(),
            " Normal ".into(),
            "<ESC> ".blue().bold(),
            if self.insert_mode {
                "Insert Mode ".light_red().bold()
            } else {
                "Normal Mode ".light_green().bold()
            },
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let mut lines = vec![];

        let width = self
            .contents
            .clone()
            .lines()
            .count()
            .checked_ilog10()
            .unwrap_or(0)
            + 1;

        let mut lines_ = self.contents.lines();

        for _ in 0..self.row {
            lines_.next();
        }

        for (num, line) in lines_.enumerate() {
            if line.trim().starts_with("//")
                || line.trim().starts_with("/*")
                || line.trim().ends_with("*/")
            {
                lines.push(Line::from(vec![
                    format!(
                        " {:width$} | ",
                        num + 1 + self.row,
                        width = width.try_into().unwrap_or(usize::MAX)
                    )
                    .yellow(),
                    line.green().dim(),
                ]));
                continue;
            }

            let mut words = vec![];
            let mut previous = "";

            let mut start = 0;
            for (index, character) in line.char_indices() {
                if character.is_whitespace() {
                    let word = highlight(&line[start..index], previous);

                    words.push(word);
                    words.push(character.to_string().into());

                    previous = &line[start..index];

                    start = index + character.len_utf8();
                }
            }

            words.push(line[start..].into());

            let mut line = vec![format!(
                " {:width$} | ",
                num + 1 + self.row,
                width = width.try_into().unwrap_or(usize::MAX)
            )
            .yellow()];

            line.extend(words);

            lines.push(Line::from(line));
        }

        let file_contents = Text::from(lines);

        Paragraph::new(file_contents)
            .block(block.clone())
            .render(area, buf);
    }
}
