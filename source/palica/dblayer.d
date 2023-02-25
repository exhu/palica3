module palica.dblayer;

/// sqlite INTEGER is signed (up to 8 bytes, i.e. 64-bit max)
alias DbId = long;

interface DbReadLayer
{
    import std.typecons : Nullable;
    Nullable!DbId getCollection(string name);
    DbId[] enumCollections();
    // TODO collection interface?
}

// On an INSERT, if the ROWID or INTEGER PRIMARY KEY column is not explicitly
// given a value, then it will be filled automatically with an unused integer,
// usually one more than the largest ROWID currently in use. This is true
// regardless of whether or not the AUTOINCREMENT keyword is used. 
// https://www.sqlite.org/autoinc.html

interface DbWriteLayer
{
    // errors
    final class CollectionAlreadyExists : Exception
    {
        this(string name, DbId dbId)
        {
            import std.string : format;
            super(format("Collection '%s' with id '%d' already exists.", name, dbId));
        }
    }

    /// Throws CollectionAlreadyExists
    DbId createCollection(string name, string srcPath);
    // TODO
    
}