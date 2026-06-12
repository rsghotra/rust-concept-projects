//trait is local to crate and types like NewsArticle + SocialPost type
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read  more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//IMPLEMENTING SUMMARY TRAIT
impl Summary for NewsArticle {
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}


pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//method with trait bound - example
