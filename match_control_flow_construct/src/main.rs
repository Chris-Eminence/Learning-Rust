
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--`

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    println!("Hello, world!");
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    

    
    
