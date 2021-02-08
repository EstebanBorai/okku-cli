use anyhow::{Error, Result};
use std::io::{self, Stdout};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

use crate::anyhowize;

mod events;
mod text_field;

const KEY_TEXT_INPUT_INSERT: char = 'i';
const KEY_QUIT_SESSION: char = 'q';

pub type TerminalInstance =
    Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;

pub struct UI;

impl UI {
    pub fn draw() -> Result<()> {
        let mut terminal = UI::make_terminal().unwrap();
        let mut text_field = text_field::TextField::default();
        let mut event_handler = events::EventHandler::new();

        loop {
            terminal
                .draw(|f| {
                    let info_header = Block::default().title("Okku").borders(Borders::ALL);
                    let chat = Block::default().title("Chat").borders(Borders::ALL);
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints([
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10),
                        ])
                        .split(f.size());

                    f.render_widget(info_header, chunks[0]);
                    f.render_widget(chat, chunks[1]);
                    f.render_widget(text_field.draw().unwrap(), chunks[2]);

                    if matches!(text_field.input_mode(), text_field::Mode::Insert) {
                        f.set_cursor(
                            chunks[2].x + text_field.value().width() as u16 + 1,
                            chunks[2].y + 1,
                        );
                    }
                })
                .map_err(|e| anyhowize!(e))?;

            if let events::Event::Input(input) = event_handler.next()? {
                match text_field.input_mode() {
                    text_field::Mode::Normal => match input {
                        Key::Char(KEY_TEXT_INPUT_INSERT) => {
                            text_field.set_input_mode(text_field::Mode::Insert);
                            event_handler.disable_exit_key();
                        }
                        Key::Char(KEY_QUIT_SESSION) => {
                            break;
                        }
                        _ => {}
                    },
                    text_field::Mode::Insert => match input {
                        Key::Esc => {
                            text_field.set_input_mode(text_field::Mode::Normal);
                            event_handler.enable_exit_key();
                        }
                        Key::Char(c) => {
                            text_field.set_value(c);
                        }
                        Key::Backspace => {
                            text_field.backspace();
                        }
                        _ => {}
                    },
                }
            }
        }

        Ok(())
    }

    fn make_terminal() -> Result<TerminalInstance> {
        let stdout = io::stdout().into_raw_mode().map_err(|e| anyhowize!(e))?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);

        Terminal::new(backend).map_err(|e| anyhowize!(e))
    }
}
