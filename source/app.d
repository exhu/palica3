int main(string[] args)
{
    import std.stdio : writeln;
    import commandr;
    import palica.cmdtool;
    
    int result = 1;

    try
    {
        ProgramArgs parsed = new Program("palica", "1.0")
            .summary("Media catalogue tool.")
            .author("Yury Benesh")
            .add(new Option(null, "db", "main database").required())
            .add(new Command("add")
                    .summary("add collection")
                    .add(new Argument("name", "collection name"))
                    .add(new Argument("path", "path to directory")))
            .add(new Command("list")
                    .summary("list collections"))
            .add(new Command("tree")
                    .summary("list collection files")
                    .add(new Argument("name", "collection name")))
            .parseArgs(args);

        parsed.on("add", (parsedArgs) {
                result = collectionAdd(parsedArgs.option("db"),
                    parsedArgs.arg("name"),
                    parsedArgs.arg("path"));
            })
            .on("list", (parsedArgs) {
                    result = collectionList(parsedArgs.option("db"));
            })
            .on("tree", (parsedArgs) {
                    result = collectionTree(parsedArgs.option("db"),
                    parsedArgs.arg("name"));
            });
    }
    catch (InvalidArgumentsException e)
    {
        writeln(e.msg);
        return 1;
    }
    return result;
}
