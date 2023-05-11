use crate::dblayer::{DbId, DirEntry, Collection};

pub type CollResult<T> = anyhow::Result<T>;
pub type OnNewDirEntry = dyn Fn(&DirEntry);

pub fn new_collection(name: &str, src_path: &str, filter_id: DbId,
                  on_new_direntry: &OnNewDirEntry) -> CollResult<Collection> {
    todo!()
}
