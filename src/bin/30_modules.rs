mod foo {
    #[allow(unused_variables)]
    pub fn do_something(value: &str) {
        println!("Doing something for foo");
    }
}

mod bar {
    #[allow(unused_variables)]
    pub fn do_something(value: &str) {
        println!("Doing something for bar");
    }
}

fn main() {
    // modules let us namespace functions and types
    foo::do_something("Some information");
    bar::do_something("Some more value");

    mod tea {
        pub fn talk() -> String {
            String::from("Hello there!")
        }
    }

    let secret = tea::talk();
    println!("Secret: {}", secret);

    // if we omit the module content in the modules above, it will force rust to look for it in another file
    // mod garden; this would tell the compiler to look src/garden.rs
    //
    // before run 2018, modules had to be placed in files like module/mod.rs instead of module.rs
    // this still works not, but if you want to have deeper nesting you can name top level file the same as the directory
    //
    // top_module.rs
    // top_module/
    //      sub_module.rs
    //
    // this can be used as top_module::sub_module
    //
    // Modules items are private by default, parent and sibling items are always visible
    // use pub to make modules public

    // structs fields are private by default too and are visible to all descendant of a module
    // a module can bring symbols from another module into scope with use
    //
    #[allow(unused_imports)]
    use std::collections::HashMap;
    #[allow(unused_imports)]
    use std::process::abort;

    // Paths
    // foo or self::foo refers to foo in the current module
    // super::foo refers to foo in the parent module
    //
    // crate::foo refers to foo in the current crate
    // bar::for refers to foo in the bar crate
    //
    // A trait must be in scope call methods on that trait
    // even if the type implementing the trait is already in scope
}
