module palica.sqlhelpers;
import d2sqlite3;

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