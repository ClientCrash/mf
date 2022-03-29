use std::env;
use std::fs::OpenOptions;
use std::io::ErrorKind;

fn main() {
    let mut numbercreated = 0;
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        println!("Creating {}", arg);
        if let Err(error) = OpenOptions::new().write(true).create_new(true).open(&arg) {
            if error.kind() == ErrorKind::AlreadyExists {
                println!("File {} already exists.", arg);
            }
        } else {
            numbercreated += 1;
        }
    }

    println!("Done. Created {} file/s.", numbercreated);
}
