// static variables will live for the entire duration of the program and therefore are not moved
// these are not inlined upon use, but actually have a memory address , when a globally
// access variable does not have the need for an object identity, prefer a const

static BANNER: &str = "Hello from Brian's Lesson of Comprehensive Rust";

fn main() {
    println!("{BANNER}");
}
