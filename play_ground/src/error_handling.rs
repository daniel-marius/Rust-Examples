static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::Path;

fn exit(x: i32) {
    if x == 0 {
        panic!("We have 0");
    }
    println!("There is no 0");
}

fn exit2(x: Option<i32>) {
    match x {
        Some(0) => panic!("We have 0"),
        Some(x) => println!("We have {}", x),
        None => println!("We got nothing")
    }
}

fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("text.txt")?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    f.read_to_string(&mut s)?;
    Ok(s);
}

pub fn run() {
    // let v = vec![1, 2];
    //
    // v[50];

    // exit(1);
    // exit(0);

    // exit2(Some(1));
    // exit2(Some(10));
    // exit2(None);
    // exit2(Some(0));

    // // Create a path to the desired file
    // let path = Path::new("text.txt");
    // let display = path.display();
    //
    // // Open the path in read-only mode, returns `io::Result<File>`
    // let mut f = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };
    //
    // // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }
    //
    // // Open a file in write-only mode, returns `io::Result<File>`
    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };
    //
    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // match file.write_all(LOREM_IPSUM.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }

    // let f = File::open("text.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("text.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Could not create file: {:?}", e)
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!("Could not open the file: {:?}", error)
    //     },
    // };
}
