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
import palica.fsdb_helpers;

private bool continueWithDupPath(DbReadLayer db, string path, bool ask)
{
    // TODO normalize path, find in db collections with same path
    // TODO prompt for continuation
    return true;
}

int collectionAdd(string dbFilename, string name, string path, bool verbose, bool ask)
{
    writefln("Adding collection '%s' into '%s' from '%s':", name, dbFilename, path);
    auto fs = new FsLayerImpl();
    auto db = new DbLayerImpl(dbFilename);
    scope (exit)
        db.close();
    
    if (!continueWithDupPath(db, path, ask))
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

int collectionTree(string dbFilename, string name, bool verbose)
{
    auto db = new DbLayerImpl(dbFilename);
    scope (exit)
        db.close();

    auto found = db.getCollectionByName(name);
    if (!found.isNull())
    {
        printCollection(found.get(), verbose);
        auto root = db.getDirEntryById(found.get().rootId);
        dumpDirEntry(root, 0, verbose);
        dumpDirEntryAsTree(root.id, db, 1, verbose);
    }
    else
    {
        stderr.writefln("Collection \"%s\" not found in \"%s\".", name,
            dbFilename);
        return 1;
    }

    return 0;
}

int collectionList(string dbFilename, bool verbose)
{
    auto db = new DbLayerImpl(dbFilename);
    scope (exit)
        db.close();

    writefln("Collections in \"%s\":", dbFilename);
    auto cols = db.enumCollections();
    foreach (ref Collection c; cols)
    {
        printCollection(c, verbose);
    }

    return 0;
}
