

fn some_value(value: i32) -> Result<i32, &'static str> {
    return if value > 32 { Ok(value) } else { Err("invalud value") }
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
}