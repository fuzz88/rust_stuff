use std::fmt::Debug;

#[derive(Debug)]
pub struct NewsArticle {
    pub body: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub body: String,
}

pub trait Summary {
    fn summarize(&self)
    where
        Self: Debug,
    {
        println!("Why do you need that? {:#?}", &self);
    }
}

impl Summary for NewsArticle {}
impl Summary for Tweet {
    fn summarize(&self) {
        println!("{}", &self.body);
    }
}

pub fn print_some_news<T, A>(news: (T, A))
where
    T: Summary + Debug,
    A: Summary + Debug,
{
    news.0.summarize();
    news.1.summarize();
}
