use anyhow::Result;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};

#[derive(Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct TextField {
    current_value: String,
    mode: Mode,
}

impl Default for TextField {
    fn default() -> Self {
        Self {
            current_value: String::default(),
            mode: Mode::Normal,
        }
    }
}

impl TextField {
    pub fn draw(&mut self) -> Result<Paragraph> {
        let next_block = if self.mode == Mode::Normal {
            Paragraph::new(self.current_value.as_ref())
                .style(Style::default())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Normal\t[Press \"i\" to Enter Insert Mode]"),
                )
        } else {
            Paragraph::new(self.current_value.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Insert\t[Press Esc to Exit Insert Mode]"),
                )
        };

        Ok(next_block)
    }

    pub fn input_mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn set_input_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn value(&self) -> String {
        self.current_value.clone()
    }

    pub fn set_value(&mut self, character: char) {
        self.current_value.push(character);
    }

    pub fn backspace(&mut self) {
        self.current_value.pop();
    }
}
