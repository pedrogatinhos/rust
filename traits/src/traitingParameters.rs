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

fn main(){
    pub fn notify(item: &impl Summary){
        println!("breaking!! {}", item.summarize())
    }
    
}