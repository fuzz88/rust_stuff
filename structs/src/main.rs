#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

fn main() {
    let user1 = build_user(String::from("email"), String::from("Pupa"));

    println!("{:#?}", user1);

    let user2 = User {
        email: String::from("normis@mail.boo"),
        ..user1
    };

    println!("{:#?}", user2);
    //println!("{:#?}", user1);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}
