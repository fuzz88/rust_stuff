enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x: _, y: _ } => println!("Move"),
            Message::Write(message) => println!("{}", message),
            Message::ChangeColor(_, _, _) => println!("ChangeColor"),
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum Pets {
    Kitty,
    Doggy,
    cowwy,
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    };

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    };

    let coin = Coin::Quarter(UsState::Alaska);

    let Coin::Quarter(state) = coin else {
        return;
    };

    println!("{:?}", state);

    println!("{:?}", Pets::cowwy);
}
