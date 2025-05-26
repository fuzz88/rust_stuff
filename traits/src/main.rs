mod news;
use crate::news:: { Tweet, NewsArticle, print_some_news };

fn main() {
    let couple_of_news = (Tweet{body: "hey tweet".to_string()}, NewsArticle{body: "hey article".to_string()});
    println!("Hello, world!\nGot some news:");
    print_some_news(couple_of_news);
}
