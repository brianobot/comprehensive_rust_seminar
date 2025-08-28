#![allow(dead_code)]

const DIGEST_SIZE: usize = 10;
const FILL_VALUE: u8 = calculate_fill_value();

// only functions marked as const can be called at compile time to generate const values
// these const functions can also be called at runtime
const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 34 } else { 43 }
}

fn main() {
    // constants are evaluated at compile time and their values are inline whethere they are used

    let value = calculate_fill_value();
    println!("Value: {value:?}");
}
