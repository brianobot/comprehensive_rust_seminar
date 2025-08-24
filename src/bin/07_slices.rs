fn main() {
    // a slice gives you a view into a larger collection
    let a = [1, 3, 52, 123, 34, 12, 5234, 21, 42];

    let s = &a[2..4];
    println!("{s:?}");
    // slices borrow data from the sliced type
    // we create a slice by borrowing a collection and specifying the start and end index to be sliced
    // the first and last index can be ignored if any of them is the bound of the slice
    // example
    // &a[0..n] == &a[..n]
    // &a[2..a.len()] == &a[2..]
    
    // NOTE: there is a difference between a reference to an Array and a slice of an array
    let arr = [0; 20];
    let _ref_arr = &arr; // this is a reference to an array of 10 items
    let _slice_arr = &arr[..]; // this is a slice of an array that contains all elements in the array
}