use ratatui::{Frame, crossterm::event::KeyEvent};

#[derive(Default)]
pub struct HomeScene;

impl HomeScene {
    pub fn render(&self, frame: &mut Frame) {
        // Renderizar o conteúdo da cena home
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Tratar eventos de teclado exclusivamente na cena home
            _ => {}
        }
    }
}
