#[derive(Debug)]

enum UsState {
    Alabama,
    Alsaka,
    Utah,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let out = describe_state_quarter_1(Coin::Quarter(UsState::Alsaka));
    out_printer(&out);
    let out = describe_state_quarter_2(Coin::Quarter(UsState::Alabama));
    out_printer(&out);
    let out = describe_state_quarter_3(Coin::Quarter(UsState::Texas));
    out_printer(&out);
    let out = describe_state_quarter_3(Coin::Quarter(UsState::Utah));
    out_printer(&out);
    value_in_coins(Coin::Penny);
    value_in_coins(Coin::Nickel);
    value_in_coins(Coin::Dime);
}

fn out_printer(out: &Option<String>) {
    match out {
        Some(val) => println!("{val}"),
        None => (),
    }
}

fn value_in_coins(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Coin from US State: {:?}", state);
            25
        }
    }
}

// fn count_no_of_coins(coin: Coin, count: &mut i32) {
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter of {state :?}")
//     } else {
//         *count += 1
//     }
// }

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year <= 1819,
            UsState::Texas => year <= 1959,
            UsState::Alsaka => year <= 1971,
            UsState::Utah => year <= 1930,
        }
    }
}

//method 1: if-let-else way standard

fn describe_state_quarter_1(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1957) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is pretty old, for America!"))
        }
    } else {
        None
    }
}

//method2: if-let-else but if-let giving outer scop variable to use

fn describe_state_quarter_2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1957) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is pretty old, for America!"))
    }
}

//method3: let-else --> the most smart way

fn describe_state_quarter_3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1957) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is pretty old, for America!"))
    }
}
