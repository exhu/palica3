
int main(string[] args)
{
    import std.stdio : writeln;
    import commandr;
    import palica.cmdtool : run, LaunchParams, Cmd = Command;
    
    LaunchParams params;
    try
    {
        auto parsed = new Program("palica", "1.0")
            .summary("Media catalogue tool.")
            .author("Yury Benesh")
            .add(new Option(null, "db", "main database").required())
            .add(new Command("add")
                .summary("add collection")
                .add(new Argument("path", "path and filename")))
            .parseArgs(args);
        
        params.dbFsName = parsed.option("db");
        assert(params.dbFsName !is null);
        parsed.on("add", (parsedArgs) {
            params.command = Cmd.addCollection;
        });
    }
    catch(InvalidArgumentsException e)
    {
        writeln(e.msg);
        return 1;
    }
    return run(params);
}