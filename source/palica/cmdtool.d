module palica.cmdtool;

import std.stdio : writefln, writeln;
import palica.collbuilder;
import palica.dblayer_impl;
import palica.dblayer;
import palica.fslayer_impl;
import palica.fsdb_helpers;

int collectionAdd(string dbFilename, string name, string path, bool verbose)
{
    writefln("Adding collection '%s' into '%s' from '%s':", name, dbFilename, path);
    auto fs = new FsLayerImpl();
    auto db = new DbLayerImpl(dbFilename);
    scope (exit)
        db.close();

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

int collectionTree(string dbFilename, string name)
{
    // TODO
    return 0;
}

int collectionList(string dbFilename)
{
    // TODO
    return 0;
}
