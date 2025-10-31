#![allow(dead_code)]

struct Dog {
    name: String,
    age: i8,
}

struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String {
        format!("Hello I am talking")
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof Woof")
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Miauuu")
    }
}

fn main() {
    // Trait objects can be used with smart pointers

    // imagine we want a vector of types that implementss a Pet Trait
    // now we can try to do this
    // let pets: Vec<dyn Pet> = vec![]; this is basically saving, i want to store different types that implement the Pet trait
    // but since the compiler has to know the size of the types to be stored, it can not know which types implements the Pet trait at runtime
    // we so have to put that value on the heap and store a pointer to those values

    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Dog {
            name: String::from("Bruno"),
            age: 2,
        }),
        Box::new(Dog {
            name: String::from("Sparks"),
            age: 3,
        }),
        Box::new(Cat { lives: 3 }),
    ];

    for pet in pets {
        println!("Hello! How are you?, {}", pet.talk());
    }

    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}
