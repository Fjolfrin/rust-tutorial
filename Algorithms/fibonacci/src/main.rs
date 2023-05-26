use std::io;
use std::time::Instant;

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
    let t = Instant::now();
    fibonacci(digit);
    let duration = t.elapsed();

    println!("Caclulated {} digits of the Fibonacci sequence at {:?}.", digit, duration);
}


fn fibonacci(d: u64) {
    
    let mut dig: num::BigUint = num::traits::One::one();
    let mut prev: num::BigUint = num::traits::Zero::zero();
    for _ in 0..d {
        //let index = i + 1;
        //println!("{}. {}", index, dig);
        let swap = dig.clone();
        dig += prev;
        prev = swap;       
    }
}
