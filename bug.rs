fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before: {:?}", vec);

    // This line causes a panic if the vector is empty
    vec.pop();
    vec.pop();

    println!("After: {:?}", vec);
}