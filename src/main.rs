// main.rs

#![feature(io_error_more)]

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};

const HELP: &'static str = "
Usage: python-su [options] name 

Options:
  -h, --help                Prints this help message
  -d, --dir <DIRECTORY>     Create project in the specified directory, 
                                by default this is the working directory
  -f, --folder <FOLDER>     Create project files in the specified folder,
                                by default this is the project name
  -F                        Use the directory as the project folder
                                this should not be used with the -f flag

Arguments:
  name                      Name of the project"; 

#[derive(Debug)]
struct AppArgs {
    directory: Option<String>,
    folder: Option<String>,
    project_folder_is_directory: bool,
    project_name: String,
}

// skeleton of this is from the pico-args docs
fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP);
        std::process::exit(0);
    }

    // create an AppArgs struct from the arguments, erroring if not possible
    let args: AppArgs = AppArgs {
        directory: match pargs.opt_value_from_str("-d")? {
            Some(t) => Some(t),
            None => pargs.opt_value_from_str("--dir")?,
        },
        folder: match pargs.opt_value_from_str("-f")? {
            Some(t) => Some(t),
            None => pargs.opt_value_from_str("--folder")?,
        },
        project_folder_is_directory: pargs.contains("-F"),
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

// create and return the pathbuf to the project folder
fn create_project_folder(args: &AppArgs) -> Result<PathBuf, IoError> {
    // get the working directory
    let cwd = std::env::current_dir()?;
    
    // convert the directory into a pathbuf struct
    let mut dir: PathBuf = match &args.directory {
        Some(d) => {
            let path = Path::new(&d);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                let mut p = cwd.to_path_buf();
                p.push(path);
                p
            }
        },
        None => std::env::current_dir()?,
    };

    // create the folder if it doesn't already exist
    if !dir.try_exists()? {
        std::fs::create_dir_all(dir.clone())?;
    } else if dir.is_file() {
        // if the directory exists but is a file, error
        return Err(IoError::new(IoErrorKind::NotADirectory, "Specified directory is a file"));
    }

    // need to create another folder if the project folder is not the directory
    if !args.project_folder_is_directory {
        // append the project folder to the directory
        dir.push(
            match &args.folder {
                Some(f) => Path::new(f),
                None => Path::new(&args.project_name),
            } 
        );

        // create the folder, error if it already exists or is a file
        if !dir.try_exists()? {
            // create the new directories
            std::fs::create_dir_all(dir.clone())?;
        } else if dir.is_file() {
            // if the folder exists but is a file, error
            return Err(IoError::new(IoErrorKind::NotADirectory, "Specified folder is a file"));
        } else {
            // if the folder already exists, still error
            return Err(IoError::new(IoErrorKind::AlreadyExists, "Specified folder already exists"));
        }
    }
    
    Ok(dir)
}

// create the base files
fn create_base_project_files(args: &AppArgs, path: &PathBuf) -> Result<(), IoError> {
    let mut py_path = path.clone();
    py_path.push(&std::format!("{}.py", &args.project_name));
    let mut py_file = File::create_new(py_path)?;
    write!(py_file, "# {}.py\n", &args.project_name)?;

    let mut bat_path = path.clone();
    bat_path.push(&std::format!("{}.bat", &args.project_name));
    let mut bat_file = File::create_new(bat_path)?;
    write!(bat_file, "python {}.py\npause", &args.project_name)?;

    Ok(())
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

    // println!("{:?}", args);

    // create the project directory and folder
    let path = match create_project_folder(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}.", e);
            eprintln!("{}", HELP);
            std::process::exit(1);
        } 
    };

    // generate project files
    if let Err(e) = create_base_project_files(&args, &path) {
        eprintln!("Error: {}.", e);
        eprintln!("{}", HELP);
        std::process::exit(1);
    }
}
