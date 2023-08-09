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

pub fn collection_add(args: CollectionAdd) -> ExitCode {
    // TODO proper error handling
    // TODO check yes for create new db
    // TODO check for existing col
    // TODO check for used path for some other col
    let conn = write::open_and_make(&args.db_file_name).unwrap();
    let rdb = read::Db::new(&conn).unwrap();
    let mut filter = rdb.glob_filter_by_id(args.filter_id).unwrap();
    let mut wdb = write::Db::new(&conn).unwrap();
    let col = coll_builder::new_collection(
        &mut wdb,
        &args.name,
        &Path::new(&args.path),
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

pub fn collection_list(db_file_name: &str) -> ExitCode {
    // TODO
    ExitCode::SUCCESS
}
