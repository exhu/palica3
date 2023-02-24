module palica.fslayer;

// TODO interfaces to abstract fs access
// for easier testing

class FsDirEntry
{
    string name;
    ulong size;
    // TODO date type
    ulong modDateTime;
}

interface FsReadLayer
{
    import std.range.interfaces : InputRange;

    bool pathExists(string p);
    bool isFile(string p);
    bool isDir(string p);
    bool isSymlink(string p);
    // TODO proper date-time type
    ulong modificationDate(string p);
    InputRange!FsDirEntry dirEntries(string p);
}

unittest
{
    import std.stdio : writeln;
    
    writeln("hello test");
    assert(false);
}

interface FsWriteLayer
{
    bool makeDir(string p);
    bool makeSymlink(string p, string src);
    bool removeFile(string p);
    bool removeDir(string p);
    bool renameFile(string p, string newName);
    bool renameDir(string p, string newName);
}

interface FileMetaProvider
{
    // sha256
    string calcHash(string p);
    string mimeType(string p);
}

interface XmpMetaProvider
{
    string[] readSubjectTags();
}