struct Point(i32, i32);

fn main() {
    {
        let p = Point(2, 1);
        dbg!(p.0);
    }

    // dbg!(p.1); # this will not compile if uncommented

    let s1 = String::from("Hello World!");
    let s2 = s1; // at this point the ownership of the String has been transferred to s2 and s2 is invalid

    // println!("S1 = {s1}"); # this would not work anymore

    // passing a valur to a function moves the value into the function parameter
    fn say_hello(name: String) {
        println!("Hello {name}");
    }

    say_hello(s2);
    // at this point s2 is not valid anymore cause it has been moved into the name argument
    // we can also not call say_hello again wth the same name beause the former one is out of scope and the value has been freed

    // some types that implement the Copy traits are automatically copy instead of moved
    // Clone in rust is explicit,
}
