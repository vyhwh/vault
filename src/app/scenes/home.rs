use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::Rect,
    widgets::Widget,
};

#[derive(Clone, Copy, Default)]
pub struct HomeScene;

impl HomeScene {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Tratar eventos de teclado exclusivamente na cena home
            _ => {}
        }
    }
}

impl Widget for &HomeScene {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Renderizar o conteúdo da cena home
    }
}
