#[rustfmt::skip]

fn main() {
    let input = 'x';

    match input {
        'q'                               => println!("Quitting"),
        'a' | 's' | 'd' | 'w'             => println!("Move Around"),
        '0'..'9'                          => println!("Number input"),
        key if key.is_lowercase()   => println!("Lowercase Input"),
        _                                 => println!("Something else")
    }

    // a variable in the matching arm would create a binding that can be used in the match arm
    // in the case, key would represent the input, 

    // a match guard causes the arm to match only if the condition is true
    // .. is used to represent a range, 
    // | is used to represent OR
    // _ is a wild card that matches if no other branch matches before it
    // @ to bind a part of a pattern to a variable

    match 'A' {
        value @ 'A' => println!("{value}"),
        _ => println!("Something else")
    }
}
