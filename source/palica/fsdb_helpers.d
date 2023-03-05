module palica.fsdb_helpers;

import palica.dblayer : DirEntry;
import palica.fslayer : FsDirEntry;

DirEntry dirEntryFromFsDirEntry(FsDirEntry fsEntry)
{
    import palica.helpers : sysTimeNowUtc;
    import std.path : baseName;

    return DirEntry(0, fsEntry.name.baseName(), fsEntry.modDateTime, sysTimeNowUtc(),
        fsEntry.isDir);
}
