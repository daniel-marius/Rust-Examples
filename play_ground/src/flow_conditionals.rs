pub fn run() {
    let n = 2;

    if n < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let n1 = 6;

    if n1 % 4 == 0 {
        println!("n divisible by 4", );
    } else if n % 3 == 0 {
        println!("n divisible by 3", );
    } else if n % 2 == 0 {
        println!("n divisible by 2", );
    } else {
        println!("n is not divisible by 4, 3, or 2");
    }

    let c = true;

    // Bindings
    let n2 = if c == true {
        50
    } else {
        76
    };

    println!("n2: {}", n2);

    let mut c1 = 0;
    loop {
        println!("finite");
        c1 += 1;

        if c1 < 10 {
            break;
        }
    }

    // embedded loops
    // 'a: loop {
    //     println!("loop a");
    //     'b: loop {
    //         println!("loop b");
    //         'c: loop {
    //             println!("loop c");
    //
    //             break 'b
    //
    //             if true {
    //                 continue
    //             } else {
    //                 break
    //             }
    //         }
    //     }
    //     continue 'a
    // }

    let x = loop {
        break 10;
    };

    println!("x: {}", x);

    let mut n3 = 10;

    while n3 != 0 {
        println!("{}", n3);

        n3 -= 1;
    }

    let a = vec![10, 20, 304, 50, 67];
    for i in a {
        println!("i: {}", i);
    }

    for i in 1..101 {
        println!("i: {}", i);
    }

    let x2 = 5;

    match x2 {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let x3 = 15;

    match x3 {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special")
    }

    let pair = (5, -5);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No match"),
    }

    let p = 5;
    match p {
        n @ 1  ... 12 => println!("n: {}", n),
        n @ 13 ... 19 => println!("n: {}", n),
        _ => println!("No match")
    };

    let k = match p {
        k @ 1  ... 12 => k,
        k @ 13 ... 19 => k,
        _ => 0,
    };

    println!("k: {}", k);
}
