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

// TODO interfaces to abstract fs access
// for easier testing

class FsDirEntry
{
    import std.datetime : SysTime;
    
    string name;
    ulong size;
    SysTime modDateTime;
}

interface FsReadLayer
{
    import std.range.interfaces : InputRange;
    import std.datetime : SysTime;

    bool pathExists(string p);
    bool isFile(string p);
    bool isDir(string p);
    bool isSymlink(string p);
    SysTime modificationDate(string p);
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