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
module palica.cmdtool;

import std.stdio : writefln, writeln, stderr;
import palica.collbuilder;
import palica.dblayer_impl;
import palica.dblayer;
import palica.fslayer_impl;
import palica.fslayer;
import palica.fsdb_helpers;
import std.typecons : Nullable;

private bool continueWithDupPath(DbReadLayer db, FsReadLayer fs, string path, bool ask)
{
    auto normalized = fs.normalizedAbsPath(path);
    Collection[] found = db.getCollectionsWithSamePath(normalized);
    if (found.length != 0)
    {
        writefln("Warning! Existing collections with path '%s' found:", normalized);
        foreach (c; found)
            writeln(c.collName);

        if (ask)
        {
            import palica.tui_helpers : promptYesNo;

            return promptYesNo("Continue?");
        }
    }
    return true;
}

int collectionAdd(string dbFilename, string name, string path, bool verbose,
    bool ask)
{
    writefln("Adding collection '%s' into '%s' from '%s':", name, dbFilename,
        path);
    auto adb = AutoDb(dbFilename);
    auto db = adb.db;

    auto existingCol = db.getCollectionByName(name);
    if (!existingCol.isNull)
    {
        writeln("Error! There's already a collection with that name.");
        return 1;
    }

    auto fs = new FsLayerImpl();
    if (!continueWithDupPath(db, fs, path, ask))
    {
        stderr.writeln("Abort for similar collection path.");
        return 1;
    }

    long entries = 1;
    auto listener = new class CollectionListener
    {
        override void onNewDirEntry(ref const DirEntry e)
        {
            if (verbose)
                writefln("Found %s", e.fsName);

            entries += 1;
        }
    };

    auto cb = CollBuilder(db, fs);
    auto col = cb.createCollection(name, path, listener);

    cb.populateDirEntriesInDepth(col.rootId, path, listener);
    writefln("Finished with %d entries.", entries);
    return 0;
}

private void printCollection(ref const Collection c, bool verbose)
{
    if (verbose)
        writefln("%d, \"%s\", \"%s\", root_id:%d", c.id, c.collName,
            c.fsPath, c.rootId);
    else
        writefln("\"%s\": \"%s\"", c.collName, c.fsPath);
}

private Nullable!Collection findCol(string dbFilename, DbReadLayer db, string name)
{
    auto found = db.getCollectionByName(name);

    if (found.isNull())
        stderr.writefln("Collection \"%s\" not found in \"%s\".", name,
            dbFilename);
    return found;
}

int collectionTree(string dbFilename, string name, bool verbose)
{
    auto adb = AutoDb(dbFilename);
    auto db = adb.db;

    auto found = findCol(dbFilename, db, name);
    if (!found.isNull())
    {
        printCollection(found.get(), verbose);
        auto root = db.getDirEntryById(found.get().rootId);
        dumpDirEntry(root, 0, verbose);
        dumpDirEntryAsTree(root.id, db, 1, verbose);
    }
    else
    {
        return 1;
    }

    return 0;
}

int collectionList(string dbFilename, bool verbose)
{
    auto adb = AutoDb(dbFilename);
    auto db = adb.db;

    writefln("Collections in \"%s\":", dbFilename);
    auto cols = db.enumCollections();
    foreach (ref Collection c; cols)
    {
        printCollection(c, verbose);
    }

    return 0;
}

int collectionSync(string dbFilename, string name, bool verbose, bool ask)
{
    auto fs = new FsLayerImpl();
    auto adb = AutoDb(dbFilename);
    auto db = adb.db;

    auto found = findCol(dbFilename, db, name);
    if (found.isNull())
        return 1;

    auto path = found.get().fsPath;
    writefln("Syncing collection '%s' into '%s' from '%s':", name, dbFilename,
        path);

    if (!continueWithDupPath(db, fs, path, ask))
    {
        stderr.writeln("Abort for similar collection path.");
        return 1;
    }
    
    // TODO

    /+
    long entries = 1;
    auto listener = new class CollectionListener
    {
        override void onNewDirEntry(ref const DirEntry e)
        {
            if (verbose)
                writefln("Found %s", e.fsName);

            entries += 1;
        }
    };

    auto cb = CollBuilder(db, fs);
    auto col = cb.createCollection(name, path, listener);

    cb.populateDirEntriesInDepth(col.rootId, path, listener);
    writefln("Finished with %d entries.", entries);
    +/
    return 0;
}
