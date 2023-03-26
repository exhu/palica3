/+
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
+/
module palica.fslayer_impl;

import palica.fslayer;

final class FsLayerImpl : FsReadLayer
{
    static import std.file;
    import std.datetime : SysTime;
    import std.typecons : Nullable;

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
            //import std.path : baseName;

            //auto base = baseName(d.name);
            auto lastModTime = d.timeLastModified;
            if (d.isDir)
                return Nullable!FsDirEntry(FsDirEntry.newDir(d.name, lastModTime));
            else
                return Nullable!FsDirEntry(FsDirEntry.newFile(d.name, d.size, lastModTime));
        }
        return Nullable!(FsDirEntry)();
    }

    override FsDirEntry[] dirEntries(string p)
    {
        FsDirEntry[] result;
        foreach (d; std.file.dirEntries(p, std.file.SpanMode.shallow, false))
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

    override string normalizedAbsPath(string path)
    {
        import std.path : absolutePath, buildNormalizedPath;

        return path.absolutePath().buildNormalizedPath();
    }
}

unittest
{
    import std.stdio : writeln;

    writeln("FsLayerImpl start.");
    scope (exit)
        writeln("FsLayerImpl end.");
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
    assert(any!q{a.name == "./sample-data"}(entries) == true);
    // adjust if directory contents changes
    assert(entries.length > 3);

    writeln("entries=", entries);
}
