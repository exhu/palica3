use crate::*;
use std::cmp::Ordering;

pub fn sort_filelist(list: &mut [FileListItem], sorting_commands: &[SortingCommand]) {
    list.sort_by(|a, b| {
        let mut ordering = Ordering::Equal;
        for s in sorting_commands {
            ordering = s.compare(a, b);
            if !ordering.is_eq() {
                break;
            }
        }
        ordering
    });
}
