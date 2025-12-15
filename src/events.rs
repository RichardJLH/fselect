use crate::Result;
use crossterm::event::Event;
use crossterm::event::{self, KeyCode, KeyEvent};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::Model;

pub enum Status {
    Exit,
    Chose(String),
    Continue,
}
pub fn handle_event(model: &mut Model) -> Result<Status> {
    match event::read()? {
        Event::Key(key) => Ok(handle_key(key, model)),
        _ => Ok(Status::Continue),
    }
}
fn shown_items(model: &Model) -> Vec<String> {
    let matcher = SkimMatcherV2::default();
    let mut scored_items = model
        .items
        .iter()
        .filter_map(|text| {
            matcher
                .fuzzy_match(text, &model.query)
                .map(|score| (text.clone(), score))
        })
        .collect::<Vec<_>>();
    scored_items.sort_by_key(|(_text, score)| -*score);

    scored_items
        .into_iter()
        .map(|(text, _score)| text)
        .collect()
}
pub fn handle_key(key: KeyEvent, model: &mut Model) -> Status {
    match key.code {
        KeyCode::Esc => return Status::Exit,
        KeyCode::Enter => {
            if let Some(i) = model.selected_item {
                return Status::Chose(shown_items(model)[i].clone());
            }
        }
        KeyCode::Char(character) => {
            model.query.insert(model.query_cursor, character);
            model.query_cursor += 1;

            let items = shown_items(model);
            if items.is_empty() {
                model.selected_item = None;
            } else if let Some(true) = model.selected_item.map(|i| i >= items.len()) {
                model.selected_item = Some(items.len() - 1);
            }
        }

        KeyCode::Backspace => {
            if model.query_cursor > 0 {
                model.query.remove(model.query_cursor - 1);
                model.query_cursor -= 1;
            }
        }
        KeyCode::Delete => {
            if model.query_cursor < model.query.len() {
                model.query.remove(model.query_cursor);
            }
        }

        KeyCode::Left => {
            model.query_cursor = model.query_cursor.saturating_sub(1);
        }
        KeyCode::Right => {
            model.query_cursor = usize::min(model.query_cursor + 1, model.query.len());
        }
        KeyCode::Home => {
            model.query_cursor = 0;
        }
        KeyCode::End => {
            model.query_cursor = model.query.len();
        }

        KeyCode::Up => {
            let items = shown_items(model);
            let items_exist = !items.is_empty();
            model.selected_item = items_exist.then_some(match model.selected_item {
                Some(0) => items.len() - 1,
                Some(i) => i - 1,
                Option::None => items.len() - 1,
            });
        }
        KeyCode::Down => {
            let items = shown_items(model);
            let items_exist = !items.is_empty();
            model.selected_item = items_exist.then_some(match model.selected_item {
                Some(i) => (i + 1) % items.len(),
                Option::None => 0,
            });
        }

        _ => {}
    }

    Status::Continue
}
