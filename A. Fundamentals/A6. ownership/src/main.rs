fn main() {

    let mut s = String::from("Hello");
    s.push_str(", World!");

    println!("{s}");

    

    {
        let num: u32 = 32;
        println!("Within scope {num}");
    }
    //println!("Out of scope {num}");
    
    let x = 5;
    let y = x;

    println!("{x}, {y}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}, {s2}");


    let s = String::from("Hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{} {}", s1, s3);


    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    println!("\n---------Now I'm learning references----------\n");

    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{s1}' is {len}");

    let mut s = String::from("hello");
    println!("String s is: {s}");
    change(&mut s);
    println!("String s is: {s}");


    let mut s = String::from("hello");
    let r1 = & s;
    let r2 = & s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
    
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
