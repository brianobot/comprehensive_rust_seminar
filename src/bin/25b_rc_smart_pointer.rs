#![allow(dead_code)]
use std::rc::Rc;

#[derive(Debug)]
struct Truck {
    capacity: i32
}

fn main() {
    // Reference Counted shared pointer
    // use this when you need to refer to the same data from multple places

    let truck_a = Truck{ capacity: 10 };
    let truck_b = Truck{ capacity: 20 };
    let truck_c = Truck{ capacity: 30 };

    // let facility_a = vec![truck_a, truck_b];
    // let facility_b = vec![truck_c, truck_b];

    // in the case above, facilty_a and facility_b both needs the same truck
    // but passing it into one takes ownership of that data
    // if we clone it, we are simply referring to an different data in memory
    // rc allows us to point to a data from different places

    let truck_a = Rc::new(truck_a);
    let truck_b = Rc::new(truck_b);
    let truck_c = Rc::new(truck_c);

    let facility_a = vec![truck_a, Rc::clone(&truck_b)];
    let facility_b = vec![truck_c, truck_b];

    dbg!(facility_a);
    dbg!(facility_b);



}