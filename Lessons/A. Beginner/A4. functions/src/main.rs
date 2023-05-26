fn main() {
    println!("Hello, world!");

    another_function();
    value_of_x(32);
    print_labeled_measurement(40, 'm');

    let y = {
        let x = 5;
        x + 1  // Expression
    };
    println!("y = {y}");
    
    let five = five();
    println!("This is five: {five}");

    let five_plus_one = plus_one(five);
    println!("This is five plus one: {five_plus_one}")
}

fn another_function() {
    println!("And goodnight, world!");
}

fn value_of_x(x: i32) {
    println!("The value of x is: {x}.");
}

fn print_labeled_measurement(value: i32, unit_label:char) {
    println!("The measurement is {value}{unit_label}.");
}

fn five() -> i32 {
    5  // Expression
}

fn plus_one(x: i32) -> i32 {
    x + 1  // Expression
}
