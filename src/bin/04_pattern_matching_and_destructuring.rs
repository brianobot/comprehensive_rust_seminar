

fn main() {
    // rust supports pattern matching and destructuring like so
    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("TUPLE: {tuple:?}");

    // irrefutable patterns are patterns that rust compiler can statically proof that the lhs matches the rhs in terms of their structure
    // interestingly, a variable name is an irrefutable pattern that match any any value
    // that's why we can assigned any structure to a variable name with let
}