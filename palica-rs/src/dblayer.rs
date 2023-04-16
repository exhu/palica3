pub type DbId = i64;

pub struct Collection {
    pub id: DbId,
    pub coll_name: String,
    pub fs_path: String,
    pub root_id: DbId,
    pub glob_filter_id: Option<DbId>,
}

pub type DbTime = i64;

pub struct DirEntry {
    pub id: DbId,
    pub fs_name: String,
    pub fs_mod_time: DbTime,
    pub last_sync_time: DbTime,
    pub is_dir: bool,
    pub fs_size: i64,
}

pub struct GlobPattern {
    pub id: DbId,
    pub regexp: String,
}

pub struct GlobFilter {
    pub id: DbId,
    pub name: String,
}

pub struct GlobFilterToPattern {
    pub id: DbId,
    pub filter_id: DbId,
    pub glob_pattern_id: DbId,
    pub include: bool,
    pub position: i32,
}

pub struct SettingValue {
    pub id: DbId,
    pub key: String,
    pub value: String,
}

/*
#[derive(Error, Debug)]
pub enum DbError {
    #[error("cannot access db")]
    Unknown,
}
*/

//pub type DbResult<T> = Result<T, DbError>;
pub type DbResult<T> = anyhow::Result<T>;

mod read {
    use super::{Collection, DbResult};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DbError {
        #[error("db file does not exist")]
        NoDbFile,
        #[error("db file schema is not compatible")]
        WrongDbSchema,
    }

    pub fn check_database_validity(conn: &sqlite::Connection) -> bool {
        // TODO check version, tables etc.
        true
    }

    /// Either open an existing database, or fail.
    pub fn open_existing(fname: &str) -> DbResult<sqlite::Connection> {
        use std::path::Path;

        let existing = Path::new(fname).exists();

        if !existing {
            return Err(DbError::NoDbFile.into());
        }

        let conn = sqlite::Connection::open(fname)?;
        if !check_database_validity(&conn) {
            return Err(DbError::WrongDbSchema.into());
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
                    "SELECT id, fs_name,
                fs_mod_time, last_sync_time, is_dir, fs_size FROM dir_entries",
                )?,
            })
        }

        pub fn enum_collections(&self) -> DbResult<Vec<Collection>> {
            todo!()
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

mod write {
    use super::*;
    use crate::dblayer::{Collection, DbId, DbResult, DirEntry};

    pub struct Db<'a> {
        conn: &'a sqlite::Connection,
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
            glob_filter_id: Option<DbId>,
        ) -> DbResult<Collection> {
            let new_id = self.max_id("collections") + 1;
            let mut stmt = self.conn.prepare(
                "INSERT INTO collections(id, coll_name, fs_path, root_id, glob_filter_id)
                VALUES(:id, :coll_name, :fs_path, :root_id, :glob_filter_id)",
            )?;

            let glob_filter: sqlite::Value = nullable_from_option(glob_filter_id);

            stmt.bind_iter::<_, (_, sqlite::Value)>([
                (":id", new_id.into()),
                (":coll_name", coll_name.to_owned().into()),
                (":fs_path", fs_path.to_owned().into()),
                (":root_id", root_id.into()),
                (":glob_filter_id", glob_filter),
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

        // optimization hints before performing many inserts
        pub fn begin(&self) -> DbResult<()> {
            self.conn.execute("BEGIN;")?;
            Ok(())
        }

        pub fn commit(&self) -> DbResult<()> {
            self.conn.execute("END;")?;
            Ok(())
        }

        pub fn rollback(&self) -> DbResult<()> {
            self.conn.execute("ROLLBACK;")?;
            Ok(())
        }
    }

    /// Either open an existing, or initialize a new database.
    pub fn open_and_make(fname: &str) -> DbResult<sqlite::Connection> {
        use super::read::{check_database_validity, DbError};
        use std::fs::read_to_string;
        use std::path::Path;

        let existing = if fname == ":memory:" {
            false
        } else {
            Path::new(fname).exists()
        };
        let conn = sqlite::Connection::open(fname)?;

        if !existing {
            let schema = "sql/schema1.sql";
            eprintln!("reading schema {}", schema);
            let sql = read_to_string(schema)?;
            //begin(&conn)?;
            eprintln!("started tx");
            match conn.execute(sql) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("failed tx {:?}", e);
                    //rollback(&conn)?;
                    return Err(e.into());
                }
            }
            eprintln!("commiting tx");
            //commit(&conn)?;
        } else {
            if !check_database_validity(&conn) {
                return Err(DbError::WrongDbSchema.into());
            }
        }

        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_and_make() {
        let r = write::open_and_make(":memory:");
        match r {
            Ok(_) => (),
            Err(_) => panic!("failed to create db"),
        }
    }

    #[test]
    fn create_dir_and_map() {
        let conn = write::open_and_make(":memory:").unwrap();

        let mut db = write::Db::new(&conn).unwrap();

        let max_id = db.max_id("dir_entries");
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

        assert_eq!(db.max_id("dir_entries"), 1);

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
        let conn = write::open_and_make(":memory:").unwrap();

        let db = write::Db::new(&conn).unwrap();
        let col = db
            .create_collection("myname", "mypath", 1, Option::None)
            .unwrap();
        assert_eq!(col.id, 1);
        let col2 = db
            .create_collection("myname2", "mypath", 1, Some(33))
            .unwrap();
        assert_eq!(col2.id, 2);
    }

    #[test]
    fn open_existing() {
        use std::path::Path;
        use std::fs::remove_file;
        let temp_filename = "tmp-dblayer-open-existing.db";
        if Path::new(&temp_filename).exists() {
            remove_file(temp_filename).unwrap();
        }

        write::open_and_make(&temp_filename).unwrap();
        read::open_existing(&temp_filename).unwrap();
        remove_file(temp_filename).unwrap();
    }
}
