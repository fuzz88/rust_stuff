fn main() {
    let mut counter = 0;
    let result = 'first:loop {
        loop {
            counter += 1;
            if counter == 10 { break 'first 5}
        }
    };
    println!("{result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!");

    let a = [1, 2, 3, 4, 5];

    for el in a {
        println!("{el}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
