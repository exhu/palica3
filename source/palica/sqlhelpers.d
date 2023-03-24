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
module palica.sqlhelpers;
import d2sqlite3;
import palica.helpers;
import std.traits;
import std.typecons : Nullable;
import palica.dblayer : DbId;

BindPairBase bindPair(string name, long value) pure nothrow
{
    return new BindPair!long(name, value);
}

BindPairBase bindPair(string name, string value) pure nothrow
{
    return new BindPair!string(name, value);
}

BindPairBase bindPair(string name, Nullable!DbId value) pure nothrow
{
    return new BindPair!(Nullable!DbId)(name, value);
}

abstract class BindPairBase
{
    void bind(ref Statement stmt);
}

private final class BindPair(T) : BindPairBase
{
    string name;
    T value;

    this(string n, T v)
    {
        name = n;
        value = v;
    }

    override void bind(ref Statement stmt)
    {
        stmt.bind(name, value);
    }
}

void bindPairsAndExec(ref Statement stmt, BindPairBase[] pairs)
{
    foreach (p; pairs)
    {
        p.bind(stmt);
    }
    stmt.execute();
    stmt.reset();
}

T structFromRow(T)(ref Row row)
{
    import std.datetime : SysTime;

    T result;
    foreach (i, ref f; result.tupleof)
    {
        static if (is(typeof(f) == SysTime))
        {
            auto t = sysTimeFromUnixEpochNanoseconds(row.peek!(long)(i));
            f = t;
        }
        else
        {
            auto v = row.peek!(typeof(f))(i);
            f = v;
        }
    }
    return result;
}

T[] structsFromRows(T)(ref ResultRange rows)
{
    T[] result;

    foreach (ref Row r; rows)
    {
        auto e = structFromRow!T(r);
        result ~= e;
    }

    return result;
}

T[] valuesFromRows(T)(ref ResultRange rows)
{
    T[] result;

    foreach (ref Row r; rows)
    {
        auto v = r.peek!T(0);
        result ~= v;
    }

    return result;
}

T[] bindAllAndExec(T, Args...)(ref Statement stmt, Args args)
{
    stmt.bindAll(args);
    scope (exit)
        stmt.reset();
    auto rows = stmt.execute();
    static if (isAggregateType!(T))
        return structsFromRows!T(rows);
    else
        return valuesFromRows!T(rows);
}

void bindAllAndExecNoResult(Args...)(ref Statement stmt, Args args)
{
    stmt.bindAll(args);
    scope (exit)
        stmt.reset();
    stmt.execute();
}
