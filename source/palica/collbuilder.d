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

}
