module palica.dblayer_impl;
import etc.c.sqlite3;

unittest
{
    import core.stdc.stdio : printf;
    printf("sqlite version = '%s'\n", sqlite3_libversion());

    import std.stdio;

    sqlite3* pdb;
    int rc = sqlite3_open("testdb.db", &pdb);
    if (rc)
    {
        writeln("failed to open db");
    }
    sqlite3_close(pdb);
}