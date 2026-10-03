mod home;

pub use home::HomeScene;

use ratatui::{Frame, crossterm::event::KeyEvent};

pub enum Scene {
    Home(HomeScene),
}

impl Scene {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self {
            Scene::Home(home) => home.handle_key_event(key_event),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        match self {
            Self::Home(home) => home.render(frame),
        }
    }
}
