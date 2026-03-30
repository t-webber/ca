use color_eyre::eyre::{ContextCompat as _, bail};
use toml_edit::{Item, Table, Value};

/// Represents a crate.
///
/// It can take many forms, including:
///
/// ```toml
/// clap = { version = "0.1.0", features = ["derive"], path = "../clap" }
/// clap = "0.1.0"
/// ```
#[allow(clippy::allow_attributes, reason = "expected doesn't work")]
#[allow(clippy::missing_docs_in_private_items, reason = "self-explanatory")]
#[allow(dead_code, reason = "todo")]
#[derive(Debug, Default)]
pub struct Crate {
    features: Vec<String>,
    name: String,
    path: Option<String>,
    version: String,
}

impl Crate {
    /// Creates a [`Crate`] from a [`Table`].
    ///
    /// This means the associated line is like:
    ///
    /// ```toml
    /// clap = { version = "0.1.0", features = ["derive"], path = "../clap" }
    /// ```
    fn try_from_table(name: String, table: &Table) -> color_eyre::Result<Self> {
        let version = table
            .get("version")
            .context("Missing key 'version'")?
            .as_str()
            .context("Key version has invalid type")?
            .to_owned();
        let path = table
            .get("path")
            .and_then(|path| path.as_value())
            .map(ToString::to_string);
        let features = table
            .get("features")
            .map(|features| -> color_eyre::Result<Vec<String>> {
                features
                    .as_array()
                    .context("Key 'features' has invalid type")?
                    .iter()
                    .map(|feat| {
                        feat.as_str()
                            .context("Feature has invalid type")
                            .map(str::to_owned)
                    })
                    .collect()
            })
            .transpose()?
            .unwrap_or_default();
        Ok(Self {
            features,
            name,
            path,
            version,
        })
    }
}

impl TryFrom<(String, Item)> for Crate {
    type Error = color_eyre::Report;
    fn try_from((name, value): (String, Item)) -> color_eyre::Result<Self> {
        Ok(match value {
            Item::Value(Value::String(version)) => Self {
                name,
                version: version.value().to_owned(),
                ..Default::default()
            },
            Item::Value(Value::InlineTable(inline_table)) => {
                Self::try_from_table(name, &inline_table.into_table())?
            }
            Item::Value(_) | Item::None | Item::Table(_) | Item::ArrayOfTables(_) => {
                bail!("Unsupported value for crate {name}: {value:?}")
            }
        })
    }
}
