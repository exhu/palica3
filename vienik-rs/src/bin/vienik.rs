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
    #[command(
        about = "Merges one or two rich lists: all tags are united for the same file, duplicate entries are merged keeping all the tags."
    )]
    Merge(MergeCommand),
    #[command(
        about = "Merges two rich lists as in 'merge' command, but only files in both lists are kept."
    )]
    Intersect(IntersectCommand),
    #[command(
        about = "Merges two rich lists as in 'merge' command, but only second list's files are kept."
    )]
    Update(UpdateCommand),
    #[command(about = "Collectes used tags from the rich list.")]
    DumpTags(DumpTagsCommand),
    #[command(about = "Compares two rich lists, displays difference as toml.")]
    Compare(CompareCommand),
    #[command(about = "Merges tags for files matching suffix groups.")]
    MergeTagsInGroups(MergeTagsInGroupsCommand),
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

#[derive(clap::Args, Debug)]
struct IntersectCommand {
    #[arg(help = "Toml list file (with tags) A.")]
    pub toml_list_a: String,
    #[arg(help = "Toml list file (with tags) B.")]
    pub toml_list_b: String,
    #[arg(short = 'o', help = "Merged (via intersection) result.")]
    pub toml_output: Option<String>,
}

#[derive(clap::Args, Debug)]
struct UpdateCommand {
    #[arg(help = "Old toml list file (with tags) A.")]
    pub toml_list_a: String,
    #[arg(help = "New toml list file (with tags) B.")]
    pub toml_list_b: String,
    #[arg(
        short = 'o',
        help = "Merged (via intersection, then union) result. Files not in list B will be removed."
    )]
    pub toml_output: Option<String>,
}

#[derive(clap::Args, Debug)]
struct CompareCommand {
    #[arg(help = "Old toml list file (with tags) A.")]
    pub toml_list_a: String,
    #[arg(help = "New toml list file (with tags) B.")]
    pub toml_list_b: String,
    #[arg(short = 'o', help = "Formatted result.")]
    pub toml_output: Option<String>,
    #[arg(short = 'i', help = "Ignore tags.", default_value_t = false)]
    pub ignore_tags: bool,
}

#[derive(clap::Args, Debug)]
struct DumpTagsCommand {
    #[arg(help = "Toml file name.")]
    pub toml_file: Option<String>,
    #[arg(short = 'o', help = "Plain text file with line-end separated tags.")]
    pub plain_file: Option<String>,
}

#[derive(clap::Args, Debug)]
struct MergeTagsInGroupsCommand {
    #[arg(help = "Toml list file (with tags).")]
    pub toml_list_a: String,
    #[arg(help = "Groups list file.")]
    pub toml_groups: Option<String>,
    #[arg(short = 'o', help = "Merged list result.")]
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
        // TODO
        Command::Intersect(_cmd) => eprintln!("not implemented"),
        // TODO
        Command::Update(_cmd) => eprintln!("not implemented"),
        // TODO
        Command::DumpTags(_cmd) => eprintln!("not implemented"),
        // TODO
        Command::Compare(_cmd) => eprintln!("not implemented"),
        // TODO
        Command::MergeTagsInGroups(_cmd) => eprintln!("not implemented"),
    }

    Ok(())
}
