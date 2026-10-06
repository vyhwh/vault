use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Color, Stylize},
    text::Text,
    widgets::{Block, Padding, Widget},
};

pub struct Password {
    pub title: String,
}

pub struct Passwords {
    pub cursor: usize,
    pub entries: Vec<Password>,
}

impl Passwords {
    pub fn render(&self, frame: &mut Frame, viewport: Rect) {
        if self.entries.len() == 0 {
            Text::from("No passwords saved yet").render(viewport, frame.buffer_mut());
            return;
        }

        for (i, item) in self.entries.iter().enumerate() {
            if i >= viewport.height as usize {
                break;
            }

            let area = Rect {
                x: viewport.x,
                y: viewport.y + i as u16,
                width: viewport.width,
                height: 1,
            };

            let block = Block::new().padding(Padding {
                left: 3,
                right: 0,
                top: 0,
                bottom: 0,
            });

            let title = Text::from(item.title.as_str());

            if self.cursor == i {
                title
                    .fg(Color::Black)
                    .render(block.inner(area), frame.buffer_mut());
                block.bg(Color::Gray).render(area, frame.buffer_mut());

                continue;
            }

            title
                .fg(Color::DarkGray)
                .render(block.inner(area), frame.buffer_mut());
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => {
                if self.cursor != 0 {
                    self.cursor -= 1;
                } else {
                    self.cursor = self.entries.len() - 1;
                }
            }
            KeyCode::Down => {
                if self.cursor < (self.entries.len() - 1) {
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                }
            }
            _ => {}
        }
    }
}

impl Default for Passwords {
    fn default() -> Self {
        Passwords {
            cursor: 0,
            entries: vec![],
        }
    }
}
