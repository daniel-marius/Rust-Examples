use std::ops;
use std::ops::Mul;
use std::fmt;

// Traits
trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32
}

struct Circle {
    radius: f64
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.1415 * self.radius * self.radius) as u32
    }
}

#[derive(Debug, Clone, Copy)]
struct A(i32);

// #[derive(Eq, PartialEq, PartialOrd, Ord)]
struct B(f32);

struct C;
struct D;

#[derive(Debug)]
struct CD;

#[derive(Debug)]
struct DC;

impl ops::Add<D> for C {
    type Output = CD;

    fn add(self, _rhs: D) -> CD {
        CD
    }
}

impl ops::Add<C> for D {
    type Output = DC;

    fn add(self, _rhs: C) -> DC {
        DC
    }
}

struct E {
    a: String
}

impl Drop for E {
    fn drop(&mut self) {
        println!("dropped {}", self.a);
    }
}

struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib{c: 1, n: 1}
}

// Generics
#[derive(Debug)]
struct Square<T> {
    x: T,
}

fn p<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
}

struct F<T> {
    x: T,
}

impl <T> F<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

struct G<U, V> {
    x: U,
    y: V,
}

struct H<V> {
    x: V,
    y: V,
}

trait Shape2<T> {
    fn area(&self) -> T;
}

struct Rectangle2<T: Mul> {
    x: T,
    y: T,
}

struct Circle2<T: Mul> {
    radius: T,
}

// impl <T: Copy> Shape2<T> for Rectangle2<T>
//     where T: Mul<Output = T>, {
//         fn area(&self) -> T {
//             self.x * self.y
//         }
//     }

impl <T:Mul<Output = T> + Copy> Shape2<T> for Rectangle2<T> {
    fn area(&self) -> T {
        self.x * self.y
    }
}

// Error - not working
// impl <T:Mul<Output = T> + Copy> Shape2<T> for Circle2 {
//     fn area(&self) -> T {
//         3.1415 * (self.radius * self.radius)
//     }
// }

pub fn run() {
    let c = Circle { radius: 100.132 };
    let r = Rectangle { x: 30, y: 20 };
    println!("{} {}", c.area(), r.area());

    let x = A(32);
    let y = B(12.223);
    let z = x;
    println!("{:?}", x);

    println!("{:?}", C + D);
    println!("{:?}", D + C);

    let a = E{a: String::from("A")};
    {
        let b = E{a: String::from("B")};
        {
            let c = E{a: String::from("C")};

            println!("leaving inner scope 2");
        }
        println!("leaving inner scope 1");
    }
    drop(a);
    println!("program ending");

    for j in fib().take(10) {
        println!("{}", j);
    }

    for j in fib().skip(14).take(10) {
        println!("{}", j);
    }

    let mut f = fib();

    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());

    let s = Square{x: 10};
    let s = Square{x: 1.0};
    let s = Square{x: "Hello"};
    let s = Square{x: 'c'};

    println!("{:?}", s);

    let a = F{x: "Hello"};

    a.item();

    let r2 = Rectangle2{ x: 10, y: 20 };
    let r3 = Rectangle2{ x: 10.10, y: 20.31 };

    println!("{} {}", r2.area(), r3.area());
}
