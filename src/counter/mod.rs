use anyhow::Result;

use miden_objects::{assembly::Library, utils::Deserializable};

const COUNTER_MASL: &[u8] = include_bytes!("./generated/library.masl");

/// Returns the deserialized `Library` for the `counter` module.
pub fn library() -> Result<Library> {
    let lib: Library = Library::read_from_bytes(COUNTER_MASL)?;
    Ok(lib)
}
