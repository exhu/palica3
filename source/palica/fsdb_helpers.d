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

import palica.dblayer : DirEntry, DbId, DbReadLayer;
import palica.fslayer : FsDirEntry;

DirEntry dirEntryFromFsDirEntry(FsDirEntry fsEntry)
{
    import palica.helpers : sysTimeNowUtc;
    import std.path : baseName;

    return DirEntry(0, fsEntry.name.baseName(), fsEntry.modDateTime,
        sysTimeNowUtc(), fsEntry.isDir, cast(long) fsEntry.size);
}

void dumpDirEntry(ref const DirEntry e, int level = 0)
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
    
    writefln("%s\"%s\" %sSz: %d M: %s S: %s", indent(), e.fsName,
        e.isDir ? "DIR " : "",
        e.fsSize,
        e.fsModTime,
        e.lastSyncTime);
}

void dumpDirEntryAsTree(DbId rootId, DbReadLayer dbRead, int level = 0)
{
    DirEntry[] items = dbRead.getDirEntriesOfParent(rootId);
    foreach (ref DirEntry i; items)
    {
        dumpDirEntry(i, level);
        if (i.isDir)
        {
            dumpDirEntryAsTree(i.id, dbRead, level + 1);
        }
    }
}
