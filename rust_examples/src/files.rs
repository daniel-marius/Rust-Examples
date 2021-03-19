use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn run() {
    let path = Path::new("./info.txt");
    // let mut file = File::open(&path).expect("Can't open file!");
    //
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("Can't read the file!");
    //
    // println!("File contents: {}", contents);

    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

}
