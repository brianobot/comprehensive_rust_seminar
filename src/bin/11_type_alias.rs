#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

enum ExplodeableItem {
    Bomb,
    Flameable,
}

// a type alias creates a name for another type, the two types can be used interchaneably
type FireRisk = ExplodeableItem;

// type alias are more useful with long complex types
type PlayerInventory = RwLock<Vec<Arc<RefCell<FireRisk>>>>;

fn main() {
    // notes: A type is often preferred over a type alias, since it create a distinct type
    // struct InventoryCount(i32) is better than let InventoryCount = i32;
}
