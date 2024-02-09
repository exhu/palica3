mod schema;
mod example; 
mod filter;
pub use schema::*;
pub use example::*;
pub use filter::*;

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

pub fn plain_to_rich_command(
    plain_file: Option<String>,
    toml_file: Option<String>,
) -> anyhow::Result<()> {
    let lines: Vec<String> = lines_from_file_or_stdin(plain_file)?;

    eprintln!("lines = {:?}", lines);
    let list_items: Vec<FileListItem> = lines
        .into_iter()
        .map(|line| FileListItem {
            path: line,
            tags: None,
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
    // TODO
    Ok(())
}

/// TODO delete
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
