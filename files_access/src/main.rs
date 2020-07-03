use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let sample_file = File::open("src/hello.txt");

    let file_contents = match sample_file {
        Ok(file) => file,
        Err(error) => {
            panic!("Can't access file");
        }
    };

    let mut buf_reader = BufReader::new(file_contents);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    println!("{}", contents);
}
