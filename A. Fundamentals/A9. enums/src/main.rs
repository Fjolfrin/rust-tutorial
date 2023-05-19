use std::println;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama, Alaska, Arizona, Arkansas, California,
    Colorado, Connecticut, Delaware, Florida, Georgia,
    Hawaii, Idaho, Illinois, Indiana, Iowa, Kansas, 
    Kentucky, Louisiana, Maine, Maryland, Massachusetts,
    Michigan, Minnesota, Mississippi, Missuri, Montana,
    Nebraska, Nevada, NewHampshire, NewJersey,
    NewMexico, NewYork, NorthCarolina, NorthDakota,
    Ohio, Oklahoma, Oregon, Pennsylvania, RhodeIsland,
    SouthCarolina, SouthDakota, Tennessee, Texas, Utah,
    Vermont, Virginia, Washington, WestVirginia,
    Winsconsin, Wyoming
}

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    
    route(&four);
    route(&six);
    
    let m = Message::Write(String::from("Hello!"));
    m.call();

    //let some_number = Some(5);
    //let some_char = Some('e');

    //let absent_number: Option<i32> = None;

    //println!("{}, {}, {}", some_number, some_char, absent_number);

    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);

    //let sum = x + y;
    
    value_in_cents(Coin::Quarter(UsState::Alaska));
    

    println!();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?} ,{:?}", five, six, none);

    ///////////////////////////////////////////////////////////

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    ///////////////////////////////////////////////////////////
    
    let config_max = Some(3u8);     
    //match config_max {
        //Some(max) => println!("The maximum is configured to be {}", max),
        //_ => ()
    //}
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }

    let coin = Coin::Penny;
    let mut count = 0;
    //match coin {
        //Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        //_ => count += 1,
    //}
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    
}

fn route(ip_kind: &IpAddrKind) {
    println!("{:?}", ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", {state});
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("You get a fancy hat!");
}

fn remove_fancy_hat() {
    println!("This hat was too fancy for you so you lose it.");
}

//fn move_player(num_spaces: u8) {
    //println!("Player moved by {} spaces", num_spaces);
//}

//fn reroll() {
    //println!("Roll again!");
//}
