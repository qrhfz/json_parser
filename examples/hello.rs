use std::{fs, time::SystemTime};

use json_parser::parser::parse;

fn main() {
    let start = SystemTime::now();

    let data = fs::read_to_string("small-file.json").expect("Unable to read file");

    let res = parse(&data);

    match res {
        Ok(_) => println!("ok"),
        Err(msg) => eprintln!("{}", msg),
    }

    let end = SystemTime::now();
    let dur = end.duration_since(start).expect("fail");
    println!("duration: {}", dur.as_millis());
}
