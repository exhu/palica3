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
use crate::dblayer;
use crate::dblayer::{Collection, DbId, DirEntry};
use crate::fsdbtime::dbtime_from_sys;
use crate::fslayer;

pub type CollResult<T> = anyhow::Result<T>;
pub type OnNewDirEntry = dyn Fn(&DirEntry);

pub fn new_collection(
    write_db: &mut dblayer::write::Db,
    name: &str,
    src_path: &std::path::Path,
    filter_id: DbId,
    on_new_direntry: &OnNewDirEntry,
) -> CollResult<Collection> {
    let root_fs_entry = fslayer::read::dir_entry(src_path)?;
    let mut tx = dblayer::Transaction::new(&write_db.conn);
    // TODO now
    let sync_time = 0;

    // TODO refactor to func
    let root_entry = dblayer::DirEntry {
        id: write_db.max_id(DirEntry::table_name()),
        fs_name: root_fs_entry.name,
        fs_mod_time: dbtime_from_sys(root_fs_entry.mod_time),
        last_sync_time: sync_time,
        is_dir: root_fs_entry.is_dir,
        fs_size: root_fs_entry.size as i64,
    };
    write_db.create_dir_entry(&root_entry)?;
    let col = write_db.create_collection(
        name,
        src_path.file_name().unwrap().to_str().unwrap().into(),
        root_entry.id,
        filter_id,
    );

    // TODO scan dir

    tx.commit();

    todo!()
}
