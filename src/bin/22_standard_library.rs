fn main() {
    // Option<T> stores either a value of type T or nothing
    // unwrap method will return the value in an option or panic, expect is similar but takes an error message
    // production code should handle None cases in a fashionable way

    // Result is similar to option, but indicates the success or failure of an operation
    // it's generaic Result<T, E>, where T is the type used in the Ok variant and E appears in the Err Variant
    // unwrap and expect can be called too in cases where an error should never happen otherwise you should
    // handle the Err variant fashionably
    // Result is the standard type to implement error handling

    // String is a growable UTF-8 encoded string
    // String::new return a new empty string,
    // String::with_capacity when you know how much data you want to push into the string
    // String::len returns the size of the string in bytes, which mught be different from it's length in character
    // String::chars returns an iteretor over the actual characters
    // When a type implements Deref<Target = T>, the compiler will let you transparently call methods from T
    // String implements Deref<Target = str>, which means you can call str methods on a String type
    // String is implemented as a wrapper round the vector of bytes, alot of methods on Vectors are supported on String
    // you can index a character in the following ways
    // String::chars().nth(i).wrap(): get the ith character from the String
    // String[0..4]
    // many types can be converted to a string with the to_string method, which is implemented for all types
    // that implement the Display trait, so anything that can be formatted can be converted to a string

    // Vec is the standard resizable heap-allocated buffer
    // Vec is a type of collection along with, String and Hashmap
    // the data it stored is stored on the heap and the amount does not need to be known at compile time
    // Vec<T> is a generic type,
    // vec![] is a canonical macro to be used instead of Vec::new(), it support adding initial elements to the vector
    // use [] to index a vector, this can panic if the index is out of bound
    // use get method to return an Option
    // pop will remove the last element

    // Hashmap: key-value pair structure, similar to dictionary in Python
    // Hashmap is not defined in the prelude and needs to be imported to use
    // there is no standard hashmap macro like there is for the Vector type
    // a hashmap can be initialized from a literal array that contains 2 item tuples
    // hashmap can be built from any iterator that yields key-value tuple
    //
}
