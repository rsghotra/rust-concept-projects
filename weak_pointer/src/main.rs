use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

//Node owning children + sharing ownership with other variables => children of type Rc<Node>
//We want to modify which nodes are children oif another node => RefCell<T> in chidlren around Vec<Rc<Node>>

fn main() {
    //creating leaf node with no children or parent

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    //lets check upgrade method to see if it give us None back because there is no parent of leaf yet
    //.borrow will return weak pointer 

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //automatic dereference

    println!(
        "leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)
    );

    {
        //branch has leaf as children
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        //declaring branch as parent of leaf, the parent will have weak pointer set
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

           println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

     println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
     );

}
