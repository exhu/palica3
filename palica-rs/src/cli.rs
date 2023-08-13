use anyhow::Context;

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

pub struct CollectionAdd {
    pub db_file_name: String,
    pub verbose: bool,
    pub yes: bool,
    pub name: String,
    pub path: String,
    pub filter_id: DbId,
}

fn check_with_existing_paths(rdb: &read::Db, fs_path: &str) -> anyhow::Result<()> {
    let cols = rdb
        .collections_by_fs_path(fs_path)
        .with_context(|| "check_with_existing_paths: Failed to read db.")?;
    if cols.is_empty() == false {
        println!("WARNING: there are existing collections with the same path '{fs_path}':");
        for c in cols {
            println!("{}, {}", c.id, c.coll_name);
        }
    }
    Ok(())
}

pub fn collection_add(args: CollectionAdd) -> anyhow::Result<()> {
    // TODO check yes for create new db
    // TODO check for existing col
    let conn = write::open_and_make(&args.db_file_name)?;
    let rdb = read::Db::new(&conn)?;
    let norm_path = crate::fslayer::read::normalized_abspath(&args.path);
    check_with_existing_paths(&rdb, &norm_path)?;

    let mut filter = rdb.glob_filter_by_id(args.filter_id)?;
    let mut wdb = write::Db::new(&conn)?;
    coll_builder::new_collection(
        &mut wdb,
        &args.name,
        &Path::new(&norm_path),
        args.filter_id,
        &mut filter,
        &|e| {
            println!("new entry {:?}", &e);
        },
    )?;
    Ok(())
}

pub fn collection_list(db_file_name: &str) -> anyhow::Result<()> {
    let conn = read::open_existing(db_file_name)?;
    let rdb = read::Db::new(&conn)?;
    let cols = rdb.enum_collections()?;
    for col in cols {
        println!("{},{},{}", col.id, col.coll_name, col.fs_path);
    }
    Ok(())
}
