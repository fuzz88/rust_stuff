fn main() {
    let v1 = vec![1, 2 ,3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let total: i32 = v1.iter().sum();

    println!("Sum: {total}");

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("Mapping into new Vec: {v2:?}");
}
