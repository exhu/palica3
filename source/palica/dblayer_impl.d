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

final class DbLayerImpl : DbReadLayer, DbWriteLayer
{
    /// Throws FailedToOpenDb
    /// Don't forget to call .close() when finished.
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
        prepareStatements();
    }

    void close()
    {
        finalizeStatements();
        db.close();
    }

    override Collection[] enumCollections()
    {
        // TODO
        return [];
    }

    override Collection createCollection(string name, string srcPath, DbId rootId)
    {
        createCollectionStmt.bind(":coll_name", name);
        createCollectionStmt.bind(":fs_path", srcPath);
        createCollectionStmt.bind(":root_id", rootId);
        createCollectionStmt.execute();
        createCollectionStmt.reset();
        return Collection(db.lastInsertRowid(), name, srcPath, rootId);
    }

    override DbId createDirEntry(ref const DirEntry entry)
    {
        createDirEntryStmt.bind(":fs_name", entry.fsName);
        createDirEntryStmt.bind(":fs_mod_time", unixEpochNanoseconds(entry.fsModTime));
        createDirEntryStmt.bind(":last_sync_time", unixEpochNanoseconds(entry.lastSyncTime));
        createDirEntryStmt.execute();
        createDirEntryStmt.reset();
        return db.lastInsertRowid();
    }

    override Nullable!DirEntry getDirEntryById(DbId id)
    {
        selectDirEntryByIdStmt.bind(1, id);
        auto r = selectDirEntryByIdStmt.execute();
        scope(exit) selectDirEntryByIdStmt.reset();
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

private:
    void prepareStatements()
    {
        createDirEntryStmt = db.prepare("INSERT INTO dir_entries(fs_name, fs_mod_time, last_sync_time) " ~
            "VALUES(:fs_name, :fs_mod_time, :last_sync_time);");
        createCollectionStmt = db.prepare("INSERT INTO collections(coll_name, fs_path, root_id) " ~
            "VALUES(:coll_name, :fs_path, :root_id);");
        selectDirEntryByIdStmt = db.prepare("SELECT fs_name, fs_mod_time, last_sync_time FROM dir_entries " ~
            "WHERE id = ?");
    }

    void finalizeStatements()
    {
        createDirEntryStmt.finalize();
        createCollectionStmt.finalize();
        selectDirEntryByIdStmt.finalize();
    }

    void executeSchema()
    {
        import std.file : readText;
        immutable schema = "sql/schema1.sql";
        writeln("reading schema " ~ schema);
        string sql = readText(schema);
        db.run(sql);
    }

    Database db;
    Statement createDirEntryStmt;
    Statement createCollectionStmt;
    Statement selectDirEntryByIdStmt;
}

unittest
{
    auto db = new DbLayerImpl(":memory:");
    scope(exit) db.close();

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
