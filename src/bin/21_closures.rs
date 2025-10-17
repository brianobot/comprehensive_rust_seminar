fn main() {
    // closures create with vertical bars
    let double_it = |n| n * 2;
    println!("{}", double_it(45));

    // we can specify the types of the arguments
    let triple_it = |n: i32| -> i32 { n * 3 };
    println!("{}", triple_it(10));

    // for single line closures, you can ignore the braces expected of a function body
    // but the can be used for clarity as in the case above

    // the arguments for a closure go between the vertical bars
    // arguments types are optional unlike regular functions and are inferred if not given
    // regular functions can be stored in variablse and invoked later one
    let sec = second_main;
    sec();

    // when functions are stored to a variables they type annotation can express the return type of the function too
    let sen: fn() -> () = second_main;
    sen();

    // this rule also apply to closures too
    // a closure can capture the variables from the environment it is defined in

    let max_value = 5;
    let clamp = |v| {
        if v > max_value { max_value } else { v }
    };

    dbg!(clamp(10));
    dbg!(clamp(2));
    dbg!(clamp(4));
    dbg!(clamp(6));
    dbg!(clamp(30));
    dbg!(clamp(3));
    println!("{}", max_value);

    // by default values are captured by reference and still valid even after they are used inside the closure
    // but we can force the compiler to move the variable into the closure by using the move keyword infront of the closure
    // vertical argument bars

    let clamp_v2 = move |v| {
        if v > max_value { max_value } else { v }
    };

    let result = clamp_v2(20);
    println!("Result: {result}");

    let second_result = clamp_v2(24);
    println!("Second Result: {second_result}");
    // it is important to notice that even though the value was moved inside the closure
    // the closure can be called multiple times with the same variable already moved into it
    // but the move value can not be used outside the closure no more

    println!("Max Value: {max_value}")
    // this lines still works here because the i32 type implements the Copy trait
    // and was not move but just copy so it previous variable in the main scope was still valid
    
    // Closure traits
    // Fn are functions/closures that neither consumes nor mutates capture values
    // FnMut might mutate captured values 
}


fn second_main() {
    println!("The second function was called!")
}