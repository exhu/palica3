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

module palica.fslayer;

// interfaces to abstract fs access
// for easier testing

immutable struct FsDirEntry
{
    import std.datetime : SysTime;

    string name;
    ulong size;
    SysTime modDateTime;
    bool isDir;

    static FsDirEntry newFile(string aName, ulong aSize, SysTime aModTime)
    {
        return FsDirEntry(aName, aSize, aModTime, false);
    }

    static FsDirEntry newDir(string aName, SysTime aModTime)
    {
        return FsDirEntry(aName, 0, aModTime, true);
    }

    private this(string aName, ulong aSize, SysTime aModTime, bool aIsDir)
    {
        name = aName;
        size = aSize;
        modDateTime = aModTime;
        isDir = aIsDir;
    }

}

interface FsReadLayer
{
    import std.datetime : SysTime;

    bool pathExists(string p);
    bool isFile(string p);
    bool isDir(string p);
    SysTime modificationDate(string p);
    /// not recursive, TODO replace with Range?
    FsDirEntry[] dirEntries(string p);
    FsDirEntry dirEntry(string p);
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
