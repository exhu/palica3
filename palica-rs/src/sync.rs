use crate::dblayer::DirEntry;
use crate::fslayer::FsDirEntry;

pub fn are_the_same(db_item: DirEntry, fs_item: FsDirEntry) -> bool {
    todo!()
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
