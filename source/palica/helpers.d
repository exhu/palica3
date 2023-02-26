module palica.helpers;

/*
-- (date)time values are the total hnsecs from midnight, January 1st, 1 A.D. UTC. 
-- An hnsec (hecto-nanosecond) is 100 nanoseconds. There are 10,000,000 hnsecs in a second.
-- https://dlang.org/phobos/std_datetime_systime.html#.SysTime.stdTime
*/

import std.datetime : SysTime, Duration, DateTime, UTC;

long unixEpochNanoseconds(SysTime st)
{
    return (st.stdTime - SysTime.fromUnixTime(0, UTC()).stdTime)*100;
}

unittest
{
    import std.stdio : writeln;
    auto k = unixEpochNanoseconds(SysTime(DateTime(1970,1,1), UTC()));
    writeln("nanos =", k, SysTime.fromUnixTime(0, UTC()));
    assert(k == 0);
}