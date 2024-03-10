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
    /// Turns out that modification date is the only attribute preserved when
    /// copying files.
    pub mod_date: Option<chrono::NaiveDate>,
}

impl FileListItem {
    pub fn has_tags(&self) -> bool {
        match &self.tags {
            Some(tags) => !tags.is_empty(),
            None => false,
        }
    }

    pub fn has_any_tag_of(&self, other_tags: &[String]) -> bool {
        match &self.tags {
            Some(tags) => {
                for t in tags {
                    for o in other_tags {
                        if t == o {
                            return true;
                        }
                    }
                }
            }
            None => {}
        }
        false
    }

    pub fn matches_date_from(&self, from_date: chrono::NaiveDate) -> bool {
        match self.mod_date {
            Some(mod_date) => mod_date >= from_date,
            None => false,
        }
    }

    pub fn matches_date_to(&self, to_date: chrono::NaiveDate) -> bool {
        match self.mod_date {
            Some(mod_date) => mod_date <= to_date,
            None => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum FilterType {
    Any,
    Tagged,
    AnyTagOf { tags: Vec<String> },
    PathContains { value: String },
    PathStartsWith { value: String },
    PathEndsWith { value: String },
    DateFrom { date: chrono::NaiveDate },
    DateTo { date: chrono::NaiveDate },
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

impl FilterItem {
    /// unwrap action, or return default action
    pub fn action_or_default(&self) -> FilterAction {
        match &self.action {
            Some(a) => a.clone(),
            None => FilterAction::Include,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FiltersList {
    pub filters: Vec<FilterItem>,
}
