module palica.dblayer_impl;
import palica.dblayer;
import palica.helpers;
import palica.sqlhelpers;

import d2sqlite3;
import std.stdio : writeln, stderr;
import std.conv : to;
import std.typecons : Unique;

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
    Statement dirEntriesFromParentStmt;
    Statement getCollectionsStmt;

    this(string dbFilename)
    {
        import std.file : exists;

        const bool isNewDb = !exists(dbFilename);
        try
        {
            this.db = Database(dbFilename);
        }
        catch (SqliteException e)
        {
            throw new FailedToOpenDb(dbFilename);
        }
        if (isNewDb)
        {
            executeSchema();
        }

        createDirEntryStmt = db.prepare("INSERT INTO dir_entries(fs_name, fs_mod_time, last_sync_time, is_dir) " ~
                "VALUES(:fs_name, :fs_mod_time, :last_sync_time, :is_dir);");
        createCollectionStmt = db.prepare(
            "INSERT INTO collections(coll_name, fs_path, root_id) " ~
                "VALUES(:coll_name, :fs_path, :root_id);");
        selectDirEntryByIdStmt = db.prepare(
            "SELECT id, fs_name, fs_mod_time, last_sync_time, is_dir FROM dir_entries " ~
                "WHERE id = ?");
        mapDirEntryToParentStmt = db.prepare(
            "INSERT INTO dir_to_sub(directory_id, entry_id) VALUES(" ~
                ":directory_id, :entry_id);");

        // need to select from dir_entries all where id == entry_id from dir_to_sub
        dirEntriesFromParentStmt = db.prepare(
            "SELECT e.id, e.fs_name, e.fs_mod_time, e.last_sync_time, e.is_dir FROM dir_entries e JOIN " ~
                "dir_to_sub d ON d.entry_id = e.id where d.directory_id = ?;");

        getCollectionsStmt = db.prepare(
            "SELECT id, coll_name, fs_path, root_id " ~
                "FROM collections;");
    }

    private void executeSchema()
    {
        import std.file : readText;

        immutable schema = "sql/schema1.sql";
        debug stderr.writeln("reading schema " ~ schema);
        string sql = readText(schema);
        db.run(sql);
    }

    DbId lastRowId()
    {
        return db.lastInsertRowid();
    }

    ~this()
    {
        debug stderr.writeln("~this DbData ", this, " ", db);
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
        debug stderr.writeln("DbLayerImpl.close");
        db.release();
    }

    override Collection[] enumCollections()
    {
        return bindAllAndExec!Collection(db.getCollectionsStmt);
    }

    override Collection createCollection(string name, string srcPath, DbId rootId)
    {
        auto id = idFromExec(db.createCollectionStmt, [
            bindPair(":coll_name", name),
            bindPair(":fs_path", srcPath),
            bindPair(":root_id", rootId),
        ]);
        return Collection(id, name, srcPath, rootId);
    }

    override DbId createDirEntry(ref const DirEntry entry)
    {
        return idFromExec(db.createDirEntryStmt, [
            bindPair(":fs_name", entry.fsName),
            bindPair(":fs_mod_time", unixEpochNanoseconds(entry.fsModTime)),
            bindPair(":last_sync_time", unixEpochNanoseconds(entry.lastSyncTime)),
            bindPair(":is_dir", entry.isDir)
        ]);
    }

    override DirEntry getDirEntryById(DbId id)
    {
        db.selectDirEntryByIdStmt.bind(1, id);
        auto r = db.selectDirEntryByIdStmt.execute();
        scope (exit)
            db.selectDirEntryByIdStmt.reset();
        if (!r.empty())
        {
            auto row = r.front();
            auto e = structFromRow!DirEntry(row);
            r.popFront();
            assert(r.empty());
            return e;
        }
        throw new Exception("no DirEntry with id=" ~ to!string(id));
    }

    override DbId mapDirEntryToParentDir(DbId entryId, DbId parentId)
    {
        return idFromExec(db.mapDirEntryToParentStmt, [
            bindPair(":directory_id", parentId),
            bindPair(":entry_id", entryId),
        ]);
    }

    override DirEntry[] getDirEntriesOfParent(DbId id)
    {
        return bindAllAndExec!(DirEntry)(db.dirEntriesFromParentStmt, id);
    }

    private DbId idFromExec(ref Statement stmt, BindPairBase[] pairs)
    {
        bindPairsAndExec(stmt, pairs);
        return db.lastRowId();
    }

    ~this()
    {
        debug stderr.writeln("~this impl");
    }

private:
    Unique!DbData db;
}

unittest
{
    auto db = new DbLayerImpl(":memory:");
    scope (exit)
    {
        writeln("scope exit KKW");
        db.close();
    }

    import std.datetime : Clock, UTC;

    auto e1 = DirEntry(0, "my", Clock.currTime(UTC()), Clock.currTime(UTC()), true);
    auto id = db.createDirEntry(e1);
    writeln("id=", id, e1);
    auto coll = db.createCollection("mycoll", "srcpath", id);
    writeln("coll=", coll);
    auto e2 = DirEntry(0, "second", Clock.currTime(UTC()), Clock.currTime(UTC()), false);
    auto id2 = db.createDirEntry(e2);
    writeln("id=", id2, e2);
    auto f1 = db.getDirEntryById(id);
    writeln("f1=", f1);
    auto f2 = db.getDirEntryById(id2);
    writeln("f2=", f2);
    assert(e1.fsName == f1.fsName);
    assert(e2.fsName == f2.fsName);
    assert(e1.fsModTime == f1.fsModTime);
    assert(e2.lastSyncTime == f2.lastSyncTime);

    assert(db.mapDirEntryToParentDir(id2, id) == 1);

    auto subs = db.getDirEntriesOfParent(id);
    assert(subs.length == 1);
    assert(subs[0].fsName == "second");

    auto e3 = DirEntry(0, "third", Clock.currTime(UTC()), Clock.currTime(UTC()), false);
    auto id3 = db.createDirEntry(e3);
    assert(id3 != id2);
    assert(db.mapDirEntryToParentDir(id3, id) == 2);

    auto subs2 = db.getDirEntriesOfParent(id);
    writeln("subs2=", subs2);
    assert(subs2.length == 2);
    assert((subs2[0].fsName == "second") || (subs2[1].fsName == "second"));
    assert((subs2[0].fsName == "third") || (subs2[1].fsName == "third"));

    auto colls = db.enumCollections();
    assert(colls.length == 1);
    assert(colls[0].collName == "mycoll");
}

version (none) unittest
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
