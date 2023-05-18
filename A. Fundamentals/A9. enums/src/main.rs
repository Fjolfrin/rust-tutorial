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

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

fn main() {

    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    
    route(&four);
    route(&six);
    
    let m = Message::Write(String::from("Hello!"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    //println!("{}, {}, {}", some_number, some_char, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;
}

fn route(ip_kind: &IpAddrKind) {
    println!("{:?}", ip_kind);
}
