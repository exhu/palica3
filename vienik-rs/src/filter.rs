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
    let action = match filter.filter {
        FilterType::Any => Some(filter.action.clone().unwrap_or(FilterAction::Include)),
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
}
