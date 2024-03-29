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
