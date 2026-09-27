mod home;

pub use home::HomeScene;

use ratatui::{crossterm::event::KeyEvent, widgets::Widget};

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

impl Widget for Scene {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self {
            Scene::Home(home) => home.render(area, buf),
        }
    }
}
