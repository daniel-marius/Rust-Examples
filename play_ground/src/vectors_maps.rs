use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub fn run() {
    let x = vec![1, 2, 3, 4];
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }

    v.push(10);

    println!("{:?} {} {}", &v, v.len(), v.capacity());
    println!("{:?}", v.pop());

    let r = vec![
        Example::Int(142),
        Example::Float(12.32),
        Example::Text(String::from("string")),
    ];

    println!("{:?}", &r);

    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 59);
    hm.remove(&String::from("strings"));

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("No match"),
    }

    let s = Some('c');

    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }

    if let Some(i) = s {
        println!("{}", i);
    }

    let mut s2 = Some(0);

    // loop {
    //     match s2 {
    //         Some(i) => if i > 19 {
    //             println!("Quit");
    //             s2 = None;
    //         } else {
    //             println!("{}", i);
    //             s2 = Some(i + 2);
    //         },
    //         _ => {
    //             break;
    //         }
    //     }
    // }

    while let Some(i) = s2 {
        if i > 19 {
            println!("Quit");
            s2 = None;
        } else {
            println!("{}", i);
            s2 = Some(i + 2);
        }
    }

    // Casting
    let f = 24.4565_f32;
    let i = f as u8;
    let c = i as char;

    println!("{} {} {}", f, i, c);
    println!("{}", 255 as char);
    println!("{} {}", 12 as char, 14 as char);

    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem: {:?}", error)
        },
    };
}
