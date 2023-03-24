/+
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
+/
module palica.dblayer_impl;
public import palica.dblayer;
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

private struct DelDirStatements
{
    this(ref Database db)
    {
        dirToSub = db.prepare("DELETE FROM dir_to_sub WHERE entry_id = ?1");
        dirEntries = db.prepare("DELETE FROM dir_entries WHERE id = ?1");
        tagToDir = db.prepare("DELETE FROM tag_to_dir_entry WHERE dir_entry_id = ?1");
        lastEdit = db.prepare("DELETE FROM last_edit WHERE dir_entry_id = ?1");
        mimeToDir = db.prepare("DELETE FROM mime_to_dir_entry WHERE dir_entry_id = ?1");
    }

    void execForId(DbId id)
    {
        foreach (ref s; this.tupleof)
        {
            bindAllAndExecNoResult(s, id);
        }
    }

private:
    Statement dirToSub;
    Statement dirEntries;
    Statement tagToDir;
    Statement lastEdit;
    Statement mimeToDir;
}

private final class DbData
{
    Database db;
    Statement createDirEntryStmt;
    Statement createCollectionStmt;
    Statement selectDirEntryByIdStmt;
    Statement mapDirEntryToParentStmt;
    Statement dirEntriesFromParentStmt;
    Statement getCollectionsStmt;
    Statement getCollectionByNameStmt;
    Statement getCollectionsByPathStmt;
    Statement getDirChildIdsStmt;
    DelDirStatements delDirStatements;
    Statement deleteCollectionStmt;

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

        createDirEntryStmt = db.prepare(
            "INSERT INTO dir_entries(fs_name, fs_mod_time, last_sync_time, is_dir, fs_size)
                VALUES(:fs_name, :fs_mod_time, :last_sync_time, :is_dir, :fs_size);");
        createCollectionStmt = db.prepare(
            "INSERT INTO collections(coll_name, fs_path, root_id)
                VALUES(:coll_name, :fs_path, :root_id);");
        selectDirEntryByIdStmt = db.prepare(
            "SELECT id, fs_name, fs_mod_time, last_sync_time, is_dir, fs_size
                FROM dir_entries WHERE id = ?");
        mapDirEntryToParentStmt = db.prepare(
            "INSERT INTO dir_to_sub(directory_id, entry_id) VALUES(
                :directory_id, :entry_id);");

        // need to select from dir_entries all where id == entry_id from dir_to_sub
        dirEntriesFromParentStmt = db.prepare(
            "SELECT e.id, e.fs_name, e.fs_mod_time, e.last_sync_time, e.is_dir,
                e.fs_size
                FROM dir_entries e JOIN
                dir_to_sub d ON d.entry_id = e.id where d.directory_id = ?;");

        getCollectionsStmt = db.prepare(
            "SELECT id, coll_name, fs_path, root_id FROM collections;");

        getCollectionByNameStmt = db.prepare(
            "SELECT id, coll_name, fs_path, root_id FROM collections
                WHERE coll_name = ?");

        getCollectionsByPathStmt = db.prepare(
            "SELECT id, coll_name, fs_path, root_id FROM collections
                WHERE fs_path = ?");

        delDirStatements = DelDirStatements(db);
        getDirChildIdsStmt = db.prepare("SELECT entry_id FROM dir_to_sub
            WHERE directory_id = ?1");

        deleteCollectionStmt = db.prepare("DELETE FROM collections
            WHERE id = ?1");
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

struct AutoDb
{
    DbLayerImpl db;

    this(string dbFilename)
    {
        db = new DbLayerImpl(dbFilename);
    }

    ~this()
    {
        db.close();
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
            bindPair(":is_dir", entry.isDir),
            bindPair(":fs_size", entry.fsSize),
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

    override void beginTransaction()
    {
        db.db.execute("BEGIN;");
    }

    override void commitTransaction()
    {
        db.db.execute("END;");
    }

    override void rollbackTransaction()
    {
        db.db.execute("ROLLBACK;");
    }

    import std.typecons : Nullable, nullable;

    Nullable!Collection getCollectionByName(string name)
    {
        auto found = bindAllAndExec!(Collection)(db.getCollectionByNameStmt, name);
        if (found.length == 1)
        {
            return nullable(found[0]);
        }
        return Nullable!Collection();
    }

    Collection[] getCollectionsWithSamePath(string path)
    {
        auto found = bindAllAndExec!(Collection)(db.getCollectionsByPathStmt,
            path);
        return found;
    }

    override void deleteDirEntry(DbId id, bool newTransaction = true)
    {
        struct ResId
        {
            DbId id;
        }

        if (newTransaction)
            beginTransaction();

        scope (failure)
            if (newTransaction)
                rollbackTransaction();

        // recursively delete
        DbId[] toProcess = [id];
        while (toProcess.length > 0)
        {
            DbId curId = toProcess[0];
            db.delDirStatements.execForId(curId);
            ResId[] foundIds = bindAllAndExec!ResId(db.getDirChildIdsStmt,
                curId);
            toProcess.reserve(toProcess.length + foundIds.length);
            foreach (i; foundIds)
                toProcess ~= i.id;

            toProcess = toProcess[1 .. $];
        }

        if (newTransaction)
            commitTransaction();
    }

    override void deleteCollection(Collection col)
    {
        beginTransaction();
        scope (failure)
            rollbackTransaction();

        bindAllAndExecNoResult(db.deleteCollectionStmt, col.id);
        deleteDirEntry(col.rootId, false);

        commitTransaction();
    }

    override GlobPattern[] getGlobPatterns()
    {
        auto stmt = prepare("SELECT id, regexp FROM glob_patterns");
        return bindAllAndExec!GlobPattern(stmt);
    }

    override GlobFilter[] getGlobFilters()
    {
        auto stmt = prepare("SELECT id, name FROM glob_filters");
        return bindAllAndExec!GlobFilter(stmt);
    }
    // returns sorted by position
    override GlobFilterToPattern[] getFilterPatterns(DbId filterId)
    {
        auto stmt = prepare("SELECT id, filter_id, glob_pattern_id,
           include, position FROM glob_filter_to_pattern WHERE filter_id = ?1
           ORDER BY position");
        return bindAllAndExec!GlobFilterToPattern(stmt, filterId);
    }

    Statement prepare(string sql)
    {
        return db.db.prepare(sql);
    }

    ~this()
    {
        debug stderr.writeln("~this DbLayerImpl");
    }

private:
    Unique!DbData db;
}

unittest
{
    writeln("DbLayerImpl long test start.");
    scope (exit)
        writeln("DbLayerImpl long test end.");

    auto adb = AutoDb(":memory:");
    auto db = adb.db;
    scope (exit)
    {
        writeln("scope exit KKW");
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

    auto coll2 = db.createCollection("mycoll2", "srcpath", id);

    auto colByName = db.getCollectionByName("mycoll2");
    assert(!colByName.isNull());
    assert(colByName.get().collName == "mycoll2");
    assert(db.getCollectionByName("asdasd").isNull());

    auto cols = db.getCollectionsWithSamePath("srcpath");
    writeln("cols =", cols);
    assert(cols.length != 0);
    assert(cols[0].collName == "mycoll2" || cols[0].collName == "mycoll");
}

unittest
{
    writeln("delDirEntries test...");
    import std.datetime : Clock, UTC;

    // delDirEntry
    auto adb = AutoDb(":memory:");
    auto db = adb.db;
    auto e1 = DirEntry(0, "dir", Clock.currTime(UTC()), Clock.currTime(UTC()), true);
    auto e2 = DirEntry(0, "subdir", Clock.currTime(UTC()), Clock.currTime(UTC()), true);
    auto e3 = DirEntry(0, "leaf", Clock.currTime(UTC()), Clock.currTime(UTC()),
        false);
    e1.id = db.createDirEntry(e1);
    e2.id = db.createDirEntry(e2);
    e3.id = db.createDirEntry(e3);

    db.mapDirEntryToParentDir(e2.id, e1.id);
    db.mapDirEntryToParentDir(e3.id, e2.id);

    auto e1Leaves = db.getDirEntriesOfParent(e1.id);
    auto e2Leaves = db.getDirEntriesOfParent(e2.id);

    writeln(e1Leaves);
    assert(e1Leaves[0] == e2);
    writeln(e2Leaves);
    assert(e2Leaves[0] == e3);

    auto found1 = db.getDirEntryById(e3.id);
    assert(found1 == e3);

    db.deleteDirEntry(e1.id);

    try
    {
        auto found = db.getDirEntryById(e3.id);
        writeln("must not reach there. found=", found);
        assert(false);
    }
    catch (Exception e)
    {
        writeln("e3 not found -- ok.");
    }
}

unittest
{
    writeln("deleteCollection test...");
    import std.datetime : Clock, UTC;

    // delDirEntry
    auto adb = AutoDb(":memory:");
    auto db = adb.db;
    auto e1 = DirEntry(0, "dir", Clock.currTime(UTC()), Clock.currTime(UTC()), true);
    auto e2 = DirEntry(0, "subdir", Clock.currTime(UTC()), Clock.currTime(UTC()), true);
    auto e3 = DirEntry(0, "leaf", Clock.currTime(UTC()), Clock.currTime(UTC()),
        false);
    e1.id = db.createDirEntry(e1);
    e2.id = db.createDirEntry(e2);
    e3.id = db.createDirEntry(e3);

    db.mapDirEntryToParentDir(e2.id, e1.id);
    db.mapDirEntryToParentDir(e3.id, e2.id);
    auto coll2 = db.createCollection("mycoll2", "srcpath", e1.id);

    auto found1 = db.getCollectionByName("mycoll2");
    assert(!found1.isNull);
    assert(found1.get() == coll2);

    auto countStmt = db.prepare("SELECT COUNT(*) FROM dir_entries");
    struct Res
    {
        long count;
    }

    auto res1 = bindAllAndExec!Res(countStmt);
    assert(res1[0].count == 3);

    db.deleteCollection(coll2);

    auto found2 = db.getCollectionByName("mycoll2");
    assert(found2.isNull);

    auto res2 = bindAllAndExec!Res(countStmt);
    assert(res2[0].count == 0);
}

unittest
{
    writeln("filters test");
    auto adb = AutoDb(":memory:");
    auto db = adb.db;
    auto patterns = db.getGlobPatterns();
    writeln("patterns =", patterns);
    assert(patterns.length > 0);
    auto filters = db.getGlobFilters();
    writeln("filters =", filters);
    assert(filters.length > 0);
    auto filterPatterns = db.getFilterPatterns(filters[0].id);
    writeln("filter ", filters[0].name, " patterns=", filterPatterns);
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
