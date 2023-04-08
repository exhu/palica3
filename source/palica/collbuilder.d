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
module palica.collbuilder;

import palica.dblayer;
import palica.fslayer;
import std.stdio : writeln;
import palica.fsdb_helpers;
import palica.globfilter;

/*
   Two scenarios: 1) new collection is being populated (this module),
   2) syncronizing existing collection
*/

struct CollBuilder
{
    interface CollectionListener
    {
        // just added to db
        void onNewDirEntry(ref const DirEntry dir);
    }

    this(DbWriteLayer aDbWrite, FsReadLayer aFsRead)
    {
        dbWrite = aDbWrite;
        fsRead = aFsRead;
    }

    Collection createCollection(string name, string path, Nullable!DbId filterId, CollectionListener listener = null)
    {
        import palica.fsdb_helpers : dirEntryFromFsDirEntry;

        immutable srcPath = fsRead.normalizedAbsPath(path);
        if (!fsRead.pathExists(srcPath))
        {
            throw new Exception(
                "Cannot create collection '" ~
                    name ~ "' -- cannot read path '" ~ srcPath ~ "'");
        }

        FsDirEntry fsEntry = fsRead.dirEntry(srcPath);
        auto dirEnt = dirEntryFromFsDirEntry(fsEntry);
        dirEnt.id = dbWrite.createDirEntry(dirEnt);
        auto col = dbWrite.createCollection(name, srcPath, dirEnt.id,
            filterId);
        if (listener)
            listener.onNewDirEntry(dirEnt);
        return col;
    }

    /// call after createCollection to populate directory tree
    /// root = root directory
    private SubDir[] populateDirEntries(DbId rootId, string rootPath,
        CollectionListener listener, ref const FsGlobFilter globFilter)
    {
        import std.algorithm : filter;
        import std.array : array;

        auto entries = filter!(e => globFilter.accept(e.name))(fsRead.dirEntries(rootPath)).array;
        auto subEntries = writeFsEntriesToDb(rootId, entries, listener);
        return subEntries;
    }

    void populateDirEntriesInDepth(DbId rootId, string rootPath,
        CollectionListener listener, ref const FsGlobFilter globFilter)
    {
        dbWrite.beginTransaction();
        scope (exit)
            dbWrite.commitTransaction();

        SubDir[] dirs = populateDirEntries(rootId, rootPath, listener,
            globFilter);
        while (dirs.length > 0)
        {
            SubDir[] subDirs;
            foreach (ref d; dirs)
            {
                subDirs ~= populateDirEntries(d.id, d.path, listener, globFilter);
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

    size_t newDirEntries = 0;
    auto listener = new class CollBuilder.CollectionListener
    {
        override void onNewDirEntry(ref const DirEntry dir)
        {
            writeln("onNewDirEntry: ", dir);
            newDirEntries++;
        }
    };

    auto col = cb.createCollection("sample-col", "./sample-data",
        Nullable!DbId(), listener);
    writeln("col=", col);

    auto fsFilter = FsGlobFilter();
    cb.populateDirEntriesInDepth(col.rootId, col.fsPath, listener, fsFilter);
    writeln("dump tree:");
    auto rootEntry = db.getDirEntryById(col.rootId);
    dumpDirEntry(rootEntry);
    dumpDirEntryAsTree(col.rootId, db, 1);

    assert(newDirEntries >= 2);
}
