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

pub fn filter_file_item_with_filter(item: &FileListItem, filter: &FilterItem) -> FileItemFilterResult {
    let action = match filter.filter
    {
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
    todo!();
}
