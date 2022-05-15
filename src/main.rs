use std::env;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
fn main() {
    let mut modifications = 0;
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len()<1 {
        return;
    }

    let mode = args[0].clone();
    args.remove(0);


    if mode == "-c" {
            
        for arg in args {
            println!("Creating {}", arg);
            let _file = OpenOptions::new().write(true).create_new(true).open(arg);
            modifications += 1;
        }

    }
    else if mode == "-h" {
        println!("mf <mode> [file ... file ... file]");
        println!("modes:");
        println!("-h: Help | -c Create file/s | -m Merge file/s | -r Delete file/s");
        println!("!! If the mode is modify first file is target file name !!");
    }

    else if mode == "-r" {
        for arg in args {
            println!("Removing {}", arg);
            fs::remove_file(arg).expect("File removal failed.");
            modifications += 1;
        }

    }

    else if mode == "-m" {
        let target = args[0].clone();
        args.remove(0);
        let mut c= Vec::new();
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
            write!(file,"{}",content).expect("Write failed.");
        }



    }

    println!("Done. Created/Modified {} file/s.", modifications);
}
