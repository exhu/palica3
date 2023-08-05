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
use std::time::SystemTime;

#[derive(Debug)]
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
    /// return true to include the path
    type FilterFn = dyn Fn(&Path) -> bool;

    /// not recursive
    /*
    pub fn dir_entries(path: &Path, filter_fn: Option<&FilterFn>) -> FsResult<Vec<FsDirEntry>> {
        let mut res = Vec::new();
        for e in std::fs::read_dir(path)? {
            let entry = e?;
            let path = entry.path();
            let allow = match filter_fn {
                Some(f) => f(&path),
                None => true,
            };
            if allow {
                match dir_entry(&path) {
                    Ok(e) => res.push(e),
                    Err(_) => eprintln!("Skipped '{}'.", path.to_str().unwrap()),
                }
            }
        }
        Ok(res)
    }
    */

    // fs::read_dir -> filter path -> map to dir_entry -> collect?
    pub fn dir_entries<'a>(
        path: &'a Path,
        filter_fn: Option<&'a FilterFn>,
    ) -> FsResult<Box<dyn Iterator<Item = FsDirEntry> + 'a>> {
        let path_str = path.to_string_lossy();
        let res = std::fs::read_dir(path)?
            .filter_map(move |result| match result {
                Ok(entry) => Some(entry),
                _ => {
                    eprintln!("read_dir failed for {}", path_str);
                    None
                }
            })
            .filter(move |item| match filter_fn {
                Some(f) => f(&item.path()),
                None => true,
            })
            .map(|item| {
                let r = dir_entry(&item.path());
                if r.is_err() {
                    eprintln!("dir_entry failed for {}", item.path().to_string_lossy());
                }
                r
            })
            .filter_map(|result| match result {
                Ok(entry) => Some(entry),
                _ => None,
            });
        Ok(Box::new(res))
    }

    #[derive(thiserror::Error, Debug)]
    enum FsError {
        #[error("not a file or dir")]
        NotAfileOrDir,
    }

    pub fn dir_entry(path: &Path) -> FsResult<FsDirEntry> {
        eprintln!("{:?}", path);
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
        Path::new(p)
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dir_entry() {
        let e = read::dir_entry(std::path::Path::new("../README.adoc")).unwrap();
        assert_eq!(e.name, "README.adoc");
    }

    #[cfg(none)]
    #[test]
    fn dir_entries_old() {
        let e = read::dir_entries(std::path::Path::new("../sample-data"), None).unwrap();
        assert!(e.len() > 3);
        assert_eq!(e.iter().any(|i| i.name == "img1.jxl"), true);
        assert_eq!(e.iter().any(|i| i.name == "img1.jpg"), true);
        assert_eq!(e.iter().any(|i| i.name == "img1.webp"), true);
    }

    #[test]
    fn dir_entries() {
        let items = read::dir_entries(std::path::Path::new("../sample-data"), None).unwrap();
        let mut count = 0;
        let known = ["img1.jxl", "img1.jpg", "img1.webp"];
        let mut found = Vec::<String>::new();
        for item in items {
            found.push(item.name);
            count += 1;
        }
        assert!(count > 3);
        println!("{:?}", &found);
        for k in known {
            assert!(found.iter().any(|i| {
                println!("{:?} == {:?}, {:?}", i, k, i == k);
                i == k
            }));
        }
    }
}
