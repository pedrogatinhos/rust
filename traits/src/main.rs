
pub trait Summary {
    fn summarize(&self) -> String;     //overwrite 

    fn defaultbehavior(&self) -> &str{
        "this is a example of a default behavior"
    }
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,

}

impl Summary for Article{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)   
    }
}
impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("pedrogatinhos"),
        content: String::from("falando abrobrinhas"),
        reply: false,
        retweet: false,
    };
    println!("new tweet! {}", tweet.summarize());
    println!("{}", tweet.defaultbehavior());
}
