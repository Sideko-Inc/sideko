use tabled::{
    settings::{object::Rows, themes::Colorization, Alignment, Color, Padding, Panel, Remove},
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

pub fn preview_table(header: &str, text: &str, line_limit: usize) -> Table {
    let split: Vec<String> = text.split("\n").map(String::from).collect();
    let mut preview = split[0..split.len().min(line_limit)].to_vec();
    if preview.len() < split.len() {
        preview.push("...".into());
    }

    let mut table = Table::new([preview.join("\n")]);
    table.with(Remove::row(Rows::first())); // tabled automatically puts the datatype as the column header
    header_panel(&mut table, header);

    table
}
