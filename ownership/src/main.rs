fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    greet(&m1, &m2);

    let s = format!("{} {}", m1, m2);

    println!("{s}");

    derefernce();
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn derefernce() {
    let mut x = Box::new(1);

    let a = *x; //*x reads the heap value, so a = 1 */
    *x += 1; //*x  on the left side modies the heap value so x points to 2*/
    let r1 = &x;
    let b = **r1; //two dereferences get us to the heap value

    let r2 = &*x;
    let c = *r2;

    println!("{} {} {}", a, b, c);

    //implicit insertion of reference and dereference happens in . operator

    let x = Box::new(-1);

    let x_abs1 = i32::abs(*x); //explicit derefence

    let x_abs2 = x.abs(); //implicit derefence

    assert_eq!(x_abs1, x_abs2);

    let r = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);

    let s_len2 = s.len();

    assert_eq!(s_len1, s_len2);

    let mut v = vec![1, 2, 3, 4];

    v.push(5);

    let mut s = String::from("Hello World!");

    let s1 = &s;
    println!("{s1}");

    let s2 = &mut s;

    s2.push_str(" pee");
    println!("{s2}");

    let v: Vec<String> = vec![String::from("Yipee")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    let v: Vec<i32> = vec![2, 3, 4];
    let v_ref = &v[0];
    println!("{v_ref}");

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    //s.clear();

    println!("the first word is: {}", word);

    let my_string = String::from("Chalo World!");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "Hello World";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);

    let mut s = String::from("hello");

    
    println!("&String={}, &str={}", std::mem::size_of::<&String>(), std::mem::size_of::<&str>());

}

fn return_a_string() -> &'static str {
    "Hello World"
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap().clone();

    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

fn add_big_strings_v1(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap();

    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();

    dst.extend(to_add);
}

fn add_big_strings_v2(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}
