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
use crate::dblayer::{DbId, DirEntry, Collection};
use crate::fslayer;

pub type CollResult<T> = anyhow::Result<T>;
pub type OnNewDirEntry = dyn Fn(&DirEntry);

pub fn new_collection(name: &str, src_path: &std::path::Path, filter_id: DbId,
                  on_new_direntry: &OnNewDirEntry) -> CollResult<Collection> {
    let root_fs_entry = fslayer::read::dir_entry(src_path)?;
    todo!()
}
