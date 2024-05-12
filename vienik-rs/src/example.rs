use chrono::NaiveDate;

use crate::*;

pub fn example_filter() {
    let mut filters = FiltersList {
        filters: Vec::new(),
    };
    filters.filters.push(FilterItem {
        filter: FilterType::Tagged,
        action: Option::None,
    });

    filters.filters.push(FilterItem {
        filter: FilterType::AnyTagOf {
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
        },
        action: Option::Some(FilterAction::Include),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::PathContains {
            text: "myname".to_owned(),
        },
        action: Option::Some(FilterAction::Exclude),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::PathStartsWith {
            text: "myname".to_owned(),
        },
        action: Option::Some(FilterAction::Exclude),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::PathEndsWith {
            text: "myname".to_owned(),
        },
        action: Option::Some(FilterAction::Exclude),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::PathList {
            paths: vec!["first-path".to_owned(), "second".to_owned()],
        },
        action: Option::Some(FilterAction::Exclude),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::Accessible,
        action: None,
    });

    use chrono::NaiveDate;
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

    filters.filters.push(FilterItem {
        filter: FilterType::DateFrom { date: d },
        action: Option::Some(FilterAction::Exclude),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::DateTo { date: d },
        action: Option::Some(FilterAction::Exclude),
    });

    let serialized = toml::to_string(&filters).expect("failed to generate toml");
    println!("{serialized}");
}

pub fn example_sorting() {
    let sorting = SortingCommands {
        sort: vec![
            SortingCommand {
                ascending: Some(false),
                criteria: SortingCriteria::PathName,
            },
            SortingCommand {
                ascending: None,
                criteria: SortingCriteria::Date,
            },
            SortingCommand {
                ascending: None,
                criteria: SortingCriteria::Size,
            },
            SortingCommand {
                ascending: None,
                criteria: SortingCriteria::TagsCount,
            },
        ],
    };
    let serialized = toml::to_string(&sorting).expect("failed to generate toml");
    println!("{serialized}");
}

pub fn example_list() {
    let mut item = FileListItem::new_with_tags(
        "/original".to_owned(),
        vec!["banana".to_owned(), "apple".to_owned()],
    );
    item.mod_date = Some(NaiveDate::from_ymd_opt(1991, 06, 07).unwrap());
    item.size = Some(123456);

    let list = RichFileList {
        files: vec![
            FileListItem::new("duplicate".to_owned()),
            FileListItem::new_with_tags(
                "duplicate".to_owned(),
                vec!["cat".to_owned(), "dog".to_owned()],
            ),
            FileListItem::new_with_tags("duplicate".to_owned(), vec!["fox".to_owned()]),
            item,
        ],
    };
    let serialized = toml::to_string(&list).expect("failed to generate toml");
    println!("{serialized}");
}

pub fn example_groups() {
    let groups = SuffixGroups {
        suffix_groups: vec![
            SuffixGroup::new(vec![".jpg".into(), ".orf".into()]),
            SuffixGroup::new(vec![".jpg".into(), ".cr2".into()]),
        ],
    };

    let serialized = toml::to_string(&groups).expect("failed to generate toml");
    println!("{serialized}");
}
