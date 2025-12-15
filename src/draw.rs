use crate::model::Model;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
};
use ratatui::{
    layout::Position,
    widgets::{List, ListItem, ListState, Paragraph},
};

pub fn draw(frame: &mut Frame, model: &Model) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.area());

    let prompt_area = chunks[0];
    let list_area = chunks[1];

    let list_widget = make_list(&model);
    let prompt_widget = make_prompt(&model);

    frame.render_widget(prompt_widget, prompt_area);
    frame.render_stateful_widget(
        list_widget.widget,
        list_area,
        &mut list_widget.state.clone(),
    );

    frame.set_cursor_position(Position {
        x: prompt_area.x
            + u16::try_from(model.prompt.len()).unwrap()
            + u16::try_from(model.query_cursor).unwrap(),
        y: prompt_area.y,
    });
}

struct ListWidget<'w> {
    pub widget: List<'w>,
    pub state: ListState,
}
fn make_list(model: &Model) -> ListWidget {
    let matcher = SkimMatcherV2::default();
    let mut scored_items = model
        .items
        .iter()
        .filter_map(|text| {
            matcher
                .fuzzy_indices(text, &model.query)
                .map(|(score, indices)| (text.clone(), score, indices))
        })
        .collect::<Vec<_>>();
    scored_items.sort_by_key(|(_text, score, _indices)| -*score);
    let items = scored_items
        .into_iter()
        .map(|(text, _score, indices)| ListItem::new(highlight_matches(model, text, indices)))
        .collect::<Vec<ListItem>>();

    ListWidget {
        widget: List::new(items).highlight_style(model.colors.selected),
        state: ListState::default().with_selected(model.selected_item),
    }
}
fn highlight_matches(model: &Model, text: String, indices: Vec<usize>) -> Line {
    let color_of = |i: usize| {
        if indices.contains(&i) {
            model.colors.matched
        } else {
            Style::default()
        }
    };
    text.chars()
        .enumerate()
        .map(|(i, character)| Span::from(character.to_string()).style(color_of(i)))
        .collect()
}

fn make_prompt(model: &Model) -> Paragraph {
    Paragraph::new(model.prompt.clone() + &model.query.clone())
}
