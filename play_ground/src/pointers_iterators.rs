#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

// fn f(i: i32) -> i32 { i + 1 }

fn rrun<F>(f: F) where F: Fn() {
    // f()
    return f();
}

fn add3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    // f(3)
    return f(3);
}

struct A<F: Fn(i32) -> i32> {
    f: F
}

fn pr() {
    println!("This is a normal function");
}

fn create() -> Box<dyn Fn()> {
    Box::new(move || println!("This is a closure in a box"))
}

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

pub fn run() {
    let b = Box::new(10);
    println!("b = {}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("{:?}", l);

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("True");
    }

    // Closure, anonymous functions
    let f = |i| i + 1;
    let k = 10;
    println!("{}", f(k));

    let p = || println!("This is a closure");
    // p();
    rrun(p);
    rrun(pr);

    let mut c = 0;

    let mut inc = || {
        c += 1;
        println!("Incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();

    let a = |i| i * 10;

    println!("3 * 10 = {}", add3(a));

    // let a2 = A { f: a };

    let x2 = create();
    x2();

    let v = vec![1, 2, 3];

    let j = v.iter().next();
    println!("{:?}", j);
    println!("v {}", v.iter().any(|&x| x != 2));

    // First variant
    let top = 10000;
    let mut c = 0;

    for n in 0.. {
        let x = n * n;

        if x >= top {
            break;
        } else if is_even(x) {
            c += x;
        }
    }

    println!("{}", c);

    // Second variant - higher order functions, functional programming
    let s: u32 =
    (0..).map(|n| n*n)
    .take_while(|&n| n < 10000)
    .filter(|&n| is_even(n))
    .fold(0, |s, i| s + i); // sum all the values together

    println!("{}", s);
}
