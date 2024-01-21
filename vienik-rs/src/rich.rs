use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RichFileList {
    pub files: Vec<FileListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct FileListItem {
    /// Generally absolute path, but if used as metadata (_tags.toml)
    /// in directories, then must be a relative path to the path to the
    /// metadata TOML itself.
    pub path: String,
    pub tags: Option<Vec<Tag>>,
}

impl FileListItem {
    pub fn has_tags(&self) -> bool {
        match &self.tags {
            Some(tags) => !tags.is_empty(),
            None => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

