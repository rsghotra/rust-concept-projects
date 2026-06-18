use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("a after = {b:?}");
    println!("a after = {c:?}");
    
}
