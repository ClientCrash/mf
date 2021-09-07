use std::env;
fn main() {
    let mut numbercreated = 0;
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {
        println!("Creating {}", arg);
        //TODO: CREATE FILE
        numbercreated+=1;
    }
    println!("Done. Created {} files.",numbercreated);
}
