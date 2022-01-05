use std::env;
use std::fs::OpenOptions;
fn main() {
    let mut numbercreated = 0;
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {
        println!("Creating {}", arg);
        let _file = OpenOptions::new().write(true).create_new(true).open(arg);
        numbercreated += 1;
    }
    println!("Done. Created {} file/s.", numbercreated);
}
