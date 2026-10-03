mod scenes;

use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Padding;
use ratatui::widgets::Widget;
use scenes::HomeScene;
use scenes::Scene;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};

pub struct App {
    scene: Scene,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let viewport = self.render_shell(frame);
        self.scene.render(frame, viewport);
    }

    fn render_shell(&self, frame: &mut Frame) -> Rect {
        // A criação do Shell deve ficar aqui.
        //
        // A cena atual deve apenas receber o lugar a onde desenhar, não definir
        // toda a UI.

        // Layout vertical
        let vertical = Layout::vertical([
            Constraint::Percentage(15),
            Constraint::Fill(1),
            Constraint::Percentage(15),
            Constraint::Length(1),
        ])
        .split(frame.area());

        // Layout horizontal
        let horizontal = Layout::horizontal([
            Constraint::Percentage(15),
            Constraint::Fill(1),
            Constraint::Percentage(15),
        ])
        .split(vertical[1]);

        // Bloco onde o conteúdo da cena atual deve ser renderizado
        let block = Block::bordered()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::new().dark_gray())
            .padding(Padding {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            });

        block.clone().render(horizontal[1], frame.buffer_mut());

        // Mensagem de ajuda
        let bottom_help = "↑/↓ for navigation";
        Text::from(bottom_help)
            .centered()
            .gray()
            .render(vertical[3], frame.buffer_mut());

        // Retorna o lugar/viewport onde a cena atual deve ser renderizada
        return block.inner(horizontal[1]);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => self.scene.handle_key_event(key_event),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            scene: Scene::Home(HomeScene::default()),
            exit: false,
        }
    }
}
