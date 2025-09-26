#[allow(unused)]

fn main() {
    let tuple = (12, 34, 45);
    let (a, b, c) = tuple;
    let (_, d, e) = tuple; // ignores the first element
    let (.., last) = tuple; // selects only the last elements
    let (first, .., last) = tuple; // ignores a range

    // all these match expressions works with array too
    let array = [1, 2, 3];
    let [first, .., last] = array;
}
