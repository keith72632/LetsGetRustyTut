enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
    None
}

enum Message {
    Quit,
    Move {x: i32, y: i32}, // this is an anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32) //tuple struct
}

impl Message {
    fn display(&self)
    {
        println!("Message");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn new(&self, kind: IpAddrKind, address: String) -> IpAddr {
        IpAddr {
            kind,
            address
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    California
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1")
    };

    // Rust has no Null values, so it has the Option enum. In scope by default
    /*
    enum Option<T>{
        Some(T),
        None
    }
    */
    let some_num = Some(6);
    let some_string = Some(String::from("Whateber"));
    let absent_num: Option<i32> = None;

    let _x: i8 = 5; // m 8 bit interger
    let _y : Option<i8> = Some(5); // 5 or nothing
    // You cannot add Optional types to primitive types
    let _sum = _x + _y.unwrap_or(0); // If no _y use 0
    println!("Sum of x + y {}", _sum);

    // Match allows you to compare a value against a set of patterns, like variables, wildcars 
    let coin_value = value_in_center(Coin::Quarter(UsState::Alabama));
    println!("Coin value is {}", coin_value);

    // Options
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let syntax isn't exhaustive, and only cares about one pattern
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("Three");
    }
}

fn value_in_center(coin: Coin) -> u8 {
    // Match expressions are exhaustive, which means all outcomes must be considered
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter is from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => {
            Some(i + 1)
        },
        _ => None // This means if there is any other pattern 
    }
}
