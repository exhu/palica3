import std.stdio : writeln, writefln;


int main(string[] args)
{
    import std.getopt;
    import palica.cmdtool : run, LaunchParams, Command;

    immutable Command[string] commandFromName = [
        "new": Command.newDb,
    ];

    string commandsInfo()
    {
        return "Commands:
        new -- create new db
        ";
    }

    LaunchParams params;
    try
    {
        auto helpInformation = getopt(
            args,
            std.getopt.config.required,
            "db", "database filename", &params.dbFsName);
        if (args.length >= 2) params.command = commandFromName[args[1]];
        if (helpInformation.helpWanted || params.command == Command.unknown)
        {
            defaultGetoptPrinter("Palica media catalogue tool.\nUsage: palica <command> [options]\n" ~
                commandsInfo,
                helpInformation.options);
            return 1;
        }
    }
    catch (GetOptException e)
    {
        writeln(e.msg);
        return 1;
    }

    return run(params);
}
