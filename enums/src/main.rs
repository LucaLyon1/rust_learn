enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_cent(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let value = value_cent(Coin::Dime);
    println!("the value of the coin you inserted is : {}", value);
}