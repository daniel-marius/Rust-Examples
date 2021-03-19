use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height,
        }
    }

    fn show(&self) {
        println!("{}x{} with area {}", self.width, self.height, self.area());
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn area(obj: Object) -> u32 {
    obj.width * obj.height
}

pub fn run() {
    let o = Object {
        width: 25,
        height: 25
    };

    let obj = Object::new(30, 30);
    obj.show();

    println!("{}x{} with area {}", o.width, o.height, o.area());
    println!("{}x{} with area {}", obj.width, obj.height, obj.area());
}
