pub mod rich;
pub use rich::*;

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

pub fn plain_to_rich_command(
    plain_file: Option<String>,
    toml_file: Option<String>,
) -> anyhow::Result<()> {
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

    eprintln!("lines = {:?}", lines);
    let list_items: Vec<FileListItem> = lines
        .into_iter()
        .map(|line| FileListItem {
            path: line,
            tags: None,
        })
        .collect();
    let rich = RichFileList { files: list_items };
    
    // TODO toml

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
