module palica.fsdb_helpers;

import palica.dblayer : DirEntry, DbId, DbReadLayer;
import palica.fslayer : FsDirEntry;

DirEntry dirEntryFromFsDirEntry(FsDirEntry fsEntry)
{
    import palica.helpers : sysTimeNowUtc;
    import std.path : baseName;

    return DirEntry(0, fsEntry.name.baseName(), fsEntry.modDateTime, sysTimeNowUtc(),
        fsEntry.isDir, cast(long) fsEntry.size);
}

void dumpDirEntryAsTree(DbId rootId, DbReadLayer dbRead, int level = 0)
{
    string indent()
    {
        string s;
        for (int i = 0; i < level; ++i)
        {
            s ~= "  ";
        }
        return s;
    }

    DirEntry[] items = dbRead.getDirEntriesOfParent(rootId);
    foreach (ref DirEntry i; items)
    {
        import std.stdio : writeln;

        writeln(indent(), i.fsName, i.isDir ? " DIR" : "", " Sz:", i.fsSize,
            " M:", i.fsModTime, " S:", i.lastSyncTime);
        if (i.isDir)
        {
            dumpDirEntryAsTree(i.id, dbRead, level + 1);
        }
    }
}
