use std::process::ExitCode;

pub struct CollectionAdd {
    pub db_file_name: String,
    pub verbose: bool,
    pub yes: bool,
    pub name: String,
    pub path: String,
    pub filter_id: Option<String>,
}

pub fn collection_add(args: CollectionAdd) -> ExitCode {


    ExitCode::SUCCESS
}
