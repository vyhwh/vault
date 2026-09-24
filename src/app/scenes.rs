mod home;

pub use home::HomeScene;

use ratatui::crossterm::event::KeyEvent;

#[derive(Clone, Copy)]
pub enum Scene {
    Home(HomeScene),
}

impl Scene {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self {
            Scene::Home(home) => home.handle_key_event(key_event),
        }
    }
}
