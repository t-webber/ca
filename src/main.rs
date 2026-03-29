//! Ca, a better cargo add.
#![allow(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_trait_methods,
    clippy::question_mark_used,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::pub_with_shorthand,
    clippy::unseparated_literal_suffix,
    clippy::else_if_without_else,
    reason = "bad lints"
)]

use clap::Parser;

/// Main struct used to parse Cli inputs.
#[derive(Parser)]
struct Ca {
    /// Features to use for the given crate.
    #[arg(short, long, requires = "name")]
    features: Vec<String>,
    /// Name of the crate to add or update.
    #[arg(short, long)]
    name: Option<String>,
    /// Path to the given crate.
    #[arg(short, long, requires = "name")]
    path: Option<String>,
}

fn main() {
    Ca::parse();
}
