module palica.helpers;

/*
-- (date)time values are the total hnsecs from midnight, January 1st, 1 A.D. UTC. 
-- An hnsec (hecto-nanosecond) is 100 nanoseconds. There are 10,000,000 hnsecs in a second.
-- https://dlang.org/phobos/std_datetime_systime.html#.SysTime.stdTime
*/

import std.datetime : SysTime, Duration, DateTime, UTC, Clock;

private immutable unixEpochHns = SysTime.fromUnixTime(0, UTC()).stdTime;

long unixEpochNanoseconds(SysTime st) pure nothrow
{
    return (st.stdTime - unixEpochHns) * 100;
}

SysTime sysTimeFromUnixEpochNanoseconds(long ns) pure nothrow
{
    long stdTime = ns / 100 + unixEpochHns;
    return SysTime(stdTime, UTC());
}

SysTime sysTimeNowUtc()
{
    return Clock.currTime(UTC());
}

unittest
{
    import std.stdio : writeln;

    auto unixEpoch = SysTime(DateTime(1970, 1, 1), UTC());
    auto k = unixEpochNanoseconds(unixEpoch);
    writeln("nanos =", k, SysTime.fromUnixTime(0, UTC()));
    assert(k == 0);
    auto backw = sysTimeFromUnixEpochNanoseconds(k);
    assert(backw == unixEpoch);
    writeln(backw, " == ", unixEpoch);

}
