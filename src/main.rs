use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{ ErrorKind, Write };
use std::process::exit;

fn print_help(exec: String) {
    println!("Usage: {} <MODE> [FILES ...]", exec);

    println!("Modes:");
    println!("\t-h, --help  \tPrint help information");
    println!("\t-c, --create\tCreate file/s");
    println!("\t-m, --merge \tMerge files");
    println!("\t-r, --remove\tRemove file/s");

    println!("!! If the mode is modify first file is target file name !!");
    println!("https://github.com/clientcrash/mf");
}

fn create(args: &Vec<String>) -> u32 {
    let mut modifications = 0;

    for arg in args {
        println!("Creating {}", arg);
        if let Err(error) = OpenOptions::new().write(true).create_new(true).open(&arg) {
            if error.kind() == ErrorKind::AlreadyExists {
                println!("File {} already exists.", arg);
            }
        } else {
            modifications += 1;
        }
    }

    return modifications;
}

fn remove(args: &Vec<String>) -> u32 {
    let mut modifications = 0;

    for arg in args {
        println!("Removing {}", arg);
        fs::remove_file(arg).expect("File removal failed.");
        modifications += 1;
    }

    return modifications;
}

fn merge(args: &mut Vec<String>) -> u32 {
    let mut modifications = 0;

    let target = args[0].clone();
    args.remove(0);

    let mut c = Vec::new();
    for arg in args {
        println!("Reading {}", arg);
        let fc = fs::read_to_string(arg).expect("Read failed.");
        c.push(fc);
    }

    println!("Writing {}", target);
    modifications += 1;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open(target)
        .unwrap();
    for content in c {
        write!(file, "{}", content).expect("Write failed.");
    }

    return modifications;
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let exec = args[0].clone();
    args.remove(0);

    if args.len() < 1 {
        print_help(exec);
        return;
    }

    let mode = args[0].clone();
    args.remove(0);

    let modifications = match mode.as_str() {
        "-c" | "--create" => create(&args),
        "-r" | "--remove" => remove(&args),
        "-m" | "--merge" => merge(&mut args),
        "-h" | "--help" => {
            print_help(exec);
            exit(0);
        },
        _ => {
            println!("Unknown mode '{}'", mode);
            print_help(exec);
            exit(1);
        },
    };

    println!("Done. Created/Modified {} file/s.", modifications);
}

