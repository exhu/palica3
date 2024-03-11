// TODO implement filtering logic
// - source item is the file path
// - filter item either matches, or not
// - filter item has either include or exclude action
// - initial source item include state = false
// - if matches and include -> include source item (override previous state)
// - if matches and exclude -> exclude source item (override previous state)
// - if not matches -> keep previous state
// - after all the chain of filters is parsed, final decision is made
use crate::schema::*;

pub enum FileItemFilterResult {
    DoNothing,
    Include,
    Exclude,
}

pub fn process_file_item_with_filter(
    item: &FileListItem,
    filter: &FilterItem,
) -> FileItemFilterResult {
    let action = match &filter.filter {
        FilterType::Any => Some(filter.action_or_default()),
        FilterType::Tagged => {
            if item.has_tags() {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::AnyTagOf { tags } => {
            if item.has_any_tag_of(tags) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::DateFrom { date } => {
            if item.matches_date_from(date.clone()) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::DateTo { date } => {
            if item.matches_date_to(date.clone()) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::PathContains { text } => {
            if item.path_contains(text) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::PathStartsWith { text } => {
            if item.path_starts_with(text) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::PathEndsWith { text } => {
            if item.path_ends_with(text) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
        FilterType::PathList { paths } => {
            if item.path_in_list(paths) {
                Some(filter.action_or_default())
            } else {
                None
            }
        }
    };

    match action {
        None => FileItemFilterResult::DoNothing,
        Some(FilterAction::Include) => FileItemFilterResult::Include,
        Some(FilterAction::Exclude) => FileItemFilterResult::Exclude,
    }
}

pub fn filter_file_item_with_filters(item: &FileListItem, filters: &FiltersList) -> FilterAction {
    filters
        .filters
        .iter()
        .fold(
            FilterAction::Exclude,
            |prev, filter| match process_file_item_with_filter(item, &filter) {
                FileItemFilterResult::DoNothing => prev,
                FileItemFilterResult::Include => FilterAction::Include,
                FileItemFilterResult::Exclude => FilterAction::Exclude,
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_filters_list() {
        let filters = FiltersList {
            filters: Vec::new(),
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Option::None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }

    #[test]
    fn any_filter() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::Any,
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Option::None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }

    #[test]
    fn any_filter_exclude() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::Any,
                action: Some(FilterAction::Exclude),
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Option::None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }

    #[test]
    fn tagged_with_tags_filter_include() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::Tagged,
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }

    #[test]
    fn tagged_no_tags_filter_exclude() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::Tagged,
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }

    #[test]
    fn tagged_with_tags_filter_exclude() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::Tagged,
                action: Some(FilterAction::Exclude),
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }

    #[test]
    fn any_tag_of_filter_include() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::AnyTagOf {
                    tags: vec!["other".to_owned()],
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }

    #[test]
    fn any_tag_of_filter_exclude() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::AnyTagOf {
                    tags: vec!["non-existing".to_owned()],
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }

    #[test]
    fn date_from_filter_include() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::DateFrom {
                    date: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }

    #[test]
    fn date_from_filter_exclude() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::DateFrom {
                    date: chrono::NaiveDate::from_ymd_opt(1990, 1, 2).unwrap(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }

    #[test]
    fn date_to_filter_include() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::DateTo {
                    date: chrono::NaiveDate::from_ymd_opt(1990, 1, 2).unwrap(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }

    #[test]
    fn date_to_filter_exclude() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::DateTo {
                    date: chrono::NaiveDate::from_ymd_opt(1990, 1, 2).unwrap(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("abc"),
            tags: Some(vec!["tag1".to_owned(), "other".to_owned()]),
            mod_date: Some(chrono::NaiveDate::from_ymd_opt(1991, 1, 1).unwrap()),
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }
    #[test]
    fn path_starts_included() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::PathStartsWith {
                    text: "/mnt".to_owned(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("/mntabc"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }
    #[test]
    fn path_ends_included() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::PathEndsWith {
                    text: "bc".to_owned(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("/mntabc"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }
    #[test]
    fn path_contains_included() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::PathContains {
                    text: "tab".to_owned(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("/mntabc"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }
    #[test]
    fn path_startsfull_included() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::PathStartsWith {
                    text: "fullstring".to_owned(),
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("fullstring"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }
    #[test]
    fn path_list_included() {
        let filters = FiltersList {
            filters: vec![FilterItem {
                filter: FilterType::PathList {
                    paths: vec!["abc".to_owned(), "zxf".to_owned()],
                },
                action: None,
            }],
        };
        let item = FileListItem {
            path: String::from("fullstring"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
        let item = FileListItem {
            path: String::from("abc"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
        let item = FileListItem {
            path: String::from("zxf"),
            tags: None,
            mod_date: None,
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Include
        );
    }
}
