/*
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use clap::Parser;
use std::process::ExitCode;
use palica::cli;
use palica::dblayer;

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
    #[arg(help = "Glob filter id.")]
    pub filter_id: Option<i64>,
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
            filter_id: c.filter_id.unwrap_or(dblayer::DEFAULT_FILTER_ID),
        }),
        _ => ExitCode::FAILURE
    };

    ret
}
