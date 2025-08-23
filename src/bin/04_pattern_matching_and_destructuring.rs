

fn main() {
    // rust supports pattern matching and destructuring like so
    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("TUPLE: {tuple:?}");
}