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

/// Parses a crate line from the `Cargo.toml`.
mod crateline;

use std::env::current_dir;
use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::eyre::{Context as _, ContextCompat as _, bail};
use toml_edit::{DocumentMut, Item, Table};

use crate::crateline::Crate;

/// Main struct used to parse Cli inputs.
#[derive(Parser, Default, Debug)]
struct Ca {
    /// Features to use for the given crate.
    #[arg(short, long, requires = "name")]
    features: Vec<String>,
    /// Name of the crate to add or update.
    #[arg(short, long)]
    name: Option<String>,
    /// Don't write to the `Cargo.toml`, simply print the output. Useful for debugging.
    #[arg(short = 'w', long, default_value_t = false)]
    nowrite: bool,
    /// Path to the given crate.
    #[arg(short, long, requires = "name")]
    path: Option<String>,
}

impl Ca {
    /// Entry point for the [`Ca`] app, modifying the right `Cargo.toml`.
    #[expect(clippy::dbg_macro, reason = "goal of nowrite")]
    fn run(self, cargo_toml_path: &Path) -> color_eyre::Result<()> {
        let input = read_to_string(cargo_toml_path).context("Failed to read file")?;
        let output = self.run_no_file(&input).context("Failed to parse file")?;
        if self.nowrite {
            dbg!(output);
            Ok(())
        } else {
            fs::write(cargo_toml_path, output).context("Failed to write to file")
        }
    }

    /// Entry point for the [`Ca`] app, taking as input the content of the `Cargo.toml` and
    /// returning the new one.
    #[expect(clippy::unused_self, clippy::dbg_macro, reason = "todo")]
    fn run_no_file(&self, file_content: &str) -> color_eyre::Result<String> {
        let toml: DocumentMut = file_content
            .trim_start_matches('\u{feff}')
            .parse()
            .context("Failed to parse toml")?;

        let old_deps = toml
            .get("dependencies")
            .map_or_else(|| Item::Table(Table::default()), Clone::clone)
            .into_table()
            .ok()
            .context("dependencies exists, but isn't a table")?;

        for item in old_deps {
            let cr = Crate::try_from(item)?;
            dbg!(cr);
        }

        dbg!(Ok(toml.to_string()))
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cargo_toml_path = find_cargo_toml()?;
    Ca::parse()
        .run(&cargo_toml_path)
        .with_context(|| format!("In {}", cargo_toml_path.display()))?;
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
