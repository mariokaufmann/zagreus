use crate::error::ZagreusError;
use std::fs::DirEntry;
use std::path::Path;

#[derive(Serialize)]
#[serde(untagged)]
pub enum DirEntryNode {
    File {
        name: String,
    },
    Dir {
        name: String,
        children: Vec<DirEntryNode>,
    },
}

pub fn traverse(path: &Path) -> Result<Vec<DirEntryNode>, ZagreusError> {
    let files = std::fs::read_dir(path)?;
    Ok(files
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let child_path = entry.path();
            let child_name = get_filename(entry)?;
            match child_path.is_file() {
                true => Ok(DirEntryNode::File { name: child_name }),
                false => Ok(DirEntryNode::Dir {
                    name: child_name,
                    children: traverse(&child_path)?,
                }),
            }
        })
        .filter_map(|entry: Result<DirEntryNode, ZagreusError>| entry.ok())
        .collect())
}

fn get_filename(entry: DirEntry) -> Result<String, ZagreusError> {
    match entry.file_name().into_string() {
        Ok(filename) => Ok(filename),
        Err(_) => Err(ZagreusError::from(format!(
            "Failed to process filename: {:?}",
            entry.file_name()
        ))),
    }
}
