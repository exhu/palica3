pub mod rich;
pub use rich::*;



pub fn ls_command(path: Option<String>) -> anyhow::Result<()>{
    use std::process::Command;
    let mut cmd = Command::new("fd");
    cmd.arg("-a").arg(".");
    match path {
        Some(p) => { cmd.arg(p); },
        None =>{},
    };

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}

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
