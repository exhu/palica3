use clap;

#[derive(Debug)]
enum Command {
    Add,
}

#[derive(Debug)]
struct Cli {
    command: Command,
    db_file_name: String,
}
fn main() {
    let cmd = clap::Command::new("palica")
        .bin_name("palica")
        .subcommand_required(true)
        .subcommand(clap::Command::new("add"));

    let matches = cmd.get_matches();
    println!("{:?}", matches);
}