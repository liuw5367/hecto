#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod document;
mod editor;
mod row;
mod terminal;
use clap::{Args, Parser, Subcommand};
pub use document::Document;
pub use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

/// Simple Editor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    config: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,

    /// file path
    value: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// test
    Test {
        /// test arg
        #[arg(short, long)]
        list: bool,
    },
    /// Reverses a string (1)
    Reverse(Reverse),
    /// Inspects a string (2)
    Inspect(Inspect),
}

#[derive(Args, Debug)]
struct Reverse {
    string: Option<String>,
}

#[derive(Args, Debug)]
struct Inspect {
    /// Tje tsing to inspect.
    string: Option<String>,

    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

fn main() {
    let args = Cli::parse();

    Editor::open(&args.value.unwrap_or_default()).run();
}
