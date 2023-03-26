module palica.tui_helpers;
import std.stdio;

bool promptYesNo(string text)
{
    while (true)
    {
        writeln(text, " (y/n)");
        auto answer = stdin.readln();
        switch (answer)
        {
        case "y\n", "Y\n":
            return true;
        case "n\n", "N\n":
            return false;
        default:
            break;
        }
    }
}

struct PromptExistingCollectionsFound
{
    string path;
    string[] collections;
}

bool prompt(const PromptExistingCollectionsFound msg, bool ask)
{
    writefln("Warning! Existing collections with path '%s' found:", msg.path);
    foreach (c; msg.collections)
        writeln(c);

    if (ask)
    {
        return promptYesNo("Continue?");
    }
    return true;
}

import std.typecons : Nullable;
import std.conv : to;

struct InfoAddingCollection
{
    string name, dbFilename, path;
    Nullable!long filterId;
}

void displayInfo(const InfoAddingCollection msg)
{
    writefln("Adding collection '%s' into '%s' from '%s', filter: %s:", msg.name,
        msg.dbFilename, msg.path, msg.filterId.isNull ? "none" :
        to!string(msg.filterId.get()));
}

struct ErrorAddingCollectionExists
{
    string name;
}

void displayError(const ErrorAddingCollectionExists msg)
{
    writefln("Error! There's already a collection with name '%s'.", msg.name);
}

void displayInfo(string msg)
{
    writeln(msg);
}

void displayError(string msg)
{
    writeln(msg);
}

void displayInfoSimilarCollectionPath()
{
    displayInfo("Abort for similar collection path.");
}

version (none) unittest
{
    assert(promptYesNo("answer yes?"));
    assert(promptYesNo("answer no?") == false);
}
