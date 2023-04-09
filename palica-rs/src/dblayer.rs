use sqlite;
use std::fs::read_to_string;
use anyhow::Result;

pub type DbId = i64;

pub struct Collection {
    pub id: DbId,
    pub coll_name: String,
    pub fs_path: String,
    pub root_id: DbId,
    pub glob_filter_id: Option<DbId>,
}

// TODO
pub type SysTime = u64;

pub struct DirEntry {
    pub id: DbId,
    pub fs_name: String,
    pub fs_mod_time: SysTime,
    pub last_sync_time: SysTime,
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
    use crate::dblayer::{Collection, DbId, DirEntry, DbResult};

    pub struct DirStatements<'a> {
        create_dir: sqlite::Statement<'a>,
        map_dir: sqlite::Statement<'a>,
    }

    pub fn create_collection(
        conn: &sqlite::Connection,
        name: &str,
        srcpath: &str,
        rootid: DbId,
    ) -> DbResult<Collection> {
        todo!()
    }

    /// entry.id is ignored.
    pub fn create_dir_entry(
        stmt: &mut sqlite::Statement,
        entry: &DirEntry,
    ) -> DbResult<DbId> {
        todo!()
    }

    pub fn map_dir_entry_to_parent_dir(
        stmt: &mut sqlite::Statement,
        entry_id: DbId,
        parent_id: DbId,
    ) -> DbResult<DbId> {
        todo!()
    }

    // optimization hints before performing many inserts
    pub fn begin(conn: &sqlite::Connection) {}

    pub fn commit(conn: &sqlite::Connection) {}

    pub fn rollback(conn: &sqlite::Connection) {}

    pub fn open_and_make(fname: &str) -> DbResult<sqlite::Connection> {
        use std::path::Path;
        use std::fs::read_to_string;

        let existing =  Path::new(fname).exists();
        let conn = sqlite::Connection::open(fname)?;

        if !existing {
            let schema = "sql/schema1.sql";
            println!("reading schema {}", schema);
            let sql = read_to_string(schema)?;
            conn.execute(sql)?;
        }

        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_and_make() {
        use crate::dblayer::write;

        let r = write::open_and_make(":memory:");
        match r {
            Ok(_) => (),
            Err(_) => panic!("failed to create db"),
        }
        
    }
}

/*
interface DbReadLayer
{
    Collection[] enumCollections();
    DirEntry getDirEntryById(DbId id);
    DirEntry[] getDirEntriesOfParent(DbId id);
    Nullable!Collection getCollectionByName(string name);
    Collection[] getCollectionsWithSamePath(string path);

    GlobPattern[] getGlobPatterns();
    GlobFilter[] getGlobFilters();
    // returns sorted by position
    GlobFilterToPattern[] getFilterPatterns(DbId filterId);
    SettingValue[] getSettings();
}

// On an INSERT, if the ROWID or INTEGER PRIMARY KEY column is not explicitly
// given a value, then it will be filled automatically with an unused integer,
// usually one more than the largest ROWID currently in use. This is true
// regardless of whether or not the AUTOINCREMENT keyword is used.
// https://www.sqlite.org/autoinc.html

interface DbWriteLayer
{
    // errors
    final class CollectionAlreadyExists : Exception
    {
        this(string name, DbId dbId)
        {
            import std.string : format;

            super(format("Collection '%s' with id '%d' already exists.", name, dbId));
        }
    }

    /// Throws CollectionAlreadyExists, DbError
    Collection createCollection(string name, string srcPath, DbId rootId, Nullable!DbId);
    /// Throws DbError
    /// entry.id is ignored.
    DbId createDirEntry(ref const DirEntry entry);

    DbId mapDirEntryToParentDir(DbId entryId, DbId parentId);

    // optimization hints before performing many inserts
    void beginTransaction();
    void commitTransaction();
    void rollbackTransaction();

    // deletes entry and all dependent items (if it's dir, then subdirs)
    // dir_to_sub, tag_to_dir_entry, mime_to_dir_entry
    void deleteDirEntry(DbId id, bool newTransaction = true);

    void deleteCollection(Collection col);
}

pub trait DbLayer {

}
*/

/*
struct DbLayer<'a> {
    db: Box<RefCell<sqlite::Connection>>,
    prepared: Option<Prepared<'a>>,
}

struct Prepared<'conn> {
    create_dir_entry:  sqlite::Statement<'conn>,
}

impl Prepared <'_> {
    pub fn new<'a>(db: &'a sqlite::Connection) -> Prepared<'a> {
        Prepared {
            create_dir_entry: db.prepare("INSERT INTO dir_entries(fs_name,
            fs_mod_time, last_sync_time) VALUES(:fs_name, :fs_mod_time, :last_sync_time);").unwrap(),
        }
    }

}

impl DbLayer<'_> {
    pub fn new(filename: &str) -> DbLayer {
        let db = Box::new(RefCell::new(sqlite::open(filename).expect("Failed to open db")));
        let sql = read_to_string("schema1.sql").unwrap();
        db.borrow().execute(sql).unwrap();

        DbLayer {
            db,
            prepared: None
        }

    }

    pub fn prep (&mut self) {
        self.prepared = Some(Prepared::new(&self.db.borrow()));
    }
}
*/
