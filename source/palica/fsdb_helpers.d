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
module palica.fsdb_helpers;

import palica.dblayer : DirEntry, DbId, DbReadLayer, GlobFilterToPattern,
    GlobPattern;
import palica.fslayer : FsDirEntry;
import palica.globfilter;

DirEntry dirEntryFromFsDirEntry(FsDirEntry fsEntry)
{
    import palica.helpers : sysTimeNowUtc;
    import std.path : baseName;

    return DirEntry(0, fsEntry.name.baseName(), fsEntry.modDateTime,
        sysTimeNowUtc(), fsEntry.isDir, cast(long) fsEntry.size);
}

void dumpDirEntry(ref const DirEntry e, int level = 0,
    bool verbose = true)
{
    import std.stdio : writefln;

    string indent()
    {
        string s;
        for (int i = 0; i < level; ++i)
        {
            s ~= "  ";
        }
        return s;
    }

    if (verbose)
        writefln("%s\"%s\" %sSz: %d M: %s S: %s", indent(), e.fsName,
            e.isDir ? "DIR " : "",
            e.fsSize,
            e.fsModTime,
            e.lastSyncTime);
    else
        writefln("%s\"%s\" %s%d", indent(), e.fsName,
            e.isDir ? "DIR " : "",
            e.fsSize);
}

void dumpDirEntryAsTree(DbId rootId, DbReadLayer dbRead, int level = 0,
    bool verbose = true)
{
    DirEntry[] items = dbRead.getDirEntriesOfParent(rootId);
    foreach (ref DirEntry i; items)
    {
        dumpDirEntry(i, level, verbose);
        if (i.isDir)
        {
            dumpDirEntryAsTree(i.id, dbRead, level + 1, verbose);
        }
    }
}

FsGlobFilter fsGlobFilterFromDb(GlobFilterToPattern[] fops, GlobPattern[] fpatterns)
{
    import std.algorithm : map;
    import std.array : array;

    string[] textPatterns;
    size_t[DbId] textPatternIndex;
    foreach (i, p; fpatterns)
    {
        textPatterns ~= p.regexp;
        textPatternIndex[p.id] = i;
    }

    GlobOperation[] ops = map!((fop) {
        auto opkind = fop.include ? GlobOperation.Kind.include : GlobOperation.Kind.exclude;
        auto patIndex = textPatternIndex[fop.globPatternId];

        return GlobOperation(opkind, patIndex);
    })(fops).array;

    auto gpatterns = GlobPatterns(textPatterns);

    return FsGlobFilter(gpatterns, ops);
}

unittest
{
    auto pats = [
        GlobPattern(31, "abc"), GlobPattern(333, "kor"), GlobPattern(22,
            "zz")
    ];
    auto fops = [
        GlobFilterToPattern(100, 1, 31, true, 1),
        GlobFilterToPattern(101, 1, 333, false, 2),
        GlobFilterToPattern(102, 1, 22, true, 3),
    ];

    auto filter = fsGlobFilterFromDb(fops, pats);
    import std.stdio : writeln;
    writeln("filter=", filter);
    assert(filter.accept("korzz") == true);
    assert(filter.accept("korabc") == false);
    assert(filter.accept("abc") == true);
    assert(filter.accept("kor") == false);
}
