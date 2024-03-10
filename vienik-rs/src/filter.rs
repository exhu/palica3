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
        } //if tags.cont,
        // TODO
        _ => None,
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
        };
        assert_eq!(
            filter_file_item_with_filters(&item, &filters),
            FilterAction::Exclude
        );
    }
}
