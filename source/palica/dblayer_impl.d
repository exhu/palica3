module palica.dblayer_impl;
import palica.dblayer;
import std.typecons;
import std.string;
import std.datetime : SysTime;
import d2sqlite3;
import std.stdio : writeln;

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
        createDirEntryStmt.bind(":fs_mod_time", entry.fsModTime.stdTime);
        createDirEntryStmt.bind(":last_sync_time", entry.lastSyncTime.stdTime);
        createDirEntryStmt.execute();
        createDirEntryStmt.reset();
        return db.lastInsertRowid();
    }

private:
    void prepareStatements()
    {
        createDirEntryStmt = db.prepare("INSERT INTO dir_entries(fs_name, fs_mod_time, last_sync_time) " ~
            "VALUES(:fs_name, :fs_mod_time, :last_sync_time);");
        createCollectionStmt = db.prepare("INSERT INTO collections(coll_name, fs_path, root_id) " ~
            "VALUES(:coll_name, :fs_path, :root_id);");
    }

    void finalizeStatements()
    {
        createDirEntryStmt.finalize();
        createCollectionStmt.finalize();
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
}

unittest
{
    auto db = new DbLayerImpl(":memory:");
    db.close();
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
