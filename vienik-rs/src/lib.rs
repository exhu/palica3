mod example;
mod filter;
mod schema;
mod sorting;
use std::os::unix::fs::MetadataExt;

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
        .iter()
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

use std::collections::HashMap;
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
