use std::cell::Cell;

fn main() {
    // cells wraps a value and can allow the value to be get or set
    // but you can't get a reference to the inner value
    let cell = Cell::new(5); // notice that the cell variable is not mutable

    println!("Cell = {:?}", cell);
    cell.set(10);

    println!("Cell = {:?}", cell);

    // Research on the RefCell cause i do not quite understand how it differs from the Cell type
}
