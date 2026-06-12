fn main() {
    let v1 = vec![1, 2, 3];

    let  v1_iter = v1.iter();
    
    for item in v1_iter {
        println!("Got: {item}");
    }

    //next takes mutable reference
    //next returns immutable reference on iter()
    //next return owned values on into_iter()
    //next returns mutable references on mut_iter() 
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));

    assert_eq!(v1_iter.next(), None);
    

    //consuming adaptors - Are the methods defined on A ITERATOR that calling them uses up the iterator
    /*
        ex: sum which takes ownership of the iterator and repetadly calls iter to get the sum of all elements
     */

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); //sum takes the ownership of the iterator we call on

    assert_eq!(total, 6);

    /*
    Iterator Adaptors: Are the methods defined on an ITERATOR that dont consume the iterator. Instead,
    they PRODUCE a new ITERATOR by CHANGING SOME ASPECTS of the ORIGINAL ITERATOR
    Ex: map, filter
     */

    let v2 = vec![3,4,5];

    let v2_iter = v2.iter();

    let v3: Vec<_> = v2_iter.map(|x| x + 1).collect();

    assert_eq!(v3, [4, 5, 6]);

    let v4: Vec<_> = v3.iter().filter(|x| **x%2 == 0).collect();

    println!("{v4:?}");

    /*
        You might wonder why the first filter uses *x and the second filter does not. 
         - v.iter() produces an Iterator<Item = &i32>. The .filter() call takes an Iterator<Item = T> as input, and passes &T to its predicate. 
         - Therefore x: &&i32 on line 66. The Rust standard library implements the remainder operator % for &i32 on the left-hand side (see the docs), but not for &&i32. 
         - So we have to dereference x once to use it in the expression *x % 2.

        By contrast on line 67, when .map() takes an Iterator<Item = T> as input, it passes T to its closure. 
        - Therefore the closure in map takes &i32 as input. 
        - The multiplication operator * is implemented for &i32, so x does not need to be dereferenced in x * 2. 
        - The operation x * 2 produces a value of type i32, so the result of the map is an Iterator<Item = i32>. 
        - The filter then takes x : &i32, which also does not need a dereference to do x % 2. Now you know!

        - In rust, most of the arithmetic ops can be run on &var since there is automatic dereferencing which happens
     */

    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);

}
