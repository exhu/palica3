use crate::*;

pub fn example_filter() {
    let mut filters = FiltersList {
        filters: Vec::new(),
    };
    filters.filters.push(FilterItem {
        filter: FilterType::Untagged,
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
            value: "myname".to_owned(),
        },
        action: Option::Some(FilterAction::Exclude),
    });

    use toml::value::Datetime;

    filters.filters.push(FilterItem {
        filter: FilterType::DateTimeSpan {
            from_date_time: Option::Some(Datetime::from(toml::value::Date { year: 1980, month: 12, day: 31})),
            to_date_time: Option::None,
        },
        action: Option::Some(FilterAction::Exclude),
    });

    let serialized = toml::to_string(&filters).expect("failed to generate toml");
    println!("{serialized}");
}
