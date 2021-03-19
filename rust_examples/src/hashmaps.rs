use std::collections::HashMap;

pub fn run() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming1", 96);
    marks.insert("Rust Programming2", 97);
    marks.insert("Rust Programming3", 98);
    marks.insert("Rust Programming4", 99);

    println!("Subjects: {}", marks.len());

    // Get a single value
    match marks.get("Rust Programming1") {
        Some(mark) => println!("{}", mark),
        None => println!("No subject found")
    }

    // Remoe a value
    marks.remove("Rust Programming1");

    // Loop through HashMap
    for (subject, mark) in &marks {
        println!("For {} you got {}%", subject, mark);
    }

    // Check for value
    println!("{}", marks.contains_key("C++"));
}
