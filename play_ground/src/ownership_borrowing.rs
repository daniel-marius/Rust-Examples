fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100]);
}

fn copy(a: i32, b: i32) {
    println!("{}", a + b);
}

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]);
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[11]);
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}

pub fn run() {
    let x = 1;
    let y = x;
    let a = 10;
    {
        // let a = 10;
    }

    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    // take(v);

    let a = 32;
    let b = 45;

    copy(a, b);

    println!("we have a: {} and b: {}", a, b);

    v = re(v);

    borrow1(&v);
    borrow2(&v);

    let v1 = vec![4, 5, 6, 7, 7, 2, 3, 5, 6, 89, 8, 7, 7];
    for &i in &v1 {
        let r = count(&v1, i);
        println!("{} is repeated {} times", i, r);
    }
}
