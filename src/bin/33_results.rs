

fn some_value(value: i32) -> Result<i32, &'static str> {
    return if value > 32 { Ok(value) } else { Err("invalud value") }
}


fn perform_some_action(input: i32) -> Result<i32, String> {
    let result = some_value(input)?;
    Ok(result)
}

fn main() {
    // Result has 2 variant Ok and Err
    // Whether or not a function can produce an error is encoded in the funnction signature
    // by having the function return a Result value
    // 
    // methods like unwrap provide a quick and dirty way to access success or error values
    match some_value(4) {
        Ok(value) => println!("Value: {:?}", value),
        Err(msg) => println!("Error Occured: {:?}", msg),
    }
    
    // let result = some_value(4)?; // this can only be used in functions that return Result value
    // when using the ? operator, the return error must match the one specified in the function signature
    // if you want to get dynamic error types you can annotate the Result type as so
    // 
    use std::error::Error;
    
    fn return_dyn_error() -> Result<i32, Box<dyn Error>> {
        Ok(1)
    }
    
    
    let result = perform_some_action(12);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(msg) => println!("Error: {}", msg),
    }
    
    // if you do not care for the particular type of error returned from a function
    // you can use the std::error::Error trait to represent an arbitary error
    use std::fs;
    use std::io::Read;
    
    // Boxing the error trait allow us to work with multiple different errors from
    // a function without specifying, which saves on code written, but in a public api 
    // this is not generally a good idea
    fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
        let mut count_str = String::new();
        fs::File::open(path)?.read_to_string(&mut count_str)?;
        let count = count_str.parse::<i32>()?;
        Ok(count)
    }
    
    fs::write("count.dat", "1i3").unwrap();
        match read_count("count.dat") {
            Ok(count) => println!("Count: {count}"),
            Err(err) => println!("Error: {err}"),
        }
}