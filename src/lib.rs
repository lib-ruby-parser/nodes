extern crate serde;
extern crate serde_yaml;

use std::path::Path;

mod field;
mod field_type;
mod node;
mod options;

pub use field::Field;
pub use field_type::FieldType;
pub use node::Node;
pub use options::Options;

#[derive(Debug)]
pub enum Error {
    CantFindYaml,
    FailedToReadYaml(String),
    YamlHasInvalidFormat(String),
    FailedToCreateTargetDir(String),
    FailedToWriteFile(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CantFindYaml => write!(f, "Error::CantFindYaml()"),
            Error::FailedToReadYaml(msg) => write!(f, "Error::FailedToReadYaml({})", msg),
            Error::YamlHasInvalidFormat(msg) => write!(f, "Error::YamlHasInvalidFormat({})", msg),
            Error::FailedToCreateTargetDir(msg) => {
                write!(f, "Error::FailedToCreateTargetDir({})", msg)
            }
            Error::FailedToWriteFile(msg) => write!(f, "Error::FailedToWriteFile({})", msg),
        }
    }
}

impl std::error::Error for Error {}

pub fn nodes() -> Result<Vec<Node>, Error> {
    let path = Path::new(file!())
        .parent()
        .ok_or_else(|| Error::CantFindYaml)?
        .join("nodes.yaml")
        .to_str()
        .ok_or_else(|| Error::CantFindYaml)?
        .to_owned();

    let f = std::fs::File::open(&path).map_err(|err| Error::FailedToReadYaml(err.to_string()))?;
    let nodes: Vec<Node> =
        serde_yaml::from_reader(f).map_err(|err| Error::YamlHasInvalidFormat(err.to_string()))?;
    Ok(nodes)
}

pub fn generate_nodes(options: &Options) -> Result<(), Error> {
    let nodes = nodes()?;

    std::fs::create_dir_all(&options.target_dir)
        .map_err(|err| Error::FailedToCreateTargetDir(err.to_string()))?;

    for node in nodes.iter() {
        let code = node.code(options);
        std::fs::write(
            &format!("{}/{}.rs", options.target_dir, node.filename),
            code,
        )
        .map_err(|err| {
            Error::FailedToWriteFile(format!(
                "failed to write {}/{}.rs: {}",
                options.target_dir, node.filename, err
            ))
        })?;
    }

    Ok(())
}

pub fn generate_mod(options: &Options) -> Result<(), Error> {
    let mod_content = nodes()?
        .iter()
        .map(|node| {
            format!(
                "mod {mod_name};\npub use {mod_name}::{struct_name};\n",
                mod_name = node.filename,
                struct_name = node.struct_name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(&format!("{}/mod.rs", options.target_dir), &mod_content).map_err(|err| {
        Error::FailedToWriteFile(format!(
            "Failed to write {}/mod.rs: {}",
            options.target_dir, err
        ))
    })?;

    Ok(())
}
