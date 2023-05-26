use std::io;

fn main() {
    println!("Enter which digit of the Fibonacci sequence you want to calculate.");
    
    let mut digit = String::new();

    io::stdin()
        .read_line(&mut digit)
        .expect("Failed to read line...");
    
    let digit: u64 = match digit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("You have not entered a valid number."),
    };

    println!("You want to calculate up to the {digit}th digit.");
    fibonacci(digit)
}


fn fibonacci(d: u64) {
    
    let mut dig: u128 = 1;
    let mut prev: u128 = 0;
    for i in 0..d {
        let index = i + 1;
        println!("{index}. {dig}");
        let swap = dig;
        dig += prev;
        prev = swap;       
    }
}
