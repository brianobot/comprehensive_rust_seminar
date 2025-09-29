// to implement traits types we use the impl Trait for TypeName block
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
    age: i8
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
    let bruno = Dog{ name: String::from("Bruno"), age: 5 };
    bruno.greet();
    println!("{}'s Name: {}", bruno.name, bruno.name());
    println!("{}'s Age: {}", bruno.name, bruno.age());
}