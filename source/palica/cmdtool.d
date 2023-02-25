module palica.cmdtool;

public enum Command
{
    unknown,
    newDb,
    listCollections,
    addCollection,
}

public struct LaunchParams
{
    Command command;
    string dbFsName;
}

public int run(LaunchParams params)
{
    alias CmdFunc = int function(LaunchParams p);
    immutable CmdFunc[Command] commands = [
        Command.newDb: &newDb,
    ];

    return commands[params.command](params);
}

private:

int newDb(LaunchParams params)
{
    import std.stdio : writeln;
    writeln("new db!!!");
    return 0;
}