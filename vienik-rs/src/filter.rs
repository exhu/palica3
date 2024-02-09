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

pub fn filter_file_item_with_filter(item: &FileListItem, filter: &FilterItem) -> FilterAction {
    let mut action = match filter.filter
    {
        FilterType::Any => filter.action.clone().unwrap_or(FilterAction::Include),
        // TODO
        _ => FilterAction::Exclude,
    };

    action
}

pub fn filter_file_item_with_filters(item: &FileListItem, filters: &FiltersList) -> FilterAction {
    todo!();
}
