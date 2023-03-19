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

version(none) unittest
{
    assert(promptYesNo("answer yes?"));
    assert(promptYesNo("answer no?") == false);
}
