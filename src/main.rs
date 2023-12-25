// main.rs

const HELP: &'static str = "
Usage: python-su [options] ... 

Options:
  -h, --help                Prints the help message
  -d, --dir <DIRECTORY>     Create project in the specified directory, 
                                default is the working directory
  -f, --folder <FOLDER>     Create project files in the specified folder,
                                default is the same as the name
  -F                        Use the working directory as the project folder

Arguments:
  name                      Name of the project 
"; 

#[derive(Debug)]
struct AppArgs {
    directory: Option<String>,
    folder: Option<String>,
    project_folder_is_parent: bool,
    project_name: String,
}

// skeleton of this is from the pico-args docs
fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP);
        std::process::exit(0);
    }

    let args: AppArgs = AppArgs {
        directory: match pargs.opt_value_from_str("-d")? {
            Some(t) => Some(t),
            None => pargs.opt_value_from_str("--dir")?,
        },
        folder: match pargs.opt_value_from_str("-f")? {
            Some(t) => Some(t),
            None => pargs.opt_value_from_str("--folder")?,
        },
        project_folder_is_parent: pargs.contains("-F"),
        project_name: pargs.free_from_str()?,
    };

    // remaining arguments indicate something has gone wrong
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Error: unknown/duplicate arguments {:?}.", remaining);
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    Ok(args)
}

// create the parent project folder if it needs to be created
fn create_project_folder(args: &AppArgs) {

}

// create the base files
fn create_base_project_files(args: &AppArgs) {

}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            eprintln!("{}", HELP);
            std::process::exit(1);
        }
    };

    println!("{:?}", args);

    create_project_folder(&args);
    create_base_project_files(&args);
}
