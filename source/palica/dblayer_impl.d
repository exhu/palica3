module palica.dblayer_impl;
import etc.c.sqlite3;

unittest
{
    import core.stdc.stdio : printf;
    printf("sqlite version = '%s'\n", sqlite3_libversion());
}