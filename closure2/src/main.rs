/*
Fn Traits: Closures implements one, two or all three Fn Traits depending upon what it does with the data
- Closures are capable of capturing from its surrounding environments

A closure body can do 4 things:

- FnOnce: Move the captured values out of the closure --> FnOnce trait gets implemented: This closure can be called only once as values would have moved after that call
- FnMut: Mutate the captured value without move --> FnMut --> Can be called multiple times + since value does not move out of clousre -> safe to call several times
- Fn: Neither mutate or move the captured value || Capture nothing --> Fn --> Can be called multiple times + Captured value does not mutate; This becomes a useful property in concurrent systems

As we know structs/functions generally are Traitbound and this traitbound defines which closures can be used over these types

*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}




fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    //FnMut implementation
    list.sort_by_key(|r| r.width); //does not move but mutate the values
    /*
    |r| r.width closured does not capture, mutate, or move anything out from it env, so it meets trait bound requirements
    outlined by sort_by_key; sort_by_key is defined to take FnMut
     */
    println!("{list:?}");

    let mut sort_operations = vec![];

    let value = String::from("closure called");
    sort_operations.push(value);

    //this will not compile as sort_by_key is implements FnMut traits which prohibit moving values in/out of closure body
    // list.sort_by_key(|r| {
    //     sort_operations.push(value); //value moved in and then moved out to sort_operations vector; therefore this closure is implementing
    //                                    FnOnce which is not acceptable to sort_by_key
    //     r.width
    // });

    println!("{list:?}");

}
