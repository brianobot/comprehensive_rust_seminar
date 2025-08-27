
fn main() {
    // arrays can be initialized with shorthand syntax [<initil_value_to_be_repeated>; <number of time to repeat>]
    let zeros = [0; 100];
    println!("Zeros: {:?}", zeros);

    // the type annotation of an array includes the number of elements
    let _ones: [i32; 5] = [1, 1, 1, 1, 1]; // [T; N]
    // because of this, two arrays with different N and the same T are two different types

    let mut _a = [1, 2, 3, 3, 4];
    // a[6] = 0; // this would cause a compilation error, because a value is assigned outside the bounds of the array

    // println!("a: {:?}", a);
    // arrays are not heap allocated, their size is known and they go on the stack
}