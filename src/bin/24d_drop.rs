struct Droppable(String);

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    // Values which implements Drop can specify code to run when they go out of scope
    {
        let _bad_habit = Droppable(String::from("Bad Habbit"));
    }

    // there is a function called std::mem::drop that can be used to explicity drop values
    // if a value implemetns the Drop trait, the drop method is called when the value is out of scope
    // the value is still dropped when out of scope regardless of the implementation

    let _bad_practise = Droppable(String::from("Poor Practise"));
    // bad_practise.drop(); // this is not allowed
}
