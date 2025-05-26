mod garden;

use garden::vegetables::Asparagus;
use std::collections::HashMap;

fn deliver_smthn() {}

pub mod delivery_service {
    pub fn fix_all() {
        super::deliver_smthn();
        wait_orders();
    }
    fn wait_orders() {}
}

fn main() {
    println!("Hello, world!");

    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    delivery_service::fix_all();

    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(2, 3);
    map.insert(3, 4);
    map.insert(4, 5);

    println!("Map len is {}", map.len());
    println!("Map cap is {}", map.capacity());

    map.shrink_to_fit();

    println!("Map cap is {}", map.capacity());

    for (key, item) in map {
        println!("{} => {}", key, item);
    }
}
