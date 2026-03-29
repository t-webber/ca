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
#![expect(clippy::missing_errors_doc, reason = "it's a cli")]

use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::{Context as _, bail};

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

impl Ca {
    /// Entry point for the [`Ca`] app.
    #[expect(clippy::unused_self, unused_variables, reason = "todo")]
    fn run(self) -> color_eyre::Result<()> {
        let cargo_toml_path = find_cargo_toml()?;
        let cargo_toml_content = read_to_string(&cargo_toml_path)
            .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;
        Ok(())
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ca::parse().run()?;
    Ok(())
}

/// Returns the path to the first `Cargo.toml` encountered.
fn find_cargo_toml() -> color_eyre::Result<PathBuf> {
    let cwd = current_dir().context("Failed to read cwd")?;
    let mut path = cwd.as_path();
    loop {
        let toml = path.join("Cargo.toml");
        if toml.is_file() {
            return Ok(toml);
        }
        if let Some(parent) = path.parent() {
            path = parent;
        } else {
            bail!("Couldn't find Cargo.toml, are you in a cargo project?");
        }
    }
}
