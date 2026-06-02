use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 3.2, 1);

    let (x, y, z) = tup;

    println!("The value of x = {x}");

    let second_value = tup.1;
    let third_value = tup.2;

    
    
    println!("{second_value}, {third_value}");

    let a: [u32; 5] = [1, 2, 3, 4, 5];

    

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
