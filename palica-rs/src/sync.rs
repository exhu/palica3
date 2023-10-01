use crate::dblayer::DirEntry;
use crate::fsdbtime::dbtime_from_sys;
use crate::fslayer::FsDirEntry;

#[derive(PartialEq, Debug)]
pub enum DbFsCompareResult {
    Same,
    DbItemBecameDir,
    DbItemBecameFile,
    ModTime,
    Size,
}

/// Quick comparison. Checks date for dir, date + size for a file.
pub fn compare_db_to_fsitem(db_item: &DirEntry, fs_item: &FsDirEntry) -> DbFsCompareResult {
    if db_item.is_dir != fs_item.is_dir {
        return if db_item.is_dir {
            DbFsCompareResult::DbItemBecameFile
        } else {
            DbFsCompareResult::DbItemBecameDir
        };
    }

    let is_same_date = db_item.fs_mod_time == dbtime_from_sys(fs_item.mod_time);

    if db_item.is_dir {
        return if is_same_date {
            DbFsCompareResult::Same
        } else {
            DbFsCompareResult::ModTime
        };
    } else {
        if is_same_date && db_item.fs_size == fs_item.size as i64 {
            return DbFsCompareResult::Same;
        } else {
            return DbFsCompareResult::Size;
        }
    }
}

/*
Sync directory algorithm fs -> db:
1) compare directory dates
2) get the fs list
3) get the db list as a map by filename
4) make "new", "update" empty lists, iterate fs list:
    - find in db list:
        - if not found then add to "new"
        - if different then add to "update"
    - remove the handled item from the db list
5) the db list now contains items to delete

*/

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    #[test]
    fn compare_db_fs() {
        let time_now = SystemTime::now();
        let db_time_now = dbtime_from_sys(time_now);
        let fs_diritem_same = FsDirEntry::new_dir("dir".to_owned(), time_now.clone());
        let fs_diritem_file = FsDirEntry::new_file("file".to_owned(), 32, time_now.clone());
        let fs_diritem_file_size = FsDirEntry::new_file("file".to_owned(), 33, time_now.clone());
        let db_diritem_same = DirEntry {
            id: 1,
            fs_name: "dir".to_owned(),
            fs_mod_time: db_time_now.clone(),
            last_sync_time: 0,
            is_dir: true,
            fs_size: 0,
        };
        let db_diritem_difftime = DirEntry {
            id: 1,
            fs_name: "dir".to_owned(),
            fs_mod_time: 3,
            last_sync_time: 0,
            is_dir: true,
            fs_size: 0,
        };
        let db_diritem_file = DirEntry {
            id: 1,
            fs_name: "file".to_owned(),
            fs_mod_time: db_time_now.clone(),
            last_sync_time: 0,
            is_dir: false,
            fs_size: 32,
        };

        assert_eq!(
            compare_db_to_fsitem(&db_diritem_same, &fs_diritem_same),
            DbFsCompareResult::Same
        );
        assert_eq!(
            compare_db_to_fsitem(&db_diritem_difftime, &fs_diritem_same),
            DbFsCompareResult::ModTime
        );
        assert_eq!(
            compare_db_to_fsitem(&db_diritem_file, &fs_diritem_same),
            DbFsCompareResult::DbItemBecameDir
        );
        assert_eq!(
            compare_db_to_fsitem(&db_diritem_file, &fs_diritem_file),
            DbFsCompareResult::Same
        );
        assert_eq!(
            compare_db_to_fsitem(&db_diritem_same, &fs_diritem_file),
            DbFsCompareResult::DbItemBecameFile
        );
        assert_eq!(
            compare_db_to_fsitem(&db_diritem_file, &fs_diritem_file_size),
            DbFsCompareResult::Size
        );
    }
}
