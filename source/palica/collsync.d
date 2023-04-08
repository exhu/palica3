module palica.collsync;

import palica.dblayer;
import palica.fslayer;
import std.stdio : writeln;
import palica.fsdb_helpers;
import palica.globfilter;

/*
   Two scenarios: 1) new collection is being populated,
   2) syncronizing existing collection (this module).
*/

struct CollectionSync
{
    interface CollectionListener
    {
        // just added to db
        void onNewDirEntry(ref const DirEntry dir);

        // updated db entry
        void onChangedDirEntry(ref const DirEntry dir);
    }

    this(DbReadLayer aDbRead, DbWriteLayer aDbWrite, FsReadLayer aFsRead)
    {
        dbRead = aDbRead;
        dbWrite = aDbWrite;
        fsRead = aFsRead;
    }
    /* will go to separate module
    void syncCollection(const ref Collection col)
    {
        // TODO
    }
    */

private:
    DbReadLayer dbRead;
    DbWriteLayer dbWrite;
    FsReadLayer fsRead;
}
