module palica.dblayer_impl;
import palica.dblayer;
import palica.helpers;

import std.typecons;
import std.string;
import std.datetime : SysTime;
import d2sqlite3;
import std.stdio : writeln;
import core.internal.gc.impl.conservative.gc;
import std.encoding;


final class FailedToOpenDb : Exception
{
    this(string dbFilename)
    {
        super("Cannot open '" ~ dbFilename ~ "' database.");
    }
}

final class DbData
{
    Database db;
    Statement createDirEntryStmt;
    Statement createCollectionStmt;
    Statement selectDirEntryByIdStmt;
    Statement mapDirEntryToParentStmt;
    
    this(string dbFilename)
    {
        import std.file : exists;
        const bool isNewDb = !exists(dbFilename);
        try
        {
            this.db = Database(dbFilename);
        }
        catch (SqliteException e) {
            throw new FailedToOpenDb(dbFilename);
        }
        if (isNewDb)
        {
            executeSchema();
        }

        createDirEntryStmt = db.prepare("INSERT INTO dir_entries(fs_name, fs_mod_time, last_sync_time) " ~
            "VALUES(:fs_name, :fs_mod_time, :last_sync_time);");
        createCollectionStmt = db.prepare("INSERT INTO collections(coll_name, fs_path, root_id) " ~
            "VALUES(:coll_name, :fs_path, :root_id);");
        selectDirEntryByIdStmt = db.prepare("SELECT fs_name, fs_mod_time, last_sync_time FROM dir_entries " ~
            "WHERE id = ?");
        mapDirEntryToParentStmt = db.prepare("INSERT INTO dir_to_sub(directory_id, entry_id) VALUES(" ~
            ":directory_id, :entry_id);");
    }

    private void executeSchema()
    {
        import std.file : readText;
        immutable schema = "sql/schema1.sql";
        writeln("reading schema " ~ schema);
        string sql = readText(schema);
        db.run(sql);
    }
    
    ~this()
    {
        writeln("~this DbData ", this, " ", db);
    }
}

final class DbLayerImpl : DbReadLayer, DbWriteLayer
{
    /// Throws FailedToOpenDb
    /// Don't forget to call .close() when finished.
    this(string dbFilename)
    {
        this.db = new DbData(dbFilename);
    }

    void close()
    {
        writeln("DbLayerImpl.close");
        db.release();
    }

    override Collection[] enumCollections()
    {
        // TODO
        return [];
    }

    override Collection createCollection(string name, string srcPath, DbId rootId)
    {
        db.createCollectionStmt.bind(":coll_name", name);
        db.createCollectionStmt.bind(":fs_path", srcPath);
        db.createCollectionStmt.bind(":root_id", rootId);
        db.createCollectionStmt.execute();
        db.createCollectionStmt.reset();
        return Collection(db.db.lastInsertRowid(), name, srcPath, rootId);
    }

    override DbId createDirEntry(ref const DirEntry entry)
    {
        db.createDirEntryStmt.bind(":fs_name", entry.fsName);
        db.createDirEntryStmt.bind(":fs_mod_time", unixEpochNanoseconds(entry.fsModTime));
        db.createDirEntryStmt.bind(":last_sync_time", unixEpochNanoseconds(entry.lastSyncTime));
        db.createDirEntryStmt.execute();
        db.createDirEntryStmt.reset();
        return db.db.lastInsertRowid();
    }

    override Nullable!DirEntry getDirEntryById(DbId id)
    {
        db.selectDirEntryByIdStmt.bind(1, id);
        auto r = db.selectDirEntryByIdStmt.execute();
        scope(exit) db.selectDirEntryByIdStmt.reset();
        if (!r.empty())
        {
            auto row = r.front();
            auto e = DirEntry(id, row.peek!string(0),
                sysTimeFromUnixEpochNanoseconds(row.peek!long(1)),
                sysTimeFromUnixEpochNanoseconds(row.peek!long(2)));
            return Nullable!DirEntry(e);
        }
        return Nullable!DirEntry();
    }
    
    override DbId mapDirEntryToParentDir(DbId entryId, DbId parentId)
    {
        db.mapDirEntryToParentStmt.bind(":directory_id", parentId);
        db.mapDirEntryToParentStmt.bind(":entry_id", parentId);
        db.mapDirEntryToParentStmt.execute();
        db.mapDirEntryToParentStmt.reset();
        return db.db.lastInsertRowid();
    }
    
    ~this()
    {
        writeln("~this impl");
    }
    
private:
    Unique!DbData db;
}

unittest
{
    auto db = new DbLayerImpl(":memory:");
    scope(exit) 
    {
        writeln("scope exit KKW");
        db.close();
    }

    import std.datetime : Clock, UTC;
    auto e1 = DirEntry(0, "my", Clock.currTime(UTC()), Clock.currTime(UTC()));
    auto id = db.createDirEntry(e1);
    writeln("id=", id, e1);
    auto coll = db.createCollection("mycoll", "srcpath", id);
    writeln("coll=", coll);
    auto e2 = DirEntry(0, "second", Clock.currTime(UTC()), Clock.currTime(UTC()));
    auto id2 = db.createDirEntry(e2);
    writeln("id=", id2, e2);
    auto f1 = db.getDirEntryById(id);
    writeln("f1=", f1);
    auto f2 = db.getDirEntryById(id2);
    writeln("f2=", f2);
    assert(e1.fsName == f1.get().fsName);
    assert(e2.fsName == f2.get().fsName);
    assert(e1.fsModTime == f1.get().fsModTime);
    assert(e2.lastSyncTime == f2.get().lastSyncTime);
    
    assert(db.mapDirEntryToParentDir(id2, id) == 1);
}

version(none) unittest
{
    try
    {
        auto db = new DbLayerImpl("./nonexistent/nodb");
        db.close();
        assert(false);
    }
    catch (FailedToOpenDb e)
    {
    }
}
