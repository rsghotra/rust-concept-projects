use std::{arch::global_asm, f32::consts::GOLDEN_RATIO};

fn main() {
    let favorite_color: Option<&str> = None;

    let is_tuesday = false;

    let age: Result<u8, _> = "38".parse();

    // if let - else if - else if let - else

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30  {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the back ground color");
    }

    //while let -- when we do not know how many number of times patterns
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(val) = rx.recv() {
        println!("{val}");
    }

    //for loop with iter

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let point = (3, 5);
    print_coordinates(&point);

    //matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //match named variables
    let x = Some(45);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    //matching multiple patterns

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //matching ranges -- only char and ints

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ascii chars"),
        'k'..='z' => println!("laste ascii letter"),
        _ => println!("something else"),
    }

    //Destructuring to Break Apart Values - structs, enums, tuples

    //destructuring of structs
    struct Point {
        x: i32,
        y: i32,
    }

    
    let p = Point {x: 0, y: 7};

    let Point { x, y} = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    //destrcuture struct with a match

    match p {
        Point { x, y: 0} => println!("on the x axis at {x}"),
        Point { x: 0, y} => println!("on the y axis at {y}"),
        Point {x, y} => println!("on neither axis: ({x}, {y}"),
    }

    //Enums

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move {x, y} => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g , b ) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }

    //Nested structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message1 {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(Color),
    }

    let msg1 = Message1::ChangeColor(Color::Hsv(0, 160, 255));

    match msg1 {
        Message1::ChangeColor(Color::Hsv(h,s , v )) => println!("Change color to hue {h}, saturation {s}, value {v}"),
        Message1::ChangeColor(Color::Rgb(r,g ,b )) => println!("Change colore to red {r}, green {g}, blue {b}"),
        _ => (),
    }

    //structs and tuples
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10});

    println!("{feet}, {inches}, {x}, {y}");

    foo(3, 4);

    //Parts of a Value ignored with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can not overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _ , fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
    }

    //Unused variable by starting its name with _; note _x it still binds the value to _x but only _ no binding

    let s = Some(String::from("Hello"));

    if let Some(_s) = s {
        println!("found a string");
    }

    //println!("{s:?}"); //move error

    //with only _ no move error occurs

    let s = Some(String::from("too much"));

    if let Some(_) = s {
        println!("found a string");
    }

    //remaining parts of a value with ..
    let numbers = (2, 4, 8, 16, 43);

    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}"),
    }

    struct Point1 {
        x: i32,
        y: i32,
        z: i32,
    }

    let point1 = Point1 {x: 0, y: 0, z: 0};

    match point1 {
        Point1 {x, ..} => println!("x is {x}"),
    }

    //adding conditionals with Match Guards: MG is additional if condition added after matched pattern
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number is odd {x}"),
        None => (),
    }

    //Shadow variable testing
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    //Using @ bindings: The at operator @ lets us create a variable that holds a value at the same time we’re testing that value for a pattern match
    //@ lets us test a value and save it in a variable within one pattern

    enum Message2 {
        Hello { id: i32},
    }

    let msg = Message2::Hello { id: 5};

    match msg {
        Message2::Hello {id: id @ 3..=7} =>{
            println!("Found an id in range: {id}");
        }
        Message2::Hello {id: 10..=12 } => println!("Found an id in another range"),
        Message2::Hello { id } => println!("Found some other id: {id}"),

    }




}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y}");
}

fn foo(_: i32, y: i32) {
    println!("The code only uses the y parameter: {y}");
}