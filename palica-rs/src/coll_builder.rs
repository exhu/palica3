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
use crate::dblayer::{Collection, DbId, DirEntry};
use crate::fsdbtime::dbtime_from_sys;
use crate::fslayer::{read, FsDirEntry};
use crate::{dblayer, fslayer};

use std::collections::VecDeque;
use std::str::FromStr;
use std::time::SystemTime;

pub type CollResult<T> = anyhow::Result<T>;
pub type OnNewDirEntry = dyn Fn(&DirEntry);

fn new_entry_from_fs(fs_entry: &FsDirEntry, new_id: DbId, sync_time: SystemTime) -> DirEntry {
    dblayer::DirEntry {
        id: new_id,
        fs_name: fs_entry.name.clone(),
        fs_mod_time: dbtime_from_sys(fs_entry.mod_time),
        last_sync_time: dbtime_from_sys(sync_time),
        is_dir: fs_entry.is_dir,
        fs_size: fs_entry.size as i64,
    }
}

pub fn new_collection(
    write_db: &mut dblayer::write::Db,
    name: &str,
    src_path: &std::path::Path,
    filter_id: DbId,
    on_new_direntry: &OnNewDirEntry,
) -> CollResult<Collection> {
    let src_path = src_path.canonicalize()?;
    let root_fs_entry = read::dir_entry(&src_path)?;
    let mut tx = dblayer::Transaction::new(&write_db.conn);
    let sync_time = std::time::SystemTime::now();

    let mut id_gen =
        dblayer::write::IdGen::new_with_last_id(write_db.max_id(DirEntry::table_name()));
    let new_id = id_gen.gen_id();
    let root_entry: DirEntry = new_entry_from_fs(&root_fs_entry, new_id, sync_time);
    write_db.create_dir_entry(&root_entry)?;
    let col = write_db.create_collection(
        name,
        src_path.file_name().unwrap().to_str().unwrap().into(),
        root_entry.id,
        filter_id,
    )?;
    on_new_direntry(&root_entry);

    // subdirs -> parent_id, Path
    let mut subdirs = VecDeque::<(DbId, std::path::PathBuf)>::new();
    subdirs.push_back((root_entry.id, src_path.to_owned()));

    // TODO get filter

    while let Some((root_id, root_path)) = subdirs.pop_front() {
        if let Ok(entries) = fslayer::read::dir_entries(&root_path, None) {
            for item in entries {
                let db_item = new_entry_from_fs(&item, id_gen.gen_id(), sync_time);
                eprintln!("db_item = {:?}", &db_item);
                write_db.create_dir_entry(&db_item)?;
                write_db.map_dir_entry_to_parent_dir(db_item.id, root_id)?;
                on_new_direntry(&db_item);
                if item.is_dir {
                    let p = root_path.join(item.name);
                    subdirs.push_back((db_item.id, p));
                }
            }
        }
    }

    tx.commit();
    Ok(col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dblayer::write;
    #[test]
    fn new_col() {
        let conn = write::open_and_make(":memory:").unwrap();
        let mut db = write::Db::new(&conn).unwrap();
        let col = new_collection(&mut db, "testcol", &std::path::Path::new("./"), 1, &|e| {
            eprintln!("new entry {:?}", &e);
        });
        eprintln!("{:?}", col);
        assert!(col.is_ok());
    }
}
