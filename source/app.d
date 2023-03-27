/+
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
+/
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
            .add(new Flag("v", "verbose", "verbose"))
            .add(new Flag("y", "yes", "do not ask confirmations"))
            .add(new Command("add")
                    .summary("add collection")
                    .add(new Argument("name", "collection name"))
                    .add(new Argument("path", "path to directory"))
                    .add(new Option(null, "filter", "glob filter id, or NULL")))
            .add(new Command("list")
                    .summary("list collections"))
            .add(new Command("tree")
                    .summary("list collection files")
                    .add(new Argument("name", "collection name")))
            .add(new Command("remove")
                    .summary("remove collection")
                    .add(new Argument("name", "collection name")))
            .add(new Command("filters")
                    .summary("display file path filters"))
            .parseArgs(args);

        parsed.on("add", (ProgramArgs args) {
            auto filterIdOption = args.option("filter");
            long filterId = 0;
            UseGlobFilter useFilter = UseGlobFilter.defaultFilter;
            if (filterIdOption !is null)
            {
                if (filterIdOption == "NULL")
                    useFilter = UseGlobFilter.none;
                else
                {
                    useFilter = UseGlobFilter.user;
                    import std.conv : to;

                    filterId = to!long(filterIdOption);
                }
            }

            result = collectionAdd(args.option("db"),
                args.arg("name"),
                args.arg("path"),
                args.flag("verbose"),
                !args.flag("yes"),
                useFilter,
                filterId);
        })
            .on("list", (args) {
                result = collectionList(args.option("db"), args.flag("verbose"));
            })
            .on("tree", (args) {
                result = collectionTree(args.option("db"),
                    args.arg("name"), args.flag("verbose"));
            })
            .on("remove", (args) {
                result = collectionRemove(args.option("db"),
                    args.arg("name"), !args.flag("yes"));
            })
            .on("filters", (args) { result = filtersDisplay(args.option("db")); });
    }
    catch (InvalidArgumentsException e)
    {
        writeln(e.msg);
        return 1;
    }
    return result;
}
