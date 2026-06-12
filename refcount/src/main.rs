use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

/*
    Rust does not allow Copy for any type that owns heap memory, like:
    Box<T>
    String
    Vec<T>
    Rc<T>

    impl Copy for List {} //wont work
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {


    // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));

    // let b = List::Cons(3, Box::new(a));

    // let c = List::Cons(4, Box::new(a));
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));
    /*
        Q: Why do we have to wrap a with Rc::new(Cons...)?
        Because a is the thing that is going to be shared. Look at the enum
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        The tail of every Cons must be an Rc<List>, not a plain List.

        When you create: let b = Cons(3, Rc::clone(&a));
        and let c = Cons(4, Rc::clone(&a));
        both b and c need to point to the same tail (a).

        What is a?

        If you wrote: let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));

        then a would have type:List not Rc<List>

        Now this fails: Rc::clone(&a) because Rc::clone() expects an Rc<T>: 
        Rc::clone(&some_rc) not a plain List.

    */

    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = List::Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));


}
