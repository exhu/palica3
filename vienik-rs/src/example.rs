use crate::*;

pub fn example_filter() {
    let mut filters = FiltersList {
        filters: Vec::new(),
    };
    filters.filters.push(FilterItem {
        filter: FilterType::Untagged,
        include: Option::None,
    });

    filters.filters.push(FilterItem {
        filter: FilterType::AnyTagOf {
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
        },
        include: Option::Some(true),
    });

    filters.filters.push(FilterItem {
        filter: FilterType::PathContains {
            value: "myname".to_owned(),
        },
        include: Option::Some(false),
    });

    use toml::value::Datetime;

    filters.filters.push(FilterItem {
        filter: FilterType::DateTimeSpan {
            from_date_time: Option::Some(Datetime::from(toml::value::Date { year: 1980, month: 12, day: 31})),
            to_date_time: Option::None,
        },
        include: Option::Some(false),
    });

    let serialized = toml::to_string(&filters).expect("failed to generate toml");
    println!("{serialized}");
}
