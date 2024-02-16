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
    DateSpan {
        from_date: Option<chrono::NaiveDate>,
        to_date: Option<chrono::NaiveDate>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum FilterAction {
    Include,
    Exclude,
}

#[derive(Serialize, Deserialize)]
pub struct FilterItem {
    pub filter: FilterType,
    /// default action is Include if None
    pub action: Option<FilterAction>,
}

#[derive(Serialize, Deserialize)]
pub struct FiltersList {
    pub filters: Vec<FilterItem>,
}
