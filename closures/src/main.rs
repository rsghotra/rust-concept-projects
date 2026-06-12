use std::thread;
use std::time::Duration;
#[derive(Debug, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: i32| -> i32 {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let add_one_v1 = |x: u32| -> u32 { x + 1 };
    let add_one_v2 = |x| x + 1; //need to be evaluated to be compiled

    let t = add_one_v2(33);

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    //Capturing References or Moving Ownership
    let mut list = vec![1,2,3];

    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // /println!("{list:?}");

    borrows_mutably();
    println!("After calling borrow mutably: {list:?}");

    thread::spawn(move || println!("From  thread: {list:?}")).join().unwrap();

     
}
