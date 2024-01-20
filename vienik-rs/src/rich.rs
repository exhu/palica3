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
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

