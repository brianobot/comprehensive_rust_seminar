fn main() {
    // if let executes a statement is a value match,
    // it is similar to excuting the match expression but ignoring all other variants

    let some_value = Some(1);
    if let Some(value) = some_value {
        println!("Value is {value}");
    } // an else clause can be used to form a pair of match cases
    // this does not support match guard like the match statement does

    // just like the if let, the while let, continously test a value against a pattern and runs it block
    // if the matches 

    // note that the line below always matches since it's an irrefutable pattern (variable assignment in this case)
    #[allow(irrefutable_let_patterns)]
    let last_name = if let _name = "Brian" {
        "Obot"
    } else {
        "Unknown"
    }; // in this case the if let is used as an expression
    println!("Last Name: {last_name}");

    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("C = {c}");
    }
    // while let can be written with infinite loops with an if blocks that breaks out of the loop
    // when the value does not match the pattern, but the while let is a syntactic sugar for this case

    // the while let can not be used as as expression cause it's possible that the block is not even enterred the first
    // time if the value does not match from the get go, and there is no fall back else block like seen above

}