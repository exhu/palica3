use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, author)]
enum Command {
    #[command(about = "Calls 'fd -a', convenient command, use fd for more.")]
    Ls(LsCommand),
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
        Command::Ls(cmd) => println!("ls command! {:?}", cmd),
    }
    Ok(())
}
