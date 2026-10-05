mod passwords;

pub use passwords::Passwords;

use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};

pub enum Scene {
    Passwords(Passwords),
}

impl Scene {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self {
            Scene::Passwords(passwords) => passwords.handle_key_event(key_event),
        }
    }

    pub fn render(&self, frame: &mut Frame, viewport: Rect) {
        match self {
            Self::Passwords(passwords) => passwords.render(frame, viewport),
        }
    }
}
