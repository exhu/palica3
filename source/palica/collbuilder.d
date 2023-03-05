module palica.collbuilder;

import palica.dblayer;
import palica.fslayer;
import std.stdio : writeln;
import palica.fsdb_helpers;

interface CollectionListener
{
    // just added to db
    void onNewDirEntry(ref const DirEntry dir);

    // updated db entry
    //void onChangedDirEntry(ref const DirEntry dir);
}

/*
   Two scenarios: 1) new collection is being populated,
   2) syncronizing existing collection
*/

struct CollBuilder
{
    this(DbWriteLayer aDbWrite, FsReadLayer aFsRead)
    {
        dbWrite = aDbWrite;
        fsRead = aFsRead;
    }

    Collection createCollection(string name, string path,
        CollectionListener listener = null)
    {
        import std.path : absolutePath, buildNormalizedPath;
        import palica.fsdb_helpers : dirEntryFromFsDirEntry;

        immutable srcPath = path.absolutePath().buildNormalizedPath();
        if (!fsRead.pathExists(srcPath))
        {
            throw new Exception(
                "Cannot create collection '" ~
                    name ~ "' -- cannot read path '" ~ srcPath ~ "'");
        }

        FsDirEntry fsEntry = fsRead.dirEntry(srcPath);
        auto dirEnt = dirEntryFromFsDirEntry(fsEntry);
        dirEnt.id = dbWrite.createDirEntry(dirEnt);
        auto col = dbWrite.createCollection(name, srcPath, dirEnt.id);
        if (listener)
            listener.onNewDirEntry(dirEnt);
        return col;
    }

    /// call after createCollection to populate directory tree
    /// root = root directory
    private SubDir[] populateDirEntries(DbId rootId, string rootPath,
        CollectionListener listener)
    {
        auto entries = fsRead.dirEntries(rootPath);
        auto subEntries = writeFsEntriesToDb(rootId, entries, listener);
        return subEntries;
    }

    void populateDirEntriesInDepth(DbId rootId, string rootPath,
        CollectionListener listener)
    {
        dbWrite.beginTransaction();
        scope(exit)
            dbWrite.commitTransaction();

        SubDir[] dirs = populateDirEntries(rootId, rootPath, listener);
        while (dirs.length > 0)
        {
            SubDir[] subDirs;
            foreach (ref d; dirs)
            {
                subDirs ~= populateDirEntries(d.id, d.path, listener);
            }
            dirs = subDirs;
        }
    }

    struct SubDir
    {
        DbId id;
        string path;
    }

    private SubDir[] writeFsEntriesToDb(DbId dirId, FsDirEntry[] dirEntries,
    CollectionListener listener)
    {
        SubDir[] result;
        foreach (d; dirEntries)
        {
            import palica.fsdb_helpers : dirEntryFromFsDirEntry;

            DirEntry e = dirEntryFromFsDirEntry(d);
            e.id = dbWrite.createDirEntry(e);
            dbWrite.mapDirEntryToParentDir(e.id, dirId);
            if (e.isDir)
            {
                result ~= SubDir(e.id, d.name);
            }
            if (listener)
                listener.onNewDirEntry(e);
        }
        return result;
    }

    /* will go to separate module
    void syncCollection(const ref Collection col)
    {
        // TODO
    }
    */

private:
    DbWriteLayer dbWrite;
    FsReadLayer fsRead;
}

unittest
{
    writeln("CollBuilder tests start.");
    scope (exit)
        writeln("CollBuilder tests end.");

    import palica.dblayer_impl;
    import palica.fslayer_impl;

    auto db = new DbLayerImpl(":memory:");
    scope (exit)
        db.close();

    auto fs = new FsLayerImpl;

    auto cb = CollBuilder(db, fs);
    auto col = cb.createCollection("sample-col", "./sample-data");
    writeln("col=", col);

    auto listener = new class CollectionListener
    {
        override void onNewDirEntry(ref const DirEntry dir)
        {

        }
    };

    cb.populateDirEntriesInDepth(col.rootId, col.fsPath, listener);
    writeln("dump tree:");
    auto rootEntry = db.getDirEntryById(col.rootId);
    dumpDirEntry(rootEntry);
    dumpDirEntryAsTree(col.rootId, db, 1);
}
