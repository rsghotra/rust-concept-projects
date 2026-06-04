fn main() {
    println!("Hello, world!");
    // let mut v = vec![1,2,3];

    // for i in &mut v { //mutable borrow -1
    //     //v.push(*i); //ineligible mutable borrow
    // }
    // println!("{} {} {}", v[3], v[4], v[5]);

    //
    let mut v = vec![1, 2, 3];
    let mut v2 = Vec::new();

    for i in &mut v {
        v2.push(i);
    }

    *v2[0] = 5;

    let a = *v2[0]; //copyable as its i32

    let b = v[0];

    println!("{a} {b}");
}
