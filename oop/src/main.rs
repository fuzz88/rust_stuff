trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}: {}x{}", self.label, self.width, self.height);
    }
}

fn main() {
    println!("@@@");
    let button: Box<Button> = Box::new(Button {
        width: 10,
        height: 10,
        label: String::from("button"),
    });

    println!("address of the box<button>: {:p}", button);

    let mut screen = Screen { components: vec![] };
    screen.components.push(button);
    screen.run();
    println!("the end of the program");
}
