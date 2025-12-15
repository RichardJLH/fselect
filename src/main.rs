use std::io::BufRead;
mod draw;
mod events;
mod model;
mod terminal;

use anyhow::Error;
pub use anyhow::Result;
use ratatui::style::Color;
use ratatui::style::Style;
use std::io::BufReader;
use std::io::stdin;

pub use draw::draw;
pub use model::Model;
pub use terminal::in_terminal;

pub use terminal::Terminal;

use crate::events::Status;
use crate::events::handle_event;
use crate::model::Colorscheme;

fn main() -> Result<()> {
    let model = Model {
        query: String::new(),
        query_cursor: 0,
        items: read_items()?,
        selected_item: None,

        prompt: String::from("> "),
        colors: Colorscheme {
            selected: Style::default().bg(Color::Blue),
            matched: Style::default().fg(Color::Red),
        },
    };

    let chosen = in_terminal(|terminal| event_loop(terminal, model))??;
    if let Some(option) = chosen {
        println!("{}", option);
    }

    Ok(())
}
fn read_items() -> Result<Vec<String>> {
    BufReader::new(stdin())
        .lines()
        .map(|result| result.map_err(Error::from))
        .collect()
}
fn event_loop(terminal: &mut Terminal, mut model: Model) -> Result<Option<String>> {
    loop {
        terminal.draw(|frame| draw(frame, &model))?;

        match handle_event(&mut model)? {
            Status::Exit => return Ok(None),
            Status::Chose(option) => return Ok(Some(option)),
            Status::Continue => {}
        }
    }
}
