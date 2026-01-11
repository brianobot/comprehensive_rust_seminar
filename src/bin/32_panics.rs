use std::panic;
 
fn main() {
    // panics are for unrecoverable and unexpected errors
    // purpose-specific panics can be the panic! macro
    // 
    // when a panic happens the compiler will unwind the stack, dropping values as if the function had returned
    // the unwinding can be caught
    let result = panic::catch_unwind(|| "No Problem here");
    dbg!(result);
    
    let result = panic::catch_unwind(|| { panic!("oh no") });
    dbg!(result);
    //
    // do not use the catch_unwind as exception catcher
    // this does not work if panic = 'abort' is set in Cargo.toml
}