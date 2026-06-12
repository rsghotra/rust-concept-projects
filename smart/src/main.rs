#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    //long way
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    //shorter way using use
    let list_2 = Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))));

    println!("List1 is {:?}", list);

    println!("List1 is {:?}", list_2);

}