use clap::Parser;
use std::process::ExitCode;
use palica::cli;

#[derive(Parser, Debug)]
#[command(version, about, author)]
enum Command {
    #[command(about="Add a new collection.")]
    Add(AddCommand),
    #[command(about="List collections.")]
    List(ListCommand),
    #[command(about="List collection files.")]
    Tree(TreeCommand),
    #[command(about="Remove collection.")]
    Remove(RemoveCommand),
    #[command(about="Display file path (glob) filters.")]
    Filters(FiltersCommand),
}


#[derive(clap::Args, Debug)]
struct AddCommand {
    #[arg(long = "db", help = "Database filename.")]
    pub db_file_name: String,
    pub verbose: Option<bool>,
    #[arg(help = "Do not ask for confirmations.")]
    pub yes: Option<bool>,
    #[arg(help = "Collection name.")]
    pub name: String,
    #[arg(help = "Path to a directory.")]
    pub path: String,
    #[arg(help = "Glob filter id or NULL.")]
    pub filter_id: Option<String>,
}

#[derive(clap::Args, Debug)]
struct ListCommand {
}

#[derive(clap::Args, Debug)]
struct TreeCommand {
}

#[derive(clap::Args, Debug)]
struct RemoveCommand {
}

#[derive(clap::Args, Debug)]
struct FiltersCommand {
}

fn main() -> ExitCode {
    let parsed = Command::parse();
    println!("{:?}", parsed);
    let ret = match parsed {
        Command::Add(c) => cli::collection_add(cli::CollectionAdd {
            db_file_name: c.db_file_name,
            verbose: c.verbose.unwrap_or(false),
            yes: c.yes.unwrap_or(false),
            name: c.name,
            path: c.path,
            filter_id: c.filter_id,
        }),
        _ => ExitCode::FAILURE
    };

    ret
}
