pub struct NewArticles { 
    pub author: String,
    pub content: String,
    pub headline: String,
 }
pub struct Tweets { 
    pub username: String, 
    pub content: String, 
    pub reply: bool,
    pub retweet: bool 
}

fn main() {
    println!("Hello, world!");
}
