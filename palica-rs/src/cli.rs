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
use crate::coll_builder;
use crate::dblayer::read;
use crate::dblayer::write;
use crate::dblayer::DbId;
use std::path::Path;
use std::process::ExitCode;

pub struct CollectionAdd {
    pub db_file_name: String,
    pub verbose: bool,
    pub yes: bool,
    pub name: String,
    pub path: String,
    pub filter_id: DbId,
}

fn check_with_existing_paths(rdb: &read::Db, fs_path: &str) {
    let cols = rdb
        .collections_by_fs_path(fs_path)
        .expect("check_with_existing_paths: Failed to read db.");
    if cols.is_empty() == false {
        println!("WARNING: there are existing collections with the same path '{fs_path}':");
        for c in cols {
            println!("{}, {}", c.id, c.coll_name);
        }
    }
}

pub fn collection_add(args: CollectionAdd) -> ExitCode {
    // TODO proper error handling
    // TODO check yes for create new db
    // TODO check for existing col
    let conn = write::open_and_make(&args.db_file_name).unwrap();
    let rdb = read::Db::new(&conn).unwrap();
    let norm_path = crate::fslayer::read::normalized_abspath(&args.path);
    check_with_existing_paths(&rdb, &norm_path);

    let mut filter = rdb.glob_filter_by_id(args.filter_id).unwrap();
    let mut wdb = write::Db::new(&conn).unwrap();
    let col = coll_builder::new_collection(
        &mut wdb,
        &args.name,
        &Path::new(&norm_path),
        args.filter_id,
        &mut filter,
        &|e| {
            println!("new entry {:?}", &e);
        },
    );
    if col.is_err() {
        println!("ERROR: {}", col.err().unwrap());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn collection_list_err(db_file_name: &str) -> anyhow::Result<()> {
    let conn = read::open_existing(db_file_name)?;
    let rdb = read::Db::new(&conn)?;
    let cols = rdb.enum_collections()?;
    for col in cols {
        println!("{},{},{}", col.id, col.coll_name, col.fs_path);
    }
    Ok(())
}

pub fn collection_list(db_file_name: &str) -> ExitCode {
    match collection_list_err(db_file_name) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("ERROR: {e}");
            ExitCode::FAILURE
        }
    }
}
