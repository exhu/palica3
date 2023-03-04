module palica.collbuilder;

import palica.dblayer;
import palica.fslayer;

interface ScanningEvents
{
    // just added to db
    void onNewDirEntry(ref const DirEntry dir);
    
    // updated db entry
    void onChangedDirEntry(ref const DirEntry dir);
}

/*
   Two scenarios: 1) new collection is being populated,
   2) syncronizing existing collection
*/

struct CollBuilder
{
    this(DbReadLayer aDbRead, DbWriteLayer aDbWrite, FsReadLayer aFsRead)
    {
        dbRead = aDbRead;
        dbWrite = aDbWrite;
        fsRead = aFsRead;
    }
    
    Collection createCollection(string name, string path)
    {
        if (!fsRead.pathExists(path))
        {
            throw new Exception("Cannot create collection '" ~
                name ~ "' -- cannot read path '" ~ path ~ "'");
        }
        
        // TODO
        return Collection();
    }
    
    void syncCollection(const ref Collection col)
    {
        // TODO
    }

private:
    DbReadLayer dbRead;
    DbWriteLayer dbWrite;
    FsReadLayer fsRead;
}