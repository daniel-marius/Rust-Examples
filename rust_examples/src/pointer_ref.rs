pub fn run() {
    // Primitve Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // Vector non-primitive
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}
