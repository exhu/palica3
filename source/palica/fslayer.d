module palica.fslayer;

// TODO interfaces to abstract fs access
// for easier testing
interface FsReadLayer
{
    bool pathExists(string p);
    bool isFile(string p);
    bool isDir(string p);
    bool isSymlink(string p);
    // TODO proper date-time type
    ulong modificationDate(string p);
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