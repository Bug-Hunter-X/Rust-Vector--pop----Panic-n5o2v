fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before: {:?}", vec);

    // Safe way to handle pop():
    if let Some(value) = vec.pop() {
        println!("Popped value: {}", value);
    } else {
        println!("Vector is empty, nothing to pop");
    }

    if let Some(value) = vec.pop() {
        println!("Popped value: {}", value);
    } else {
        println!("Vector is empty, nothing to pop");
    }

    println!("After: {:?}", vec);
} 