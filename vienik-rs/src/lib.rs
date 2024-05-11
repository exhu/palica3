mod example;
mod filter;
mod schema;
mod sorting;
use std::collections::{HashMap, HashSet};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use chrono::Local;
pub use example::*;
pub use filter::*;
pub use schema::*;
pub use sorting::*;

pub fn ls_command(path: Option<String>) -> anyhow::Result<()> {
    use std::process::Command;
    let mut cmd = Command::new("fd");
    cmd.arg("-a").arg(".");
    match path {
        Some(p) => {
            cmd.arg(p);
        }
        None => {}
    };

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}

fn lines_from_file_or_stdin(plain_file: Option<String>) -> anyhow::Result<Vec<String>> {
    let lines: Vec<String> = match plain_file {
        Some(path) => std::fs::read_to_string(path)?
            .lines()
            .map(|s| s.to_owned())
            .collect(),
        None => std::io::stdin()
            .lines()
            .into_iter()
            .collect::<Result<Vec<String>, _>>()?,
    };
    Ok(lines)
}

fn string_from_file_or_stdin(file: Option<String>) -> anyhow::Result<String> {
    use std::io::Read;
    let mut text = String::new();
    match file {
        Some(path) => {
            text = std::fs::read_to_string(path)?;
        }
        None => {
            std::io::stdin().read_to_string(&mut text)?;
        }
    };
    Ok(text)
}

fn string_to_file_or_stdout(text: &str, filename: Option<String>) -> anyhow::Result<()> {
    use std::io::Write;
    match filename {
        Some(f) => {
            let mut file = std::fs::File::create(f)?;
            file.write_all(&text.as_bytes())?;
        }
        None => std::io::stdout().write_all(&text.as_bytes())?,
    }
    Ok(())
}

fn file_mod_date(path: &str) -> Option<chrono::NaiveDate> {
    let meta = std::path::Path::new(path).metadata();
    match meta {
        Ok(meta) => match meta.modified() {
            Ok(file_date) => {
                let dt_utc: chrono::DateTime<chrono::Utc> = file_date.into();
                let dt_local: chrono::DateTime<Local> = dt_utc.into();
                Some(chrono::NaiveDate::from(dt_local.date_naive()))
            }
            _ => None,
        },
        _ => None,
    }
}

fn file_size(path: &str) -> Option<u64> {
    let meta = std::path::Path::new(path).metadata();
    match meta {
        Ok(meta) => Some(meta.size()),
        _ => None,
    }
}

pub fn plain_to_rich_command(
    plain_file: Option<String>,
    toml_file: Option<String>,
) -> anyhow::Result<()> {
    let lines: Vec<String> = lines_from_file_or_stdin(plain_file)?;

    eprintln!("lines = {:?}", lines);
    let list_items: Vec<FileListItem> = lines
        .into_iter()
        .map(|line| {
            let date = file_mod_date(&line);
            let size = file_size(&line);

            FileListItem {
                path: line,
                tags: None,
                mod_date: date,
                size: size,
            }
        })
        .collect();
    let rich = RichFileList { files: list_items };

    let serialized = toml::to_string(&rich)?;
    string_to_file_or_stdout(&serialized, toml_file)?;
    Ok(())
}

pub fn rich_to_plain_command(
    toml_file: Option<String>,
    plain_file: Option<String>,
) -> anyhow::Result<()> {
    let toml_string = string_from_file_or_stdin(toml_file)?;
    let rich: RichFileList = toml::from_str(&toml_string)?;
    let lines = rich
        .files
        .into_iter()
        .map(|f| format!("{}\n", &f.path))
        .collect::<Vec<String>>();
    string_to_file_or_stdout(&lines.join(""), plain_file)?;
    Ok(())
}

pub fn rich_filter_command(
    toml_list: Option<String>,
    toml_filter: Option<String>,
    toml_sort: Option<String>,
    toml_file: Option<String>,
) -> anyhow::Result<()> {
    let toml_string = string_from_file_or_stdin(toml_list)?;
    let paths: RichFileList = toml::from_str(&toml_string)?;

    let filters: FiltersList;
    match toml_filter {
        Some(toml_filter) => {
            let toml_string = std::fs::read_to_string(toml_filter)?;
            filters = toml::from_str(&toml_string)?;
        }
        None => {
            filters = FiltersList {
                filters: vec![FilterItem {
                    filter: FilterType::Any,
                    action: None,
                }],
            }
        }
    }

    let sorting: SortingCommands;
    match toml_sort {
        Some(toml_sort) => {
            let toml_string = std::fs::read_to_string(toml_sort)?;
            sorting = toml::from_str(&toml_string)?;
        }
        None => sorting = SortingCommands { sort: Vec::new() },
    }

    let mut filtered_paths = filter_filelist_with_filters(paths.files.into_iter(), &filters);
    sort_filelist(&mut filtered_paths, &sorting.sort);
    let filtered_list = RichFileList {
        files: filtered_paths,
    };
    let serialized = toml::to_string(&filtered_list)?;
    string_to_file_or_stdout(&serialized, toml_file)?;
    Ok(())
}

pub fn plain_paths_to_filter_command(
    plain_file: Option<String>,
    toml_file: Option<String>,
) -> anyhow::Result<()> {
    let lines: Vec<String> = lines_from_file_or_stdin(plain_file)?;

    eprintln!("lines = {:?}", lines);

    let filters = FiltersList {
        filters: vec![FilterItem {
            filter: FilterType::PathList { paths: lines },
            action: None,
        }],
    };

    let serialized = toml::to_string(&filters)?;
    string_to_file_or_stdout(&serialized, toml_file)?;
    Ok(())
}

fn rich_from_file(path: &Path) -> anyhow::Result<RichFileList> {
    let text = std::fs::read_to_string(path)?;
    let paths: RichFileList = toml::from_str(&text)?;
    Ok(paths)
}

pub fn check_paths_command(toml_file: Option<String>) -> anyhow::Result<()> {
    let toml_string = string_from_file_or_stdin(toml_file)?;
    let paths: RichFileList = toml::from_str(&toml_string)?;
    let mut paths_map = HashMap::<std::path::PathBuf, String>::new();
    for item in paths.files {
        let path_buf = std::path::PathBuf::from(&item.path)
            .canonicalize()
            .expect(&format!("failed to get canonical path for {}", item.path));

        if paths_map.contains_key(&path_buf) {
            eprintln!(
                "Duplicate path found for '{}' first mentioned as '{}':",
                path_buf.to_string_lossy(),
                paths_map[&path_buf]
            );
            println!("{}", item.path);
        } else {
            paths_map.insert(path_buf, item.path);
        }
    }
    eprintln!("Paths checked: {}", paths_map.len());
    Ok(())
}

fn merge_tags(
    tags_a: Option<HashSet<String>>,
    tags_b: Option<HashSet<String>>,
) -> Option<HashSet<String>> {
    match tags_a {
        None => match tags_b {
            None => None,
            Some(_) => tags_b,
        },
        Some(set_a) => match tags_b {
            None => Some(set_a),
            Some(set_b) => Some(set_a.union(&set_b).map(String::clone).collect()),
        },
    }
}

fn merge_rich_list_dupes(list_a: Vec<FileListItem>) -> HashMap<String, FileListItem> {
    let mut result = HashMap::<String, FileListItem>::new();

    for item in list_a {
        match result.get(&item.path) {
            Some(old) => {
                let mut new_item = item.clone();
                new_item.tags = merge_tags(item.tags, old.tags.clone());
                result.insert(item.path.clone(), new_item);
            }
            None => {
                result.insert(item.path.clone(), item);
            }
        }
    }

    result
}

fn merge_rich_lists(list_a: RichFileList, list_b: Option<RichFileList>) -> RichFileList {
    match list_b {
        None => RichFileList {
            files: merge_rich_list_dupes(list_a.files)
                .into_iter()
                .map(|v| v.1)
                .collect(),
        },
        Some(list_b) => {
            let concated = list_a
                .files
                .into_iter()
                .chain(list_b.files.into_iter())
                .collect();

            RichFileList {
                files: merge_rich_list_dupes(concated)
                    .into_iter()
                    .map(|v| v.1)
                    .collect(),
            }
        }
    }
}

pub fn merge_command(
    toml_list_a: String,
    toml_list_b: Option<String>,
    toml_output: Option<String>,
) -> anyhow::Result<()> {
    let list_a = rich_from_file(&PathBuf::from(&toml_list_a))?;
    let list_b = match toml_list_b {
        None => None,
        Some(list_b_path) => Some(rich_from_file(&PathBuf::from(&list_b_path))?),
    };
    let merged = merge_rich_lists(list_a, list_b);
    let serialized = toml::to_string(&merged)?;
    string_to_file_or_stdout(&serialized, toml_output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FileListItem, RichFileList};

    #[test]
    fn test_merge_list() {
        let list_a = RichFileList {
            files: vec![
                FileListItem::new("duplicate".to_owned()),
                FileListItem::new_with_tags(
                    "duplicate".to_owned(),
                    vec!["cat".to_owned(), "dog".to_owned()],
                ),
                FileListItem::new_with_tags("duplicate".to_owned(), vec!["fox".to_owned()]),
            ],
        };
        let merged = merge_rich_lists(list_a, None);
        assert_eq!(merged.files.len(), 1);
        assert_eq!(
            merged.files.get(0).unwrap().tags,
            Some(HashSet::from([
                "fox".to_owned(),
                "cat".to_owned(),
                "dog".to_owned()
            ]))
        );
    }
}
