use std::time::SystemTime;

pub struct FsDirEntry {
    pub name: String,
    pub size: u64,
    pub mod_time: SystemTime,
    pub is_dir: bool,
}

impl FsDirEntry {
    pub fn new_file(name: String, size: u64, mod_time: SystemTime) -> FsDirEntry {
        FsDirEntry {
            name,
            size,
            mod_time,
            is_dir: false,
        }
    }

    pub fn new_dir(name: String, mod_time: SystemTime) -> FsDirEntry {
        FsDirEntry {
            name,
            size: 0,
            mod_time,
            is_dir: true,
        }
    }
}

pub mod read {
    use super::FsDirEntry;
    use std::path::Path;

    type FsResult<T> = anyhow::Result<T>;

    /// not recursive
    pub fn dir_entries(p: &str) -> Vec<FsDirEntry> {
        todo!()
    }

    #[derive(thiserror::Error, Debug)]
    enum FsError {
        #[error("not a file or dir")]
        NotAfileOrDir,
    }

    pub fn dir_entry(p: &str) -> FsResult<FsDirEntry> {
        let path = Path::new(p);
        let fname = path.file_name().unwrap().to_str().unwrap().into();
        let modtime = path.metadata().unwrap().modified()?;
        if path.is_dir() {
            return Ok(FsDirEntry::new_dir(fname, modtime));
        } else if path.is_file() {
            return Ok(FsDirEntry::new_file(
                fname,
                path.metadata().unwrap().len(),
                modtime,
            ));
        }

        Err(FsError::NotAfileOrDir.into())
    }

    pub fn normalized_abspath(p: &str) -> String {
        todo!()
    }
}
