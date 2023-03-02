module palica.sqlhelpers;
import d2sqlite3;

abstract class BindPairBase
{
    void bind(ref Statement stmt);
}

final class BindPair(T) : BindPairBase
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