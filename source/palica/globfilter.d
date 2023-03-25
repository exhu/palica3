module palica.globfilter;

import std.regex;
public import std.typecons : Nullable;
import std.stdio : writeln;

struct GlobPatterns
{
    Regex!char[] regexps;

    this(string[] patterns)
    {
        // compile regex
        foreach(p; patterns)
            regexps ~= regex(p);
    }

    bool match(string text, size_t patIndex) const
    {
        auto matches = matchFirst(text, regexps[patIndex]);
        return !matches.empty;
    }
}

struct GlobOperation
{
    enum Kind
    {
        include,
        exclude
    }

    Kind operation;
    size_t patIndex;

    bool match(string text, const ref GlobPatterns allPatterns) const
    {
        return allPatterns.match(text, patIndex);
    }
}

struct FsGlobFilter
{
    GlobPatterns patterns;
    GlobOperation[] ops; 

    bool accept(string text) const
    {
        bool res = true;
        
        foreach(op; ops)
        {
            if (op.match(text, patterns))
            {
                auto p = patterns.regexps[op.patIndex];
                debug writeln("matched %s", p);
                if (op.operation == GlobOperation.Kind.exclude)
                    res = false;
                else
                    res = true;
            }
        }

        return res;
    }
}

unittest
{
    auto patterns = GlobPatterns(["middle", "^middle$", "ddl"]);
    auto ops = [GlobOperation(GlobOperation.Kind.include, 2), GlobOperation(GlobOperation.Kind.include, 0),
         GlobOperation(GlobOperation.Kind.exclude, 1)];

    auto f1 = FsGlobFilter(patterns, ops);
    assert(f1.accept("middle") == false);
    assert(f1.accept(" middle") == true);
    assert(f1.accept("123ddll") == true);
}

unittest
{
    writeln("patterns match tests");
    immutable pat = "(/|^)[^./]+[.]?$";
    assert(matchFirst("abc", pat));
    assert(matchFirst("abc.", pat));
    assert(!matchFirst("abc.jpg", pat));

    assert(matchFirst("/.kk/abc", pat));
    assert(matchFirst("/s/v/abc.", pat));
    auto a = matchFirst("/abc.jpg", pat);
    writeln("matched=", a);
    assert(!a);
}

version(none) unittest
{
    auto a = [1, 2, 3];
    auto b = a;
    a[1] = 77;
    writeln("a = ", a, " b = ", b);
    assert(b[1] == 77);
    assert(a[1] == 77);
}

