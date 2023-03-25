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

import palica.collbuilder;
import palica.dblayer_impl;
import palica.dblayer;
import palica.fslayer_impl;
import palica.fslayer;
import palica.fsdb_helpers;
import palica.tui_helpers;
import palica.dbhelpers;
import palica.globfilter;

import std.typecons : Nullable;
import std.algorithm : map, filter;
import std.array : array;
import std.format : format;

private bool continueWithDupPath(DbReadLayer db, FsReadLayer fs, string path, bool ask)
{
    auto normalized = fs.normalizedAbsPath(path);
    Collection[] found = db.getCollectionsWithSamePath(normalized);
    if (found.length != 0)
    {
        auto names = found.map!(c => c.collName).array;
        return prompt(PromptExistingCollectionsFound(normalized, names), ask);
    }
    return true;
}

int collectionAdd(string dbFilename, string name, string path, bool verbose,
    bool ask)
{
    displayInfo(InfoAddingCollection(name, dbFilename, path));

    auto adb = AutoDb(dbFilename);
    auto db = adb.db;

    auto existingCol = db.getCollectionByName(name);
    if (!existingCol.isNull)
    {
        displayError(ErrorAddingCollectionExists(name));
        return 1;
    }

    auto fs = new FsLayerImpl();
    if (!continueWithDupPath(db, fs, path, ask))
    {
        displayInfoSimilarCollectionPath();
        return 1;
    }

    long entries = 1;
    auto listener = new class CollectionListener
    {
        override void onNewDirEntry(ref const DirEntry e)
        {
            if (verbose)
                displayInfo("Found %s".format(e.fsName));

            entries += 1;
        }
    };
    auto settings = settingsMapFromDb(db.getSettings());
    auto defaultFilterId = getDefaultFilterId(settings);


    FsGlobFilter fsGlobFilterFromId(DbId filterId)
    {
        import palica.fsdb_helpers : fsGlobFilterFromDb;
        return fsGlobFilterFromDb(db.getFilterPatterns(filterId),
                db.getGlobPatterns());
    }

    auto fsGlobFilter = defaultFilterId.isNull ? FsGlobFilter() :
        fsGlobFilterFromId(defaultFilterId.get());

    auto cb = CollBuilder(db, fs);
    auto col = cb.createCollection(name, path, defaultFilterId, listener);

    cb.populateDirEntriesInDepth(col.rootId, path, listener, fsGlobFilter);
    displayInfo("Finished with %d entries.".format(entries));
    return 0;
}

private void printCollection(ref const Collection c, bool verbose)
{
    if (verbose)
        displayInfo("%d, \"%s\", \"%s\", root_id:%d".format(c.id, c.collName,
                c.fsPath, c.rootId));
    else
        displayInfo("\"%s\": \"%s\"".format(c.collName, c.fsPath));
}

private Nullable!Collection findCol(string dbFilename, DbReadLayer db, string name)
{
    auto found = db.getCollectionByName(name);

    if (found.isNull())
        displayError("Collection \"%s\" not found in \"%s\".".format(name,
                dbFilename));
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

    displayInfo("Collections in \"%s\":".format(dbFilename));
    auto cols = db.enumCollections();
    foreach (ref Collection c; cols)
    {
        printCollection(c, verbose);
    }

    return 0;
}

int collectionRemove(string dbFilename, string name, bool ask)
{
    // don't create db, if there's none
    auto fs = new FsLayerImpl();
    if (!fs.pathExists(dbFilename))
    {
        displayError("There's no '%s' database to remove collection
                from.".format(dbFilename));
        return 1;
    }

    auto adb = AutoDb(dbFilename);
    auto db = adb.db;

    auto found = db.getCollectionByName(name);
    if (found.isNull)
    {
        displayError("Error: collection '%s' has not been found.".format(name));
        return 1;
    }

    import palica.tui_helpers : promptYesNo;

    if (!ask || promptYesNo("Delete collection '%s'?".format(name)))
    {
        displayInfo("Deleting collection '%s'...".format(name));
        db.deleteCollection(found.get());
        displayInfo("Collection '%s' has been deleted.".format(name));
        return 0;
    }

    return 1;
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
    displayInfo("Syncing collection '%s' into '%s' from '%s':".format(name,
            dbFilename, path));

    if (!continueWithDupPath(db, fs, path, ask))
    {
        displayInfoSimilarCollectionPath();
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

private Nullable!DbId getDefaultFilterId(string[string] settings)
{
    auto v = settings["default_filter"];
    if (v && v.length)
    {
        import std.conv : to;

        return Nullable!DbId(to!long(v));
    }
    return Nullable!DbId();
}

int filtersDisplay(string dbFilename)
{
    auto adb = AutoDb(dbFilename);
    auto db = adb.db;
    auto settings = settingsMapFromDb(db.getSettings());
    auto filters = db.getGlobFilters();
    auto defaultFilterId = getDefaultFilterId(settings);
    if (!defaultFilterId.isNull)
    {
        import std.algorithm : find;

        auto foundDefault = find!(e => e.id ==
                defaultFilterId)(filters)[0];
        displayInfo("Default filter is '%s'.".format(foundDefault.name));
    }
    else
    {
        displayInfo("No default filter.");
    }
    auto patterns = db.getGlobPatterns();
    auto patmap = patternMapFromDb(patterns);
    foreach (f; filters)
    {
        displayInfo("%d: %s".format(f.id, f.name));
        auto fpatterns = db.getFilterPatterns(f.id);
        foreach (fp; fpatterns)
        {
            auto w = fp.include ? "include" : "exclude";
            displayInfo("%s: %s".format(w, patmap[fp.globPatternId].regexp));
        }
    }

    return 0;
}
