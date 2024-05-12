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
    #[command(about = "Builds a rich file list using filter and sort settings.")]
    RichFilter(RichFilterCommand),
    #[command(about = "Builds a TOML filter from a plain text file list to match paths.")]
    PathsToFilter(PlainPathsToFilterCommand),
    #[command(about = "Prints example toml files.")]
    Example(ExampleTomlCommand),
    #[command(about = "Checks paths for being unique.")]
    CheckPaths(CheckPathsCommand),
    #[command(about = "Merges two rich lists: all tags are united for the same file.")]
    Merge(MergeCommand),
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
struct RichFilterCommand {
    #[arg(help = "Toml list file (with tags).")]
    pub toml_list: Option<String>,
    #[arg(short = 'f', help = "Toml filter file.")]
    pub toml_filter: Option<String>,
    #[arg(short = 's', help = "Toml sort file.")]
    pub toml_sort: Option<String>,
    #[arg(short = 'o', help = "Toml output file name.")]
    pub toml_file: Option<String>,
}

#[derive(clap::Args, Debug)]
struct PlainPathsToFilterCommand {
    #[arg(help = "Plain text file with line-end separated paths.")]
    pub plain_file: Option<String>,
    #[arg(short = 'o', help = "Toml file name.")]
    pub toml_file: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ExampleKind {
    List,
    Filter,
    Sort,
    Group,
}

#[derive(clap::Args, Debug)]
struct ExampleTomlCommand {
    #[arg(help = "Kind of example file")]
    pub kind: ExampleKind,
}

#[derive(clap::Args, Debug)]
struct LsCommand {
    #[arg(help = "Path to a directory.")]
    pub path: Option<String>,
}

#[derive(clap::Args, Debug)]
struct CheckPathsCommand {
    #[arg(help = "Toml file name.")]
    pub toml_file: Option<String>,
}

#[derive(clap::Args, Debug)]
struct MergeCommand {
    #[arg(help = "Toml list file (with tags) A.")]
    pub toml_list_a: String,
    #[arg(help = "Toml list file (with tags) B.")]
    pub toml_list_b: Option<String>,
    #[arg(short = 'o', help = "Merged result.")]
    pub toml_output: Option<String>,
}

fn example(kind: ExampleKind) {
    match kind {
        ExampleKind::Filter => example_filter(),
        ExampleKind::Sort => example_sorting(),
        ExampleKind::List => example_list(),
        ExampleKind::Group => example_groups(),
    }
}

fn main() -> anyhow::Result<()> {
    let parsed = Command::parse();
    eprintln!("{:?}", parsed);
    match parsed {
        Command::Ls(cmd) => {
            ls_command(cmd.path)?;
        }
        Command::PlainToRich(cmd) => plain_to_rich_command(cmd.plain_file, cmd.toml_file)?,
        Command::RichToPlain(cmd) => rich_to_plain_command(cmd.toml_file, cmd.plain_file)?,
        Command::RichFilter(cmd) => {
            rich_filter_command(cmd.toml_list, cmd.toml_filter, cmd.toml_sort, cmd.toml_file)?
        }
        Command::PathsToFilter(cmd) => {
            plain_paths_to_filter_command(cmd.plain_file, cmd.toml_file)?
        }
        Command::Example(cmd) => example(cmd.kind),
        Command::CheckPaths(cmd) => check_paths_command(cmd.toml_file)?,
        Command::Merge(cmd) => merge_command(cmd.toml_list_a, cmd.toml_list_b, cmd.toml_output)?,
    }

    Ok(())
}
