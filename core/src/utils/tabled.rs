use tabled::{
    settings::{object::Rows, themes::Colorization, Alignment, Color, Padding, Panel},
    Table,
};

pub fn header_panel(table: &mut Table, text: &str) {
    table
        .with(Panel::header(text))
        .with(Colorization::exact(
            [Color::BOLD | Color::BG_WHITE | Color::FG_BLACK],
            Rows::first(),
        ))
        .modify(Rows::first(), Padding::new(0, 0, 1, 1))
        .modify(Rows::first(), Alignment::center());
}
