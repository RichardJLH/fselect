use std::io;
use std::io::Stdout;

use crate::Result;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::CrosstermBackend;

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
pub fn in_terminal<F, R>(callback: F) -> Result<R>
where
    F: FnOnce(&mut Terminal) -> R,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let r = callback(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(r)
}
