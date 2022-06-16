use std::fmt::format;

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

impl Summary for Tweets {
    fn summary(&self) -> String {
        format!("{} by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}

fn main() {
    let tweet = Tweets {
        username: String::from("@melitus"),
        content: String::from("Hello, world!"),
        reply: true,   
        retweet: true, 
    };

    let new_articles  = NewArticles{
        author: String::from("Sunday Aroh"),
        content: String::from("Election is coming up in Nigeria!"),
        headline: String::from("Election 2023"),    
    };

    print!("Tweets summary: {}\n", tweet.summary());
    print!("Articles summary: {}", new_articles.summary());
}
