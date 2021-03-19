// use std::mem;

// mod ownership_borrowing;
// mod functions_structs;
// mod flow_conditionals;
// mod enums_options;
// mod vectors_maps;
// mod snake;
// mod traits_generic;
// mod pointers_iterators;
// mod macro_metaprog;
// mod error_handling;
// mod concurrency;
// mod port_sniffer;

#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    // let mut x = 5;
    // let x2: u32 = 5;
    //
    // let a = 1 + 20;
    //
    // let s = 30 - 20;
    // let m = 5 * 10;
    // let d = 4 / 6;
    // let r = 49 % 2;
    //
    // let c: char = 'z';
    //
    // let t: (i32, f64, char) = (42, 6.12, 'j');
    // let (_, _, x) = t;
    // t.0, t.1, t.2

    // let a = [1, 2, 3, 4, 7, 8, 9];
    // let a1 = a[0];

    // let t = (1, 'a', false);
    // let f = (2, (1, 'a', false));
    // println!("{} {} {}", t.0, t.1, t.2);
    // println!("{:#?}", f);
    //
    // let xs: [i32; 5] = [4, 5, 6, 7, 8];
    // println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs)); // size in bytes in memory
    //
    // let ys = &xs[2..4];
    // println!("{:?}", ys);
    //
    // let s = "String".to_string();
    // let ss = String:: from("String!");
    // println!("{}", s);
    // println!("{}", ss);
    //
    // let slice = &ss[0..4];
    // println!("{}", slice);
    //
    // let h = String::from("Hello, ");
    // let w = String::from("World!");
    // // (self, &String)
    // let con = h + &w;
    // println!("{}", con);
    //
    // ownership_borrowing::run();
    // functions_structs::run();
    // flow_conditionals::run();
    // enums_options::run();
    // vectors_maps::run();
    // snake::run();
    // traits_generic::run();
    // pointers_iterators::run();
    // macro_metaprog::run();
    // error_handling::run();
    // concurrency::run();
    // port_sniffer::run();


    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 =>
            {
                println!("exiting!");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(),
                                        receiver.trim().to_string(),
                                        amount.trim().parse().unwrap());

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            },
            2 =>
            {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            },
            3 =>
            {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Update Difficulty"),
                }
            },
            4 =>{
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            _ => println!("Invalid option please retry"),
        }

    }
}
