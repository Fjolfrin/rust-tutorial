fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true.");
    }
    else {
        println!("Condition was false.");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    }
    else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("Number is divisible by 2");
    }
    else {
        println!("Number is not divisible by 2, 3, or 4.");
    }
    

    let condition = true;
    let number =  if condition {5} else {6};

    println!("The value of number is: {number}.");
    

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}.")
}
