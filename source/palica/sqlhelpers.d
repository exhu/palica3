module palica.sqlhelpers;
import d2sqlite3;
import palica.helpers;

BindPairBase bindPair(string name, long value) pure nothrow
{
    return new BindPair!long(name, value);
}

BindPairBase bindPair(string name, string value) pure nothrow
{
    return new BindPair!string(name, value);
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

void bindPairsAndExec(ref Statement stmt,  BindPairBase[] pairs)
{
    foreach(p; pairs)
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
    foreach(i, ref f; result.tupleof)
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

T[] bindAllAndExec(T, Args...)(ref Statement stmt, Args args)
{
    stmt.bindAll(args);
    scope(exit) stmt.reset();
    auto rows = stmt.execute();
    return structsFromRows!T(rows);
}