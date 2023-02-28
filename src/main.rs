use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{ ErrorKind, Write };
use std::process::exit;

fn print_help(exec: String) {
    println!("Usage: {} <COMMAND> [FILES ...]

Commands:
    h, help             Print help information
    c, create           Create file/s
    m, merge <TARGET>   Merge files into target
    r, remove           Remove files", exec);
}

fn create(args: &Vec<String>) -> u32 {
    let mut modifications = 0;

    for arg in args {

        print!("creating file '{}'...\r", arg);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(arg) {
            Ok(_) => modifications += 1,
            Err(error) => match error.kind() {
                ErrorKind::AlreadyExists =>
                    println!("error: file '{}' already exists", arg),
                ErrorKind::PermissionDenied =>
                    println!("error: missing permission to create file '{}'", arg),
                _ => {},
            }
        }
    }

    modifications
}

fn remove(args: &Vec<String>) -> u32 {
    let mut modifications = 0;

    for arg in args {

        print!("removing file {}...\r", arg);

        match fs::remove_file(arg) {
            Ok(_) => modifications += 1,
            Err(error) => match error.kind() {
                ErrorKind::PermissionDenied =>
                    println!("error: missing permission to remove file '{}'", arg),
                ErrorKind::NotFound =>
                    println!("error: file '{}' does not exist", arg),
                _ => {},
            }
        }
    }

    modifications
}

fn merge(args: &[String]) -> u32 {

    if args.is_empty() {
        println!("error: no target file specified");
        return;
    }

    let mut modifications = 0;

    let mut target_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open(&args[0])
        .expect("error: could not create merge target");

    for arg in args[1..].iter() {
        print!("reading file '{}'...\r", arg);
        
        let file_contents = fs::read_to_string(arg)
            .expect("error: could not read file");

        write!(target_file, "{}", file_contents)
            .expect("error: could not write to merge target");

        modifications += 1;
    }

    modifications
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let exec = args[0].clone();
    args.remove(0);

    if args.is_empty() {
        print_help(exec);
        return;
    }

    let command = args[0].clone();
    args.remove(0);

    let modifications = match command.as_str() {
        "c" | "create" => create(&args),
        "r" | "remove" => remove(&args),
        "m" | "merge" => merge(&args[..]),
        "h" | "help" => {
            print_help(exec);
            exit(0);
        },
        _ => {
            println!("error: unknown command '{}'", command);
            print_help(exec);
            exit(1);
        },
    };

    if modifications == 0 {
        println!("no files were modified");
    } else {
        println!("modified {} files", modifications);
    }
}

