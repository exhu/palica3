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