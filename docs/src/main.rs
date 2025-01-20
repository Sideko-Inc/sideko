use clap_markdown::{help_markdown_custom, MarkdownOptions};
use sideko::cli::SidekoCli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let options = MarkdownOptions::new()
        .title("The Sideko Command Line Interface for generating API Ecosystem tools".into())
        .show_footer(false)
        .show_table_of_contents(true);

    let docs = help_markdown_custom::<SidekoCli>(&options);
    let docs_path = std::env::current_dir()?.join("../docs/CLI.md");
    std::fs::write(docs_path, docs.as_bytes())?;

    println!("Documentation generated successfully!");
    Ok(())
}
