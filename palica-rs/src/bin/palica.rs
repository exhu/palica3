/*
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use clap::Parser;
use palica::cli;
use palica::dblayer;

#[derive(Parser, Debug)]
#[command(version, about, author)]
enum Command {
    #[command(about = "Initialize a new database.")]
    CreateDb(CreateDbCommand),
    #[command(about = "Add a new collection.")]
    Add(AddCommand),
    #[command(about = "List collections.")]
    List(ListCommand),
    #[command(about = "List collection files.")]
    Tree(TreeCommand),
    #[command(about = "Remove collection.")]
    Remove(RemoveCommand),
    #[command(about = "Display file path (glob) filters.")]
    Filters(FiltersCommand),
    #[command(about = "Read tags from files and sidecars.")]
    ReadTags(ReadTagsCommand),
    #[command(about = "Write tags from db to sidecars.")]
    WriteTags(WriteTagsCommand),
}

#[derive(clap::Args, Debug)]
struct AddCommand {
    #[arg(long = "db", help = "Database filename.")]
    pub db_file_name: String,
    #[arg(long = "verbose", short = 'v', help = "Print more info.")]
    pub verbose: bool,
    #[arg(long = "yes", help = "Do not ask for confirmations.")]
    pub yes: bool,
    #[arg(
        long = "dry",
        short = 'n',
        help = "Only display what would be done, no modifications."
    )]
    pub dry: bool,
    #[arg(help = "Collection name.")]
    pub name: String,
    #[arg(help = "Path to a directory.")]
    pub path: String,
    #[arg(help = "Glob filter id.")]
    pub filter_id: Option<i64>,
}

#[derive(clap::Args, Debug)]
struct ListCommand {
    #[arg(long = "db", help = "Database filename.")]
    pub db_file_name: String,
}

#[derive(clap::Args, Debug)]
struct TreeCommand {
    #[arg(long = "db", help = "Database filename.")]
    pub db_file_name: String,
    #[arg(help = "Collection name.")]
    pub name: String,
}

#[derive(clap::Args, Debug)]
struct RemoveCommand {
    #[arg(long = "db", help = "Database filename.")]
    pub db_file_name: String,
    #[arg(help = "Collection name.")]
    pub name: String,
}

#[derive(clap::Args, Debug)]
struct FiltersCommand {}

#[derive(clap::Args, Debug)]
struct ReadTagsCommand {}

#[derive(clap::Args, Debug)]
struct WriteTagsCommand {}

#[derive(clap::Args, Debug)]
struct CreateDbCommand {
    #[arg(help = "Database file.")]
    pub db_file_name: String,
}

fn main() -> anyhow::Result<()> {
    let parsed = Command::parse();
    println!("{:?}", parsed);
    match parsed {
        Command::Add(c) => cli::collection_add(cli::CollectionAdd {
            db_file_name: c.db_file_name,
            verbose: c.verbose,
            yes: c.yes,
            name: c.name,
            path: c.path,
            filter_id: c.filter_id.unwrap_or(dblayer::DEFAULT_FILTER_ID),
            dry: c.dry,
        })?,
        Command::List(c) => cli::collection_list(&c.db_file_name)?,
        Command::Tree(c) => cli::collection_tree(&c.db_file_name, &c.name)?,
        Command::CreateDb(c) => cli::create_db(&c.db_file_name)?,
        Command::Remove(c) => cli::collection_remove(&c.db_file_name, &c.name)?,
        // TODO
        _ => todo!(),
    };
    Ok(())
}
