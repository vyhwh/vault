mod app;

use app::App;
use color_eyre::eyre::Result;

pub fn init() -> Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))?;

    Ok(())
}
