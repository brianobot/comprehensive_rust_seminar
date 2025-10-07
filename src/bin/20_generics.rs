#![allow(dead_code)]

fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond { left } else { right }
}


// this examples shows how the simple generic code above 
// reduces duplication of code to be implementated for each type that the function needs to work
fn pick_i32(cond: bool, left: i32, right: i32) -> i32 {
    if cond { left } else { right }
}

fn pick_char(cond: bool, left: char, right: char) -> char {
    if cond { left } else { right }
}

fn main() {
    // generics allow implementations to be asbtracted over the data structure used
    // when the pick function is eventually called, rust actually generates a concrete function
    // that operates the type used in the call, this way you get exactly the same results as if you 
    // had hardcoded the function for that type

    // when working with generic you often want to restrict the types to be used in place of those generics
    // to types that implement certain traits, so you can call their trait methods, you can do this by
    // defining the trait bound as so

    fn duplicate<T: Clone>(a: T) -> (T, T) {
        (a.clone(), a.clone())
    }

    // when multiple traits are required use + to seperate them
    fn duplicate_and_copy<T: Clone + Copy>(a: T) -> (T, T) {
        (a.clone(), a.clone())
    }

    // trait bounds can also be expressed with the where clause

    fn duplicate_version_2<T>(a: T) -> (T, T)
    where 
        T: Clone 
    {
        (a.clone(), a.clone())
    }
    // notice that the where clause is between the return type and the start of the function body braces
    // this where clause declutter the function definition
    // 
    pub trait Logger {
        /// Log a message at the given verbosity level.
        fn log(&self, verbosity: u8, message: &str);
    }

    struct StderrLogger;

    impl Logger for StderrLogger {
        fn log(&self, verbosity: u8, message: &str) {
            eprintln!("verbosity={verbosity}: {message}");
        }
    }

    struct VerbosityFilter<L> {
        max_verbosity: u8,
        inner: L,
    }

    impl<L: Logger> Logger for VerbosityFilter<L> {
        fn log(&self, verbosity: u8, message: &str) {
            if verbosity <= self.max_verbosity {
                self.inner.log(verbosity, message);
            }
        }
    }
    // looking at this, VerbosityFilter<L> is the type here
    // and impl<L: Logger> is a generic impl over the generic L

    // impl Trait
    // can be used in function definition

    fn add_42_millions(x: impl Into<i32>) -> i32 {
        x.into() + 42_000_000
    }

    fn pair_of(x: u32) -> impl std::fmt::Debug {
        (x + 1, x - 1)
    }


    let many = add_42_millions(102u8);
    println!("Many: {:?}", many);

    let pair = pair_of(32);
    println!("pair: {:?}",  pair);
    dbg!(pair);

    // dyn Trait
    //  unlike the generic function that generates concrete implementation for each type it is called with
    //  the dyn trait uses the same function for all the type that is passed to it
    // it does this by have a vtable to know which function to call, it's basically translates to
    // this type must be a type that implements a particular trait like in the case of the generic
    // but the type pointed to must be behind some kind of indirection


    fn dynamic(pet: &dyn Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }


}