module palica.dblayer_impl;
import etc.c.sqlite3;
import palica.dblayer;
import std.typecons;
import std.string;
import std.datetime : SysTime;

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
        int rc = sqlite3_open(dbFilename.toStringz, &pdb);
        if (rc)
        {
            sqlite3_close(pdb);
            throw new FailedToOpenDb(dbFilename);
        }
    }

    void close()
    {
        sqlite3_close(pdb);
    }

    override Collection[] enumCollections()
    {
        // TODO
        return [];
    }

    override Collection createCollection(string name, string srcPath, ref const DirEntry rootEntry)
    {
        auto sql = format(
            "INSERT INTO collections(coll_name, fs_path, root_id) " ~
            "VALUES('%s' %d, %d);", rootEntry.fsModTime.stdTime,
            rootEntry.lastSyncTime.stdTime);
        execSql(sql);
        DbId lastId = sqlite3_last_insert_rowid(pdb);
        return Collection(lastId, name, srcPath, rootEntry.id);
    }

    override DbId createDirEntry(ref const DirEntry entry)
    {
        auto sql = format(
            "INSERT INTO dir_entries(fs_name, fs_mod_time, last_sync_time) " ~
            "VALUES('/', %d, %d);", entry.fsModTime.stdTime, entry.lastSyncTime.stdTime);
        
        execSql(sql);

        return sqlite3_last_insert_rowid(pdb);
    }

private:
    struct ResultItem
    {
        string col, value;
    }
    /// Throws DbError
    ResultItem[] execSql(string sql)
    {
        char* zErrMsg;
        ResultItem[] res = [];
        int rc = sqlite3_exec(pdb, sql.toStringz, &callback, &res, &zErrMsg);
        if (rc != SQLITE_OK)
        {
            string err = cast(string)fromStringz(zErrMsg).dup;
            sqlite3_free(zErrMsg);
            throw new DbError(err);
        }
        return res;
    }

    static extern(C) int callback(void* userData, int argc, char** argv, char** azColName)
    {
        import std.stdio : writeln;
        writeln("sqlite callback");
        ResultItem[] res = *cast(ResultItem[]*)userData;
        for (int i = 0; i < argc; i++)
        {
            import core.stdc.stdio : printf;
            printf("%s = %s\n", azColName[i], argv[i] ? argv[i] : "NULL");
            res ~= ResultItem(cast(string)fromStringz(azColName[i]).dup,
                cast(string)fromStringz(argv[i] ? argv[i] : "NULL").dup);
        }
        return 0;
    }

    sqlite3* pdb;
}

unittest
{
    import core.stdc.stdio : printf;
    import std.stdio: writeln;

    printf("sqlite version = '%s'\n", sqlite3_libversion());

    auto db = new DbLayerImpl("test.db");
    auto res = db.execSql("CREATE TABLE app_info(info_key TEXT UNIQUE NOT NULL, info_value TEXT NOT NULL);");
    writeln(res);
    db.close();
}

unittest
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
