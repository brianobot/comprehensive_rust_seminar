// this is an example of a named struct, similar to struct in C and C++
// and similar to classes in Python
struct Person {
    name: String,
    age: u8,
}

fn describe_person(person: &Person) {
    println!("Name is {0}! {0} is {1} years old", person.name, person.age);
}

struct Point(i32, i32);

fn main() {
    // the struct is a template for creating objects
    let first_person = Person {
        name: String::from("Brian David Obot"),
        age: 25,
    };

    describe_person(&first_person);

    // if the name of a variable match the field of a struct a shorthand can be used to create
    // the struct instance

    let name = String::from("Brian Obot");
    let age = 25.4 as u8;

    let original_person = Person { name, age };
    describe_person(&original_person);

    // we can use the fields from a struct to fill in another struct
    let mutant = Person { ..original_person };
    describe_person(&mutant);

    // tuple struct are unnamed struct whose fields are identified by their positions as in regular tuples

    let origin = Point(0, 0);
    println!("Origin is on {} x axis and {} y axis", origin.0, origin.1);

    // this can be used for single fields wrapper with unimportant field names
    #[allow(unused)]
    #[derive(Debug)]
    struct Newton(f64);

    let ten_netwtons = Newton(10.0);
    println!("Force is {:?}", ten_netwtons);

    // when a tuple field has zero field the () can be omitted
    #[allow(unused)]
    struct SemiColon;
    // this is common for types that implement some behavior but have no data

    let size_of_i32 = size_of::<i32>();
    println!("Size of I32 = {}", size_of_i32);
}
