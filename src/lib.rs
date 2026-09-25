mod app;

use app::App;
use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};

pub fn init() -> Result<()> {
    execute!(std::io::stdout(), EnableMouseCapture)?;
    ratatui::run(|terminal| App::default().run(terminal))?;

    execute!(std::io::stdout(), DisableMouseCapture)?;
    Ok(())
}
