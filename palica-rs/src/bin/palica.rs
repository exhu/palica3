use clap;

#[derive(Debug, Default)]
enum Command {
    #[default]
    Add,
}

#[derive(Debug, Default)]
struct Cli {
    pub command: Command,
    pub db_file_name: String,
}

fn main() {
    let mut parsed = Cli::default();

    let cmd = clap::Command::new("palica")
        .version(::palica::PALICA_VERSION)
        .bin_name("palica")
        .arg(clap::Arg::new("database").long("db").required(true).help("database file").required(true))
        .subcommand(clap::Command::new("add"));

    let matches = cmd.get_matches();
    println!("{:?}", matches);
}
