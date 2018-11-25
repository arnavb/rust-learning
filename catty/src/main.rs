#[macro_use] extern crate clap;

use std::{io, error, env, fs::read_to_string, path::PathBuf, process};

fn main() {
    process::exit(
        if let Err(err) = cli(env::args().collect::<Vec<_>>()) {
            // CLI parsing errors
            if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
                eprint!("{}", clap_err);
            } else {
                eprintln!("{}", err);
            }
            1
        } else {
            0
        }
    );
}

fn cli(cli_args: Vec<String>) -> Result<(), Box<error::Error>> {
    let matches = clap::App::new("catty")
        .version(crate_version!())
        .about("A minimal clone of the linux utility cat.")
        .arg(clap::Arg::with_name("FILE")
            .help("The file to concatenate to standard output")
            .required(true))
        .arg(clap::Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Numbers all output lines"))
        .get_matches_from_safe(cli_args)?;

    let file_contents = get_file_contents(matches.value_of("FILE").unwrap())?;
    let file_contents: Vec<&str> = file_contents.split("\n").collect();
    
    let number_lines = matches.is_present("number");
    
    for (i, line) in file_contents.iter().enumerate() {
        let formatted_line = if number_lines {
            format!("{:>6}  {}", i + 1, line)
        } else {
            line.to_string()
        };

        if i == file_contents.len() - 1 && line.len() > 0 {
            print!("{}", formatted_line);
        } else if !(i == file_contents.len() - 1 && line.len() == 0) {
            println!("{}", formatted_line);
        }
    }
    
    Ok(())
}

fn get_file_contents(passed_argument: &str) -> Result<String, Box<error::Error>> {
    let mut resolved_path = PathBuf::from(passed_argument);
    
    if !resolved_path.exists() || !resolved_path.is_file() {
        resolved_path = PathBuf::from(env::current_dir()?);
        resolved_path.push(passed_argument);
        
        if !resolved_path.exists() || !resolved_path.is_file() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "The passed file is either not a file or does not exist!").into());
        }
    }

    Ok(read_to_string(resolved_path)?)
}