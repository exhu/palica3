/// Config files
use serde::{Deserialize, Serialize};

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
    pub tags: Option<Vec<String>>,
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
pub enum FilterType {
    Any,
    Untagged,
    AnyTagOf {
        tags: Vec<String>,
    },
    PathContains {
        value: String,
    },
    PathStartsWith {
        value: String,
    },
    PathEndsWith {
        value: String,
    },
    // TODO use proper date types, implement serde support if none
    DateTimeSpan {
        from_date_time: Option<toml::value::Datetime>,
        to_date_time: Option<toml::value::Datetime>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct FilterItem {
    pub filter: FilterType,
    /// include=false -> discard files matching this filter
    pub include: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FiltersList {
    pub filters: Vec<FilterItem>,
}
