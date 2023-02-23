module palica.dblayer;

import std.typecons;

alias DbId = ulong;

interface DbReadLayer
{
    import std.typecons.nullable;
    Nullable!DbId getCollection(string name);
    DbId[] enumCollections();
    // TODO collection interface?
}

interface DbWriteLayer
{
    bool createCollection(string name, string srcPath);
    // TODO
    
}