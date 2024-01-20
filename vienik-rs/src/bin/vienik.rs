use clap::Parser;
use vienik_rs::*;

#[derive(Parser, Debug)]
#[command(version, about, author)]
enum Command {
    #[command(about = "Calls 'fd -a', convenient command, use fd for more.")]
    Ls(LsCommand),
    #[command(about = "Builds a TOML from a plain text file list.")]
    PlainToRich(PlainToRichCommand),
    #[command(about = "Builds a plain text file list from TOML.")]
    RichToPlain(RichToPlainCommand),
}

#[derive(clap::Args, Debug)]
struct PlainToRichCommand {
    #[arg(help = "Plain text file with line-end separated paths.")]
    pub plain_file: Option<String>,
    #[arg(short = 'o', help = "Toml file name.")]
    pub toml_file: Option<String>,
}

#[derive(clap::Args, Debug)]
struct RichToPlainCommand {
    #[arg(help = "Toml file name.")]
    pub toml_file: Option<String>,
    #[arg(short = 'o', help = "Plain text file with line-end separated paths.")]
    pub plain_file: Option<String>,
}

#[derive(clap::Args, Debug)]
struct LsCommand {
    #[arg(help = "Path to a directory.")]
    pub path: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let parsed = Command::parse();
    println!("{:?}", parsed);
    match parsed {
        Command::Ls(cmd) => {
            println!("{:?}", cmd);
            ls_command(cmd.path)?;
        }
        Command::PlainToRich(cmd) => println!("command! {:?}", cmd),
        Command::RichToPlain(cmd) => println!("command! {:?}", cmd),
    }
    Ok(())
}
