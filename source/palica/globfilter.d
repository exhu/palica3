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
    size_t[] patIndexes;

    Nullable!size_t match(string text, const ref GlobPatterns allPatterns) const
    {
        foreach(i; patIndexes)
        {
            if (allPatterns.match(text, i))
                return Nullable!size_t(i);
        }
        return Nullable!size_t();
    }
}

struct GlobFilter
{
    GlobPatterns patterns;
    GlobOperation[] ops; 

    bool accept(string text) const
    {
        bool res = true;
        
        foreach(op; ops)
        {
            auto matched = op.match(text, patterns);
            if (!matched.isNull)
            {
                auto p = patterns.regexps[matched.get()];
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
    auto ops = [GlobOperation(GlobOperation.Kind.include, [2, 0]),
         GlobOperation(GlobOperation.Kind.exclude, [1])];

    auto f1 = GlobFilter(patterns, ops);
    assert(f1.accept("middle") == false);
    assert(f1.accept(" middle") == true);
    assert(f1.accept("123ddll") == true);
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

