use sqlite;
use std::fs::read_to_string;

struct DbLayer<'a> {
    db: Box<sqlite::Connection>,
    create_dir_entry_stmt: sqlite::Statement<'a>,
}

impl DbLayer<'_> {
    fn new(filename: &str) -> DbLayer {
        let db = Box::new(sqlite::open(filename).unwrap());
        let sql = read_to_string("schema1.sql").unwrap();
        db.execute(sql).unwrap();
        let dir_stmt = db.prepare("INSERT INTO dir_entries(fs_name,
            fs_mod_time, last_sync_time) VALUES(:fs_name, :fs_mod_time, :last_sync_time);").unwrap();

        DbLayer {
            db,
            create_dir_entry_stmt: dir_stmt,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
