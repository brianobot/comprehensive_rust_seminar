fn main() {
    println!("{1} {} {0} {}", 1, 2);
    // here the format string first filled the first place {1} with the value @ index 1 which is 2
    //  then for the second placeholder {} it takes the first item in the iterators of arguments which is 1
    // then for the third, it uses positional check again like in the first and gets the item at the oth index which is 1
    // for the forth, it takes the next item in the iterator of argument, which is 2, 
    // this means that if we provide more arguments that the string consumes, it would fail
    // this samples would fail
    // println!("{}{}{}", 1, 2, 3, 4);
    // println!("{}{}", 1);

    println!("{:.*}", 2, 2.326);

    // when using the : to format the type of an argument the character after : determines how the argument
    // would be formatted
    // nothing means it should use the Display trait to format the argument
    // :.* float trait
    // :? Debug
    // :o Octal
    // :x lower Hex
    // :X UPPER Hex
    // :p Pointer
    // :b Binary
    // :e lower EXPONENT
    // :E UPPER EXPONENT

    // what this means that any type that implements that traits for a particular formatting can be used in that formatting
    println!("Binary 10: {:b}", 10);
    println!("Hexdecimal 10: {:X}", 10);
    println!("Upper Exponent: {:E}", 100000);
    println!("Lower Exponent: {:e}", 100000);

    // adding # in front of the format argument adds pretty printing to the formater
    println!("Binaary 11: {:#b}", 11);
    println!("Hexadecimal 12: {:#x}", 12);

    // notice how we did not use the variable inside the placeholder
    // if you want to use the varaible inside the placeholder like this {variable_name} and also use the formatter argument
    // the argument must come after the variabel
    let ten = 10;
    println!("Octal Form of 10: {ten:o}");

}