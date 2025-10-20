fn main() {
    // types that implement the Copy traits are copied by default instead of bring moved

    let x = 32;
    let y = x;

    dbg!(x);
    dbg!(y);

    // copying does not work on types that do not implement the Drop trait
    // shared references are Copy/Clone but mutable references are not
}
