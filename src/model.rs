use ratatui::style::Style;

pub struct Model {
    pub query: String,
    pub query_cursor: usize,
    pub items: Vec<String>,
    pub selected_item: Option<usize>,

    pub prompt: String,
    pub colors: Colorscheme,
}
pub struct Colorscheme {
    pub selected: Style,
    pub matched: Style,
}
