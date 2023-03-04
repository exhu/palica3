module palica.fslayer_impl;

import palica.fslayer;

import std.typecons : Nullable;

final class FsLayerImpl : FsReadLayer
{
    static import std.file;
    import std.datetime : SysTime;
    
    
    override bool pathExists(string p)
    {
        return std.file.exists(p);
    }

    override bool isFile(string p)
    {
        return std.file.isFile(p);
    }

    override bool isDir(string p)
    {
        return std.file.isDir(p);
    }

    override SysTime modificationDate(string p)
    {
        return std.file.timeLastModified(p);
    }
    
    private Nullable!FsDirEntry fsDirEntryFrom(ref std.file.DirEntry d)
    {
        // check for broken symlink
        if (std.file.exists(d.name) && !std.file.isSymlink(d.name))
        {
            import std.path : baseName;

            auto base = baseName(d.name);
            auto lastModTime = d.timeLastModified;
            if (d.isDir)
                return Nullable!FsDirEntry(FsDirEntry.newDir(base, lastModTime));
            else
                return Nullable!FsDirEntry(FsDirEntry.newFile(base, d.size, lastModTime));
        }
        return Nullable!(FsDirEntry)();
    }

    override FsDirEntry[] dirEntries(string p)
    {
        FsDirEntry[] result;
        foreach(d; std.file.dirEntries(p, std.file.SpanMode.shallow, false))
        {
            auto dsEntry = fsDirEntryFrom(d);
            if (!dsEntry.isNull())
            {
                result ~= dsEntry.get();
            }
        }
        return result;
    }
    
    override FsDirEntry dirEntry(string p)
    {
        auto fEntry = std.file.DirEntry(p);
        auto e = fsDirEntryFrom(fEntry);
        if (e.isNull())
        {
            throw new Exception("Cannot create FsDirEntry from '" ~ p ~ "'");
        }
        return e.get();
    }
}

unittest
{
    auto fs = new FsLayerImpl();
    auto l = fs.dirEntry("LICENSE");

    // adjust if LICENSE file is changed
    assert(l.size > 35_000);
    assert(l.size < 90_000);

    import std.datetime : SysTime, UTC, Date;

    assert(l.modDateTime > SysTime(Date(2023, 01, 01), UTC()));
    assert(l.isDir == false);
    auto sql = fs.dirEntry("sql");
    assert(sql.isDir == true);
    
    import std.algorithm.searching : any;
    auto entries = fs.dirEntries("./");
    assert(any!q{a.name == "sample-data"}(entries) == true);
    // adjust if directory contents changes
    assert(entries.length > 3);

    import std.stdio : writeln;
    writeln("entries=", entries);
}