extern crate reqwest;
use std::collections::HashMap;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:?}", resp);
    Ok(())

    // match reqwest::get("https://www.rust-lang.org") {
    //     Ok(mut response) => {
    //         // Check if 200 OK
    //         if response.status() == reqwest::StatusCode::OK {
    //             match response.text() {
    //                 Ok(text) => println!("Response text: {}", text),
    //                 Err(_) => println!("Could not read response text!"),
    //             }
    //         } else {
    //             println!("Response was not 200 OK");
    //         }
    //     }
    //     Err(_) => println!("Could not make the request!"),
    // }
}
