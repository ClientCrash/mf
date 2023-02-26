use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{ ErrorKind, Write };
use std::process::exit;

fn print_help(exec: String) {
    println!("Usage: {} <COMMAND> [FILES ...]

Commands:
    h, help             Print help information
    c  create           Create file/s
    m, merge <TARGET>   Merge files into target
    r, remove           Remove files", exec);
}

fn create(args: &Vec<String>) -> u32 {
    let mut modifications = 0;

    for arg in args {

        println!("creating file {}...", arg);

        if let Err(err) = OpenOptions::new().write(true).create_new(true).open(&arg) {
            
            if err.kind() == ErrorKind::AlreadyExists {
                println!("error: file {} exists already.", arg);
            }

        } else {
            modifications += 1;
        }
    }

    modifications
}

fn remove(args: &Vec<String>) -> u32 {
    let mut modifications = 0;

    for arg in args {

        println!("removing file {}...", arg);

        fs::remove_file(arg)
            .expect("error: could not remove file.");

        modifications += 1;
    }

    modifications
}

fn merge(args: &mut Vec<String>) -> u32 {
    let mut modifications = 0;

    println!("creating file {}...", args[0]);

    let mut target_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open(&args[0])
        .expect("error: could not create merge target");

    args.remove(0);

    for arg in args {
        println!("reading file {}...", arg);
        
        let file_contents = fs::read_to_string(arg)
            .expect("error: could not read file.");

        write!(target_file, "{}", file_contents)
            .expect("error: could not write to merge target.");

        modifications += 1;
    }

    modifications
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let exec = args[0].clone();
    args.remove(0);

    if args.len() < 1 {
        print_help(exec);
        return;
    }

    let command = args[0].clone();
    args.remove(0);

    let modifications = match command.as_str() {
        "c" | "create" => create(&args),
        "r" | "remove" => remove(&args),
        "m" | "merge" => merge(&mut args),
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

    println!("finished creating/modifying {} file/s.", modifications);
}

