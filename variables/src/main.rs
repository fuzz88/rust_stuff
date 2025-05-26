use std::io;

fn main() {
    vars();
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed");

    let index: usize = index
        .trim()
        .parse()
        .expect("parse failed");

    let el = a[index];

    println!("a[{index}]={el}");
}

fn vars() {
    let x = 5;
    println!("The value of x is: {}", x);
    {
        let x = 6;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
    
    let tup: (usize, f32, f64) = (500, 2.0, -42.3);
    println!("Tuple created: {:?}", tup);
    println!("3th value: {}", tup.2);

    let arr = [("A", "B"); 5];

    println!("Array created: {:?}", arr);

}
