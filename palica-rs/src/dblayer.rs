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

    pub struct DirStatements<'a> {
        pub create_dir: sqlite::Statement<'a>,
        pub map_dir: sqlite::Statement<'a>,
    }

    impl DirStatements<'_> {
        pub fn new<'a>(conn: &'a sqlite::Connection) -> DbResult<DirStatements<'a>> {
            Ok(DirStatements {
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
    }

    pub fn max_id(conn: &sqlite::Connection, table_name: &str) -> DbId {
        for row in conn
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
        conn: &sqlite::Connection,
        name: &str,
        srcpath: &str,
        rootid: DbId,
    ) -> DbResult<Collection> {
        todo!()
    }

    /// entry.id is requred, let new_id = max_id("dir_entries")+1;
    pub fn create_dir_entry(stmt: &mut sqlite::Statement, entry: &DirEntry) -> DbResult<()> {
        stmt.bind_iter::<_, (_, sqlite::Value)>([
            (":id", entry.id.into()),
            (":fs_name", entry.fs_name.clone().into()),
            (":fs_mod_time", entry.fs_mod_time.into()),
            (":last_sync_time", entry.last_sync_time.into()),
            (":is_dir", (entry.is_dir as i64).into()),
            (":fs_size", entry.fs_size.into()),
        ])?;

        while let sqlite::State::Row = stmt.next()? {}

        stmt.reset()?;

        Ok(())
    }

    pub fn map_dir_entry_to_parent_dir(
        stmt: &mut sqlite::Statement,
        entry_id: DbId,
        parent_id: DbId,
    ) -> DbResult<()> {
        stmt.bind_iter::<_, (_, sqlite::Value)>([
            (":entry_id", entry_id.into()),
            (":directory_id", parent_id.into()),
        ])?;

        while let sqlite::State::Row = stmt.next()? {}

        stmt.reset()?;

        Ok(())
    }

    // optimization hints before performing many inserts
    pub fn begin(conn: &sqlite::Connection) -> DbResult<()> {
        conn.execute("BEGIN;")?;
        Ok(())
    }

    pub fn commit(conn: &sqlite::Connection) -> DbResult<()> {
        conn.execute("END;")?;
        Ok(())
    }

    pub fn rollback(conn: &sqlite::Connection) -> DbResult<()> {
        conn.execute("ROLLBACK;")?;
        Ok(())
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

        let mut stmts = write::DirStatements::new(&conn).unwrap();

        let new_id = write::max_id(&conn, "dir_entries")+1;
        assert_eq!(new_id, 1);

        write::create_dir_entry(
            &mut stmts.create_dir,
            &DirEntry {
                id: new_id,
                fs_name: "mydir".to_owned(),
                fs_mod_time: 1,
                last_sync_time: 2,
                is_dir: true,
                fs_size: 0,
            },
        )
        .unwrap();

        assert_eq!(write::max_id(&conn, "dir_entries"), 1);

        // TODO get last row insert id
        //conn.last_ro
    }
}
