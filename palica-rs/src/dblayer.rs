use thiserror::Error;

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
/// 'default' filter from schema1.sql
pub const DEFAULT_FILTER_ID: DbId = 1;

pub type DbId = i64;

pub struct Transaction<'a> {
    conn: &'a sqlite::Connection,
    finished: bool,
}

impl Transaction<'_> {
    pub fn new<'a>(conn: &'a sqlite::Connection) -> Transaction<'a> {
        eprintln!("starting tx...");
        conn.execute("BEGIN").expect("Failed to begin tx.");
        Transaction {
            conn,
            finished: false,
        }
    }

    pub fn commit(&mut self) {
        eprintln!("commiting tx...");
        self.conn.execute("END").expect("Failed to commit tx.");
        self.finished = true;
    }

    pub fn rollback(&mut self) {
        eprintln!("rollback tx...");
        self.conn
            .execute("ROLLBACK")
            .expect("Failed to rollback tx.");
        self.finished = true;
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if !self.finished {
            self.rollback();
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Collection {
    pub id: DbId,
    pub coll_name: String,
    pub fs_path: String,
    pub root_id: DbId,
    pub glob_filter_id: DbId,
}

impl Collection {
    fn from_row(row: &sqlite::Row) -> Collection {
        Collection {
            id: row.read::<i64, usize>(0),
            coll_name: row.read::<&str, usize>(1).to_string(),
            fs_path: row.read::<&str, usize>(2).to_string(),
            root_id: row.read::<i64, usize>(3),
            glob_filter_id: row.read::<i64, usize>(4),
        }
    }

    pub fn table_name() -> &'static str {
        "collections"
    }
}

pub type DbTime = i64;

#[derive(Debug)]
pub struct DirEntry {
    pub id: DbId,
    pub fs_name: String,
    pub fs_mod_time: DbTime,
    pub last_sync_time: DbTime,
    pub is_dir: bool,
    pub fs_size: i64,
}

impl DirEntry {
    pub fn from_row(row: &sqlite::Row) -> DirEntry {
        DirEntry {
            id: row.read::<i64, usize>(0),
            fs_name: row.read::<&str, usize>(1).to_owned(),
            fs_mod_time: row.read::<i64, usize>(2).to_owned(),
            last_sync_time: row.read::<i64, usize>(3).to_owned(),
            is_dir: row.read::<i64, usize>(4) != 0,
            fs_size: row.read::<i64, usize>(5),
        }
    }

    pub fn table_name() -> &'static str {
        "dir_entries"
    }
}

#[derive(Debug)]
pub struct GlobPattern {
    pub id: DbId,
    pub regexp: String,
}

impl GlobPattern {
    pub fn from_row(row: &sqlite::Row) -> GlobPattern {
        GlobPattern {
            id: row.read::<i64, usize>(0),
            regexp: row.read::<&str, usize>(1).to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct GlobFilter {
    pub id: DbId,
    pub name: String,
}

impl GlobFilter {
    pub fn from_row(row: &sqlite::Row) -> GlobFilter {
        GlobFilter {
            id: row.read::<i64, usize>(0),
            name: row.read::<&str, usize>(1).to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct GlobFilterToPattern {
    pub id: DbId,
    pub glob_filter_id: DbId,
    pub glob_pattern_id: DbId,
    pub include: bool,
    pub position: i32,
}

impl GlobFilterToPattern {
    pub fn from_row(row: &sqlite::Row) -> GlobFilterToPattern {
        GlobFilterToPattern {
            id: row.read::<i64, usize>(0),
            glob_filter_id: row.read::<i64, usize>(1),
            glob_pattern_id: row.read::<i64, usize>(2),
            include: row.read::<i64, usize>(3) != 0,
            position: row.read::<i64, usize>(4) as i32,
        }
    }
}

#[derive(Debug)]
pub struct SettingValue {
    pub id: DbId,
    pub key: String,
    pub value: String,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlite error: {error}")]
    SqliteError { error: sqlite::Error },
    #[error("db file does not exist: {name}")]
    NoDbFile { name: String },
    #[error("db file schema is not compatible: {name}")]
    WrongDbSchema { name: String },
    #[error("io error: {error}")]
    IoError { error: std::io::Error },
    #[error("db file aready exists: {name}")]
    AlreadyExists { name: String },
    #[error("filter to glob integrity error, filter: {filter_id}, glob: {glob_id}")]
    FilterToGlobIntegrityError { filter_id: DbId, glob_id: DbId },
}

impl From<std::io::Error> for DbError {
    fn from(error: std::io::Error) -> Self {
        DbError::IoError { error: error }
    }
}

impl From<sqlite::Error> for DbError {
    fn from(error: sqlite::Error) -> Self {
        DbError::SqliteError { error }
    }
}

pub type DbResult<T> = Result<T, DbError>;

fn complete_statement(stmt: &mut sqlite::Statement) -> sqlite::Result<()> {
    loop {
        match stmt.next() {
            Ok(state) => {
                if state == sqlite::State::Done {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn exec_statement_with_arg<T>(stmt: &mut sqlite::Statement, arg: T) -> sqlite::Result<()>
where
    T: sqlite::BindableWithIndex,
{
    stmt.bind((1, arg))?;
    complete_statement(stmt)
}

fn exec_sql_stmt_with_arg<T>(
    conn: &sqlite::Connection,
    sql_text: &'static str,
    arg: T,
) -> sqlite::Result<()>
where
    T: sqlite::BindableWithIndex,
{
    let mut stmt = conn.prepare(sql_text)?;
    exec_statement_with_arg(&mut stmt, arg)
}

pub mod read {
    use std::collections::HashMap;

    use super::*;
    use crate::glob_filter::{Filter, FilterItem, Pattern};

    pub fn check_database_validity(_conn: &sqlite::Connection) -> bool {
        // TODO check version, tables etc.
        true
    }

    /// Either open an existing database, or fail.
    pub fn open_existing(fname: &str) -> DbResult<sqlite::Connection> {
        use std::path::Path;

        let existing = if fname == ":memory:" {
            false
        } else {
            Path::new(fname).exists()
        };

        if !existing {
            return Err(DbError::NoDbFile {
                name: fname.to_owned(),
            });
        }

        let conn = sqlite::Connection::open(fname)?;
        if !check_database_validity(&conn) {
            return Err(DbError::WrongDbSchema {
                name: fname.to_owned(),
            });
        }

        Ok(conn)
    }

    pub struct Db<'a> {
        conn: &'a sqlite::Connection,
        list_dir: sqlite::Statement<'a>,
    }

    impl Db<'_> {
        pub fn new<'a>(conn: &'a sqlite::Connection) -> DbResult<Db<'a>> {
            Ok(Db {
                conn,
                list_dir: conn.prepare(
                    "SELECT e.id, e.fs_name, e.fs_mod_time,
                    e.last_sync_time, e.is_dir, e.fs_size FROM dir_entries e
                    JOIN dir_to_sub d ON d.entry_id = e.id
                    WHERE d.directory_id = ?1 ORDER BY e.fs_name, e.is_dir DESC",
                )?,
            })
        }

        pub fn enum_collections(&self) -> DbResult<Vec<Collection>> {
            let mut res: Vec<Collection> = Vec::new();
            let prep = self.conn.prepare(
                "SELECT id, coll_name, fs_path,
                root_id, glob_filter_id FROM collections ORDER BY coll_name",
            )?;
            for row in prep.into_iter() {
                let c = Collection::from_row(&row?);
                res.push(c);
            }
            Ok(res)
        }

        /// returns items sorted by name, directories first
        pub fn enum_dir_entries(&mut self, parent_id: DbId) -> DbResult<Vec<DirEntry>> {
            self.list_dir.bind((1, parent_id))?;
            let res: DbResult<Vec<DirEntry>> = self
                .list_dir
                .iter()
                .map(|r| Ok(DirEntry::from_row(&r?)))
                .collect();
            self.list_dir.reset()?;
            res
        }

        pub fn enum_glob_filters(&self) -> DbResult<Vec<GlobFilter>> {
            let res = self
                .conn
                .prepare("SELECT id, name FROM glob_filters ORDER BY name")?
                .iter()
                .map(|r| Ok(GlobFilter::from_row(&r?)))
                .collect();
            res
        }

        pub fn enum_glob_patterns(&self) -> DbResult<Vec<GlobPattern>> {
            let res = self
                .conn
                .prepare("SELECT id, regexp FROM glob_patterns ORDER BY regexp")?
                .iter()
                .map(|r| Ok(GlobPattern::from_row(&r?)))
                .collect();
            res
        }

        /// returns sorted by position
        pub fn filter_patterns(&self, filter_id: DbId) -> DbResult<Vec<GlobFilterToPattern>> {
            let mut stmt = self.conn.prepare(
                "SELECT id, glob_filter_id, glob_pattern_id, include,
                    position FROM glob_filter_to_pattern WHERE glob_filter_id = ?1 ORDER BY position",
            )?;
            stmt.bind((1, filter_id))?;
            let res = stmt
                .iter()
                .map(|r| Ok(GlobFilterToPattern::from_row(&r?)))
                .collect();
            res
        }

        pub fn glob_filter_by_id(&self, filter_id: DbId) -> DbResult<Filter> {
            let dbpatterns = self.enum_glob_patterns()?;
            let dbfilter_patterns = self.filter_patterns(filter_id)?;

            // keep only glob patterns that are used by the filter
            let dbpatterns = dbpatterns.into_iter().filter(|item| {
                dbfilter_patterns
                    .iter()
                    .find(|pattern| pattern.glob_pattern_id == item.id)
                    .is_some()
            });

            // build new patterns list
            let mut patterns = Vec::<Pattern>::new();
            let mut pattern_id_to_index = HashMap::<DbId, usize>::new();

            for filtered_pat in dbpatterns {
                let found_pattern = pattern_id_to_index.get(&filtered_pat.id);
                if found_pattern.is_none() {
                    let new_index = patterns.len();
                    patterns.push(Pattern::new(&filtered_pat.regexp));
                    pattern_id_to_index.insert(filtered_pat.id, new_index);
                }
            }

            let mut filter_items = Vec::<FilterItem>::new();
            // build filter items
            for filter_pat in dbfilter_patterns {
                filter_items.push(FilterItem {
                    pattern_index: *pattern_id_to_index.get(&filter_pat.glob_pattern_id).ok_or(
                        DbError::FilterToGlobIntegrityError {
                            filter_id: filter_id,
                            glob_id: filter_pat.glob_pattern_id,
                        },
                    )?,
                    include: filter_pat.include,
                });
            }
            Ok(Filter {
                patterns,
                items: filter_items,
            })
        }

        pub fn collections_by_fs_path(&self, fs_path: &str) -> DbResult<Vec<Collection>> {
            let mut res: Vec<Collection> = Vec::new();
            let mut prep = self.conn.prepare(
                "SELECT id, coll_name, fs_path,
                root_id, glob_filter_id FROM collections WHERE fs_path=?1
                ORDER BY coll_name",
            )?;
            prep.bind((1, fs_path))?;
            for row in prep.into_iter() {
                let c = Collection::from_row(&row?);
                res.push(c);
            }
            Ok(res)
        }

        pub fn collection_by_name(&self, name: &str) -> DbResult<Option<Collection>> {
            let mut prep = self.conn.prepare(
                "SELECT id, coll_name, fs_path,
                root_id, glob_filter_id FROM collections WHERE coll_name=?1",
            )?;
            prep.bind((1, name))?;
            for row in prep.into_iter() {
                let c = Collection::from_row(&row?);
                return Ok(Some(c));
            }
            Ok(None)
        }

        pub fn dir_entry_by_id(&self, id: DbId) -> DbResult<Option<DirEntry>> {
            let mut prep = self.conn.prepare(
                "SELECT id, fs_name, fs_mod_time,
                    last_sync_time, is_dir, fs_size FROM dir_entries
                    WHERE id = ?1",
            )?;
            prep.bind((1, id))?;
            for row in prep.into_iter() {
                let row = row?;
                return Ok(Some(DirEntry::from_row(&row)));
            }
            Ok(None)
        }
    }
}

pub fn nullable_from_option<T>(o: Option<T>) -> sqlite::Value
where
    sqlite::Value: From<T>,
{
    let r: sqlite::Value = match o {
        Some(v) => v.into(),
        None => sqlite::Value::Null,
    };
    r
}

pub fn value_from_bool(b: bool) -> sqlite::Value {
    (b as i64).into()
}

/// For optimization hints before performing many inserts use Transaction.
pub mod write {
    use super::*;
    use crate::dblayer::{Collection, DbId, DbResult, DirEntry};

    pub struct IdGen {
        next_id: DbId,
    }

    impl IdGen {
        /// pass last id, e.g. result of select max(id) from my_sql_table;
        pub fn new_with_last_id(last_id: DbId) -> IdGen {
            IdGen {
                next_id: last_id + 1,
            }
        }

        pub fn gen_id(&mut self) -> DbId {
            let result = self.next_id;
            self.next_id += 1;
            result
        }
    }

    #[derive(Error, Debug)]
    pub enum DeleteError {
        #[error("Db error: {error}")]
        DbError { error: DbError },
        #[error("Sqlite error: {error}")]
        SqliteError { error: sqlite::Error },
        #[error("Not a file: {name}")]
        NotAfile { name: String },
        #[error("Not a directory: {name}")]
        NotAdir { name: String },
        #[error("No root entry")]
        NoRootEntry,
    }

    impl From<sqlite::Error> for DeleteError {
        fn from(error: sqlite::Error) -> Self {
            DeleteError::SqliteError { error }
        }
    }

    impl From<DbError> for DeleteError {
        fn from(error: DbError) -> Self {
            DeleteError::DbError { error }
        }
    }

    pub type DeleteResult<T> = Result<T, DeleteError>;

    pub struct Db<'a> {
        pub conn: &'a sqlite::Connection,
        create_dir: sqlite::Statement<'a>,
        map_dir: sqlite::Statement<'a>,
    }

    impl Db<'_> {
        pub fn new<'a>(conn: &'a sqlite::Connection) -> DbResult<Db<'a>> {
            Ok(Db {
                conn,
                create_dir: conn.prepare(
                    "INSERT INTO dir_entries(id, fs_name,
                fs_mod_time, last_sync_time, is_dir, fs_size)
                VALUES(:id, :fs_name, :fs_mod_time, :last_sync_time,
                       :is_dir, :fs_size)",
                )?,
                map_dir: conn.prepare(
                    "INSERT INTO dir_to_sub(directory_id,
                entry_id) VALUES(:directory_id, :entry_id)",
                )?,
            })
        }

        pub fn create_dir_entry(&mut self, entry: &DirEntry) -> DbResult<()> {
            self.create_dir.bind_iter::<_, (_, sqlite::Value)>([
                (":id", entry.id.into()),
                (":fs_name", entry.fs_name.clone().into()),
                (":fs_mod_time", entry.fs_mod_time.into()),
                (":last_sync_time", entry.last_sync_time.into()),
                (":is_dir", value_from_bool(entry.is_dir)),
                (":fs_size", entry.fs_size.into()),
            ])?;

            while let sqlite::State::Row = self.create_dir.next()? {}

            self.create_dir.reset()?;

            Ok(())
        }

        pub fn max_id(&self, table_name: &str) -> DbId {
            for row in self
                .conn
                .prepare(format!("SELECT MAX(id) FROM {};", table_name))
                .unwrap()
                .into_iter()
                .map(|row| row.unwrap())
            {
                return row.try_read::<i64, _>(0).unwrap_or(0);
            }
            -1
        }

        pub fn create_collection(
            &self,
            coll_name: &str,
            fs_path: &str,
            root_id: DbId,
            glob_filter_id: DbId,
        ) -> DbResult<Collection> {
            let new_id = self.max_id(Collection::table_name()) + 1;
            let mut stmt = self.conn.prepare(
                "INSERT INTO collections(id, coll_name, fs_path, root_id, glob_filter_id)
                VALUES(:id, :coll_name, :fs_path, :root_id, :glob_filter_id)",
            )?;

            stmt.bind_iter::<_, (_, sqlite::Value)>([
                (":id", new_id.into()),
                (":coll_name", coll_name.to_owned().into()),
                (":fs_path", fs_path.to_owned().into()),
                (":root_id", root_id.into()),
                (":glob_filter_id", glob_filter_id.clone().into()),
            ])?;

            while let sqlite::State::Row = stmt.next()? {}
            Ok(Collection {
                id: new_id,
                coll_name: coll_name.to_string(),
                fs_path: fs_path.to_string(),
                root_id,
                glob_filter_id,
            })
        }

        pub fn map_dir_entry_to_parent_dir(
            &mut self,
            entry_id: DbId,
            parent_id: DbId,
        ) -> DbResult<()> {
            self.map_dir.bind_iter::<_, (_, sqlite::Value)>([
                (":entry_id", entry_id.into()),
                (":directory_id", parent_id.into()),
            ])?;

            while let sqlite::State::Row = self.map_dir.next()? {}

            self.map_dir.reset()?;

            Ok(())
        }

        fn delete_dir_entry_dir(&self, entry: DirEntry) -> DeleteResult<()> {
            if !entry.is_dir {
                return Err(DeleteError::NotAfile {
                    name: entry.fs_name,
                });
            }
            let mut subdirs = Vec::<DirEntry>::new();
            subdirs.push(entry);
            let mut read_db = super::read::Db::new(self.conn)?;
            while let Some(subdir) = subdirs.pop() {
                let contents = read_db.enum_dir_entries(subdir.id)?;
                for item in contents {
                    if item.is_dir {
                        subdirs.push(item);
                    } else {
                        self.delete_dir_entry_file(item)?;
                    }
                }

                exec_sql_stmt_with_arg(
                    self.conn,
                    "DELETE FROM dir_entries WHERE id = ?1",
                    subdir.id,
                )?;

                exec_sql_stmt_with_arg(
                    self.conn,
                    "DELETE FROM dir_to_sub WHERE directory_id = ?1 OR entry_id = ?1",
                    subdir.id,
                )?;
            }
            Ok(())
        }

        fn delete_dir_entry_file(&self, entry: DirEntry) -> DeleteResult<()> {
            if entry.is_dir {
                return Err(DeleteError::NotAfile {
                    name: entry.fs_name,
                });
            }
            // TODO thumbnails
            exec_sql_stmt_with_arg(self.conn, "DELETE FROM dir_entries WHERE id = ?1", entry.id)?;
            exec_sql_stmt_with_arg(
                self.conn,
                "DELETE FROM dir_to_sub WHERE entry_id = ?1",
                entry.id,
            )?;
            exec_sql_stmt_with_arg(
                self.conn,
                "DELETE FROM tag_to_dir_entry WHERE dir_entry_id = ?1",
                entry.id,
            )?;
            exec_sql_stmt_with_arg(
                self.conn,
                "DELETE FROM last_edit WHERE dir_entry_id = ?1",
                entry.id,
            )?;
            exec_sql_stmt_with_arg(
                self.conn,
                "DELETE FROM mime_to_dir_entry WHERE dir_entry_id = ?1",
                entry.id,
            )?;
            Ok(())
        }

        /// Deletes a directory (and it's subdirectories/files) or a file, and
        /// all associated data in other tables.
        pub fn delete_dir_entry(&self, entry: DirEntry) -> DeleteResult<()> {
            let mut tx = Transaction::new(&self.conn);
            if entry.is_dir {
                self.delete_dir_entry_dir(entry)?;
            } else {
                self.delete_dir_entry_file(entry)?;
            }

            tx.commit();
            Ok(())
        }

        pub fn delete_collection(&self, col: Collection) -> DeleteResult<()> {
            let read_db = super::read::Db::new(self.conn)?;
            let root = read_db.dir_entry_by_id(col.root_id)?;
            if let Some(root_entry) = root {
                self.delete_dir_entry(root_entry)?;
            } else {
                return Err(DeleteError::NoRootEntry);
            }
            exec_sql_stmt_with_arg(self.conn, "DELETE FROM collections WHERE id = ?1", col.id)?;
            Ok(())
        }
    }

    pub fn create_new(fname: &str) -> DbResult<sqlite::Connection> {
        use std::fs::read_to_string;
        use std::path::Path;

        let existing = if fname == ":memory:" {
            false
        } else {
            Path::new(fname).exists()
        };

        if existing {
            return Err(DbError::AlreadyExists {
                name: fname.to_owned(),
            });
        }
        let conn = sqlite::Connection::open(fname)?;

        let schema = "sql/schema1.sql";
        eprintln!("reading schema {}", schema);
        let sql = read_to_string(schema)?;
        conn.execute(sql)?;
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_and_make() {
        let r = write::create_new(":memory:");
        match r {
            Ok(_) => (),
            Err(_) => panic!("failed to create db"),
        }
    }

    #[test]
    fn create_dir_and_map() {
        let conn = write::create_new(":memory:").unwrap();

        let mut db = write::Db::new(&conn).unwrap();

        let max_id = db.max_id(DirEntry::table_name());
        assert_eq!(max_id, 0);

        let mydir_id = max_id + 1;

        db.create_dir_entry(&DirEntry {
            id: mydir_id,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        })
        .unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 1);

        let myfile_id = mydir_id + 1;

        db.create_dir_entry(&DirEntry {
            id: myfile_id,
            fs_name: "myfile".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 7,
        })
        .unwrap();

        db.map_dir_entry_to_parent_dir(myfile_id, mydir_id).unwrap();
    }

    #[test]
    fn create_collection() {
        let conn = write::create_new(":memory:").unwrap();

        let db = write::Db::new(&conn).unwrap();
        let col = db.create_collection("myname", "mypath", 1, 1).unwrap();
        assert_eq!(col.id, 1);
        let col2 = db.create_collection("myname2", "mypath", 1, 33).unwrap();
        assert_eq!(col2.id, 2);
    }

    #[test]
    fn open_existing() {
        use std::fs::remove_file;
        use std::path::Path;
        let temp_filename = "tmp-dblayer-open-existing.db";
        if Path::new(&temp_filename).exists() {
            remove_file(temp_filename).unwrap();
        }

        write::create_new(&temp_filename).unwrap();
        read::open_existing(&temp_filename).unwrap();
        remove_file(temp_filename).unwrap();
    }

    #[test]
    fn enum_collections() {
        let conn = write::create_new(":memory:").unwrap();

        let db = write::Db::new(&conn).unwrap();
        let _col = db.create_collection("myname", "mypath", 1, 1).unwrap();
        let _col2 = db.create_collection("myname2", "mypath", 1, 33).unwrap();

        let dbread = read::Db::new(&conn).unwrap();
        let cols = dbread.enum_collections().unwrap();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].coll_name, "myname");
        assert_eq!(cols[1].coll_name, "myname2");
        eprintln!("{:?}", cols);
    }

    #[test]
    fn enum_dir() {
        let conn = write::create_new(":memory:").unwrap();

        let mut db = write::Db::new(&conn).unwrap();

        let max_id = db.max_id(DirEntry::table_name());
        assert_eq!(max_id, 0);

        let mydir_id = max_id + 1;

        db.create_dir_entry(&DirEntry {
            id: mydir_id,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        })
        .unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 1);

        let myfile_id = mydir_id + 1;

        db.create_dir_entry(&DirEntry {
            id: myfile_id,
            fs_name: "fileA".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 7,
        })
        .unwrap();

        let myfile_id2 = myfile_id + 1;
        db.create_dir_entry(&DirEntry {
            id: myfile_id2,
            fs_name: "fileB".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 7,
        })
        .unwrap();

        let mydir_id2 = myfile_id2 + 1;
        db.create_dir_entry(&DirEntry {
            id: mydir_id2,
            fs_name: "Zsubdir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        })
        .unwrap();

        db.map_dir_entry_to_parent_dir(myfile_id, mydir_id).unwrap();
        db.map_dir_entry_to_parent_dir(myfile_id2, mydir_id)
            .unwrap();
        db.map_dir_entry_to_parent_dir(mydir_id2, mydir_id).unwrap();

        let mut dbread = read::Db::new(&conn).unwrap();
        let files = dbread.enum_dir_entries(mydir_id).unwrap();
        assert_eq!(files.len(), 3);
        assert_eq!(files[0].fs_name, "Zsubdir");
        assert_eq!(files[1].fs_name, "fileA");
        assert_eq!(files[2].fs_name, "fileB");

        eprintln!("{:?}", files);
    }

    #[test]
    fn id_gen_test() {
        let mut gen = write::IdGen::new_with_last_id(0);
        assert_eq!(gen.gen_id(), 1);
        assert_eq!(gen.gen_id(), 2);
    }

    #[test]
    fn glob_filter_by_id_test() {
        let conn = write::create_new(":memory:").unwrap();
        let db = read::Db::new(&conn).unwrap();

        let mut default_filter = db.glob_filter_by_id(1).unwrap();
        eprintln!("default filter = {:?}", default_filter);
        assert!(default_filter.include("/asdasd/abra.jpeg"));
        assert_eq!(default_filter.include("asdasd/.git/abra.jpeg"), false);
    }

    #[test]
    fn col_by_name() {
        let conn = write::create_new(":memory:").unwrap();
        let db = write::Db::new(&conn).unwrap();
        let _col = db.create_collection("cola", "mypath", 1, 1).unwrap();
        let dbread = read::Db::new(&conn).unwrap();
        let c = dbread.collection_by_name("cola").unwrap();
        assert_eq!(c.is_some(), true);
        assert_eq!(c.unwrap().coll_name, "cola");
    }

    #[test]
    fn delete_dir() {
        let conn = write::create_new(":memory:").unwrap();
        let mut db = write::Db::new(&conn).unwrap();
        let max_id = db.max_id(DirEntry::table_name());
        assert_eq!(max_id, 0);
        let mydir_id = max_id + 1;
        let dir = DirEntry {
            id: mydir_id,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        };
        db.create_dir_entry(&dir).unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 1);

        let myfile_id = mydir_id + 1;

        db.create_dir_entry(&DirEntry {
            id: myfile_id,
            fs_name: "myfile".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 7,
        })
        .unwrap();

        db.map_dir_entry_to_parent_dir(myfile_id, mydir_id).unwrap();

        let subdir_id = myfile_id + 1;
        db.create_dir_entry(&DirEntry {
            id: subdir_id,
            fs_name: "mysubdir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        })
        .unwrap();
        db.map_dir_entry_to_parent_dir(subdir_id, mydir_id).unwrap();
        let subfile_id = subdir_id + 1;
        db.create_dir_entry(&DirEntry {
            id: subfile_id,
            fs_name: "myfile-inside".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 0,
        })
        .unwrap();
        db.map_dir_entry_to_parent_dir(subfile_id, subdir_id)
            .unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 4);
        db.delete_dir_entry(dir).unwrap();
        assert_eq!(db.max_id(DirEntry::table_name()), 0);
    }

    #[test]
    fn delete_file() {
        let conn = write::create_new(":memory:").unwrap();
        let mut db = write::Db::new(&conn).unwrap();
        let max_id = db.max_id(DirEntry::table_name());
        assert_eq!(max_id, 0);
        let mydir_id = max_id + 1;
        let dir = DirEntry {
            id: mydir_id,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        };
        db.create_dir_entry(&dir).unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 1);

        let myfile_id = mydir_id + 1;

        let file_entry = DirEntry {
            id: myfile_id,
            fs_name: "myfile".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 7,
        };
        db.create_dir_entry(&file_entry).unwrap();

        db.map_dir_entry_to_parent_dir(myfile_id, mydir_id).unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 2);
        db.delete_dir_entry(file_entry).unwrap();
        assert_eq!(db.max_id(DirEntry::table_name()), 1);
    }

    #[test]
    fn delete_collection() {
        let conn = write::create_new(":memory:").unwrap();
        let mut db = write::Db::new(&conn).unwrap();
        let max_id = db.max_id(DirEntry::table_name());
        assert_eq!(max_id, 0);
        let mydir_id = max_id + 1;
        let dir = DirEntry {
            id: mydir_id,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        };
        db.create_dir_entry(&dir).unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 1);

        let myfile_id = mydir_id + 1;

        let file_entry = DirEntry {
            id: myfile_id,
            fs_name: "myfile".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: false,
            fs_size: 7,
        };
        db.create_dir_entry(&file_entry).unwrap();

        db.map_dir_entry_to_parent_dir(myfile_id, mydir_id).unwrap();

        assert_eq!(db.max_id(DirEntry::table_name()), 2);

        let col = db
            .create_collection("sample_col", "path/aa", mydir_id, 1)
            .unwrap();

        let read_db = super::read::Db::new(db.conn).unwrap();

        assert_eq!(
            read_db.collection_by_name("sample_col").unwrap().unwrap(),
            col
        );
        db.delete_collection(col).unwrap();
        assert_eq!(
            read_db.collection_by_name("sample_col").unwrap().is_none(),
            true
        );
        assert_eq!(db.max_id(DirEntry::table_name()), 0);
    }
}
