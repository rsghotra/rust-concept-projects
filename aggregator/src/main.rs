//import crate, type, and trait | we are implementing the trait Summary on Type SocialPost and its crate is local to them

//NOTE: we can not implement external traits on external types

use aggregator::{SocialPost, NewsArticle, Summary};
use std::fmt::{Display, Debug};

//Trait Syntax - 1
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

//Two traits getting implemented
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}


//Trait bound syntax - detailed

pub fn notify4<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

pub fn some_function<T: Summary + Display + Clone, U: Summary + Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("Breaking news! {}", t.summarize());
    println!("Breaking news! {}", u.summarize());
    return 4
}

pub fn some_function1<T, U>(t: &T, u: &U) -> i32 
where
    T: Summary + Display + Clone,
    U: Summary + Clone + Debug,
{
    println!("Breaking news! {}", t.summarize());
    println!("Breaking news! {}", u.summarize());
    return 4
}

//Trait bound - when we want both items to be of same Type

pub fn notify5<T: Summary>(item1 : &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

//Returning a Type which implement a Trait | We cano have two diff types return from this function - for that we have Trait Objects
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("ofcourse"),
        reply: false,
        repost: false,
    }
}

//method getting trait bound

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x: {}", self.x);
        } else {
            println!("The largest number is y: {}", self.y);
        }
    }
}

//trait getting implemented based on itself Trait bound

// impl<T: Display> ToString for Summary {

// }

fn displayable<T: Display>(t: T) -> impl Display {
    t
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, you probably alreadey know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins with the Stanley cup championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The PTA is owwesome in NHL"),
    };

    println!("New article available! {}", article.summarize());

    returns_summarizable();

    let pair = Pair::new(3, 4);

    pair.cmp_display();



}