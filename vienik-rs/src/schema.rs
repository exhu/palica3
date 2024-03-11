/// Config files
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RichFileList {
    pub files: Vec<FileListItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileListItem {
    /// Generally absolute path, but if used as metadata (_tags.toml)
    /// in directories, then must be a relative path to the path to the
    /// metadata TOML itself.
    pub path: String,
    pub tags: Option<Vec<String>>,
    /// Turns out that modification date is the only attribute preserved when
    /// copying files.
    pub mod_date: Option<chrono::NaiveDate>,
    pub size: Option<u64>,
}

impl FileListItem {
    pub fn tags_count(&self) -> usize {
        match &self.tags {
            Some(tags) => tags.len(),
            None => 0,
        }
    }
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

    pub fn path_contains(&self, text: &str) -> bool {
        self.path.contains(text)
    }

    pub fn path_starts_with(&self, text: &str) -> bool {
        self.path.starts_with(text)
    }

    pub fn path_ends_with(&self, text: &str) -> bool {
        self.path.ends_with(text)
    }

    pub fn path_in_list(&self, list: &[String]) -> bool {
        list.contains(&self.path)
    }
}

#[derive(Serialize, Deserialize)]
pub enum FilterType {
    Any,
    Tagged,
    AnyTagOf { tags: Vec<String> },
    DateFrom { date: chrono::NaiveDate },
    DateTo { date: chrono::NaiveDate },
    PathContains { text: String },
    PathStartsWith { text: String },
    PathEndsWith { text: String },
    PathList { paths: Vec<String> },
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

#[derive(Serialize, Deserialize)]
pub struct SortingCommands {
    pub sort: Vec<SortingCommand>,
}

#[derive(Serialize, Deserialize)]
pub struct SortingCommand {
    pub ascending: Option<bool>,
    pub criteria: SortingCriteria,
}

use std::cmp::Ordering;
impl SortingCommand {
    fn compare_dates(a: &Option<chrono::NaiveDate>, b: &Option<chrono::NaiveDate>) -> Ordering {
        if a.is_none() && b.is_none() {
            return Ordering::Equal;
        }
        if a.is_none() && b.is_some() {
            return Ordering::Greater;
        }
        if b.is_none() && a.is_some() {
            return Ordering::Less;
        }

        let a = a.as_ref().unwrap();
        let b = b.as_ref().unwrap();

        a.cmp(b)
    }

    fn compare_size(a: &Option<u64>, b: &Option<u64>) -> Ordering {
        if a.is_none() && b.is_none() {
            return Ordering::Equal;
        }

        if a.is_none() && b.is_some() {
            return Ordering::Greater;
        }

        if b.is_none() && a.is_some() {
            return Ordering::Less;
        }

        let a = a.unwrap();
        let b = b.unwrap();

        a.cmp(&b)
    }

    pub fn compare(&self, a: &FileListItem, b: &FileListItem) -> Ordering {
        match self.criteria {
            SortingCriteria::Date => Self::compare_dates(&a.mod_date, &b.mod_date),
            SortingCriteria::PathName => a.path.cmp(&b.path),
            SortingCriteria::Size => Self::compare_size(&a.size, &b.size),
            SortingCriteria::TagsCount => a.tags_count().cmp(&b.tags_count()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SortingCriteria {
    PathName,
    Date,
    Size,
    TagsCount,
}
