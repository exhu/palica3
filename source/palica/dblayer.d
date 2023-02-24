module palica.dblayer;

alias DbId = ulong;

interface DbReadLayer
{
    import std.typecons : Nullable;
    Nullable!DbId getCollection(string name);
    DbId[] enumCollections();
    // TODO collection interface?
}

interface DbWriteLayer
{
    bool createCollection(string name, string srcPath);
    // TODO
    
}