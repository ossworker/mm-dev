use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use typst::{
    Library,
    diag::{FileError, FileResult},
    foundations::Bytes,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};

pub struct TypstWorld {
    /// content source
    pub source: Source,
    /// standard library
    library: LazyHash<Library>,
    /// metadata about the all known fonts
    book: LazyHash<FontBook>,
    /// all known fonts
    fonts: Vec<Font>,

    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,
}

#[derive(Debug, Clone)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
    }
}
