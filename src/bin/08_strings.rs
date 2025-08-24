fn main() {
    // &str is a slice of UTF-8 encoded bytes similar to &[u8]
    // String is an owned Buffer of UTF-8 encoded bytes similar to Vec<T>

    let s1 = "Hello World!";
    println!("{s1}");

    let arr = [1, 2, 3, 4];
    let b = &arr[..];

}