#[derive(Debug)]
enum UsState {
    Alabama,
    Illinois,
    Texas
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25
    }
}

fn main() {
    let coins = [
        Coin::Penny,
        Coin::Quarter(UsState::Texas),
        Coin::Dime,
        Coin::Dime,
        Coin::Penny,
        Coin::Nickel,
        Coin::Penny,
        Coin::Nickel,
        Coin::Quarter(UsState::Alabama),
        Coin::Nickel,
        Coin::Quarter(UsState::Illinois),
        Coin::Dime,
        Coin::Quarter(UsState::Alabama)
    ];

    let mut value = 0;
    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state)
        } else {
            println!("Regular coin")
        }
        value += value_in_cents(coin);
    }

    println!("Total value: {}", value);
}
