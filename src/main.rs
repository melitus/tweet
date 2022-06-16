pub struct NewArticles { 
    pub author: String,
    pub content: String,
    pub headline: String,
 }

 impl Summary for NewArticles {
    fn summary(&self) -> String {
        format!("{} by {}", self.headline, self.author) 
    }
}

pub struct Tweets { 
    pub username: String, 
    pub content: String, 
    pub reply: bool,
    pub retweet: bool 
}

pub trait Summary {
    fn summary(&self) -> String;
}

fn main() {
    println!("Hello, world!");
}
