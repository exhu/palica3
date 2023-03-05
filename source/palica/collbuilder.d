module palica.collbuilder;

import palica.dblayer;
import palica.fslayer;
import std.stdio : writeln;

interface ScanningEvents
{
    // just added to db
    void onNewDirEntry(ref const DirEntry dir);

    // updated db entry
    void onChangedDirEntry(ref const DirEntry dir);
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

    Collection createCollection(string name, string path)
    {
        import std.path : absolutePath;
        import palica.fsdb_helpers : dirEntryFromFsDirEntry;

        immutable srcPath = path.absolutePath();
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
        return col;
    }

    /// call after createCollection to populate directory tree
    /// root = root directory
    void populateDirEntries(DbId rootId, string rootPath)
    {
        auto entries = fsRead.dirEntries(rootPath);
        auto subEntries = writeFsEntriesToDb(rootId, entries);
        // TODO repeat for subEntries
        
    }
    
    struct SubDir
    {
        DbId id;
        string fsName;
    }
    
    private SubDir[] writeFsEntriesToDb(DbId dirId, FsDirEntry[] dirEntries)
    {
        SubDir[] result;
        foreach(d; dirEntries)
        {
            import palica.fsdb_helpers : dirEntryFromFsDirEntry;
            DirEntry e = dirEntryFromFsDirEntry(d);
            e.id = dbWrite.createDirEntry(e);
            dbWrite.mapDirEntryToParentDir(e.id, dirId);
            if (e.isDir)
            {
                result ~= SubDir(e.id, e.fsName);
            }
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
    auto col = cb.createCollection("sample-col", "sample-data");
    writeln("col=", col);
    cb.populateDirEntries(col.rootId, col.fsPath);
    auto rootEntries = db.getDirEntriesOfParent(col.rootId);
    writeln("rootEntries=", rootEntries);

}
