// References:
//  std::primitives: https://doc.rust-lang.org/std/index.html#primitive
//  std::collections: https://doc.rust-lang.org/std/collections/index.html

#[derive(Debug)]
struct Vector<T> {
    vector: Vec<T>,
}

impl<T> Vector<T> {
    fn push(&mut self, value: T) {
        self.vector.push(value)
    }
}

fn main() {
    let _declared_array: [i32; 3]; // Can't be used (eg printed) until initialized
    let normal_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Infers type and size
    println!("normal array: {:?}", normal_array);
    let mut vector = vec![1, 2, 3, 4, 5, 6, 7];
    // let mut vector = Vec::<i32> {1,2,3,4,5,6,7}; // Alternative instantiation
    println!("vector: {:?}", vector);
    vector.push(8);
    println!("vector: {:?}", vector);
    let mut struct_vector = Vector { vector: vec![] };
    // ^^^ Implicitly infers the type of the generic
    // let mut struct_vector = Vector::<String> { vector: vec![] };
    // ^^^ Uses turbo fish to explicitly declare the generic type 'T'
    let a: String = "complex".into();
    println!("struct vector: {:?}", struct_vector);
    struct_vector.push(a);
    println!("struct vector: {:?}", struct_vector);
    // let b = a; // Fails because a is already moved
    let mut tuple = ("1", 2, 3.5);
    println!("tuple: {:?}", tuple);
    // tuple.0 = 4; // Wrong type
    tuple.1 = 4;
    println!("tuple: {:?}", tuple);
}
