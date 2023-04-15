use anyhow::Result;
use sqlite;
use std::fs::read_to_string;

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
    use crate::dblayer::{Collection, DbResult};
    pub fn enum_collections(conn: &sqlite::Connection) -> DbResult<Vec<Collection>> {
        todo!()
    }
}

mod write {
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
                (":is_dir", (entry.is_dir as i64).into()),
                (":fs_size", entry.fs_size.into()),
            ])?;

            while let sqlite::State::Row = self.create_dir.next()? {}

            self.create_dir.reset()?;

            Ok(())
        }

        pub fn max_id(&self, table_name: &str) -> DbId {
            for row in self
                .conn
                .prepare(format!("SELECT MAX(id) from {};", table_name))
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
            name: &str,
            srcpath: &str,
            rootid: DbId,
        ) -> DbResult<Collection> {
            todo!()
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

    pub fn open_and_make(fname: &str) -> DbResult<sqlite::Connection> {
        use std::fs::read_to_string;
        use std::path::Path;

        let existing = Path::new(fname).exists();
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

        let new_id = db.max_id("dir_entries") + 1;
        assert_eq!(new_id, 1);

        db.create_dir_entry(&DirEntry {
            id: new_id,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        })
        .unwrap();

        assert_eq!(db.max_id("dir_entries"), 1);

        db.create_dir_entry(&DirEntry {
            id: new_id+1,
            fs_name: "mydir".to_owned(),
            fs_mod_time: 1,
            last_sync_time: 2,
            is_dir: true,
            fs_size: 0,
        })
        .unwrap();
        // TODO get last row insert id
        //conn.last_ro
    }
}
