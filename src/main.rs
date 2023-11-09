#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod row;
mod terminal;
use clap::Parser;
pub use document::Document;
pub use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    println!("started !");

    let args = Args::parse();

    let document = if args.path.is_empty() {
        Document::default()
    } else {
        let path = args.path;
        Document::open(&path).unwrap_or_default()
    };

    Editor::default(document).run();
}
