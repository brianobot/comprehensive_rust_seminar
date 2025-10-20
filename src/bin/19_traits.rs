// to implement traits types we use the impl Trait for TypeName block
// A trait defines the behaviour that a type must implement in order to be considered of that trait type

trait Pet {
    fn talk(&self) -> String;

    fn name(&self) -> String;

    fn age(&self) -> i8;

    fn greet(&self) {
        println!("Oh! You're so cute, What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        String::from("Woof Woof")
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn age(&self) -> i8 {
        self.age
    }
}

fn main() {
    let bruno = Dog {
        name: String::from("Bruno"),
        age: 5,
    };
    bruno.greet();
    println!("{}'s Name: {}", bruno.name, bruno.name());
    println!("{}'s Age: {}", bruno.name, bruno.age());

    // unlike other programming languagegs, having the trait methods in the type implementation does not
    // automatically make the type a trait type, we have to explicitly implement the trait for the type

    // Super trait: this is a situation where a trait require types implementing it to also implement another trait
    let cat = Cat {
        breed: String::from("white Russian"),
        age: 3,
    };
    println!("Cat is alive: {}", cat.is_alive());
    println!("Cat talking: {}", cat.talk());

    // Associated types are types determined by the implementator of a trait instead of the caller
    // generic is the reverse where the type is determined by the caller and not the implementor
    println!("{:?}", Meters(10).multiply(&Meters(2)));

    // supported traits can be automatically implemented with the derive attribute
    println!("{:?}", Meters::default())
}

trait LivingThing {
    fn is_alive(&self) -> bool;
}

trait Animal: LivingThing {
    fn talk(&self) -> String;
}

struct Cat {
    #[allow(unused)]
    breed: String,
    #[allow(unused)]
    age: i8,
}

impl LivingThing for Cat {
    fn is_alive(&self) -> bool {
        true
    }
}

impl Animal for Cat {
    fn talk(&self) -> String {
        String::from("Meow Meow")
    }
}

#[allow(unused)]
struct Snail {
    color: String,
}

// the compiler raises an error when a super trait is not implemented for a type
// impl Animal for Snail {
//     fn talk(&self) -> String {
//         "todo!()".to_owned()
//     }
// }

trait Multiply {
    type Output;

    fn multiply(&self, other: &Self) -> Self::Output;
}

#[derive(Debug, Default)]
struct Meters(i32);

#[allow(dead_code)]
#[derive(Debug)]
struct MeterSquared(i32);

impl Multiply for Meters {
    type Output = MeterSquared;

    fn multiply(&self, other: &Self) -> Self::Output {
        MeterSquared(self.0 * other.0)
    }
}
