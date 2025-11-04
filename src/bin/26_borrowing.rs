#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let mut vec = vec![1, 2, 3, 4];
    let elem = &vec[2]; // this takes a reference to the item at index of 2

    // vec.push(6); // since pushing can potentially reallocate the vec, this invalidates the reference above
    // dbg!(vec);
    // dbg!(elem);
}
