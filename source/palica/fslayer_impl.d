module palica.fslayer_impl;

import palica.fslayer;

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

    override FsDirEntry[] dirEntries(string p)
    {
        import std.path : baseName;

        FsDirEntry[] result;
        foreach(d; std.file.dirEntries(p, std.file.SpanMode.shallow, false))
        {
            // check for broken symlink
            if (std.file.exists(d.name) && !std.file.isSymlink(d.name))
            {
                auto base = baseName(d.name);
                auto lastModTime = d.timeLastModified;
                if (d.isDir)
                    result ~= FsDirEntry.newDir(base, lastModTime);
                else
                    result ~= FsDirEntry.newFile(base, d.size, lastModTime);
            }
        }
        return result;
    }
}

