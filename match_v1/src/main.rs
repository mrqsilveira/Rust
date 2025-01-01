enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {

    let coin = Coin::Penny;

    let value = value_in_cents(coin);
    println!("Penny value is {} cents.", value);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}