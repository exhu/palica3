module palica.dbhelpers;

import palica.dblayer;

string[string] settingsMapFromDb(SettingValue[] values)
{
    string[string] res;
    foreach(v; values)
    {
        res[v.key] = v.value;
    }
    return res;
}

unittest
{
    auto values = [ SettingValue(0, "k1", "v1"),
        SettingValue(1, "k2", "v2"),
        SettingValue(2, "k3", "v3"),
    ];

    auto m = settingsMapFromDb(values);
    assert(m["k1"] == "v1");
    assert(m["k2"] == "v2");
    assert(m["k3"] == "v3");
}

GlobPattern[DbId] patternMapFromDb(GlobPattern[] patterns)
{
    GlobPattern[DbId] res;
    foreach(p; patterns)
        res[p.id] = p;

    return res;
}
