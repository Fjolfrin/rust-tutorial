use std::{io, cmp::Ordering};


fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{spaces}");

    //////////////////////////////////////////////////

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("sum: {sum}, difference: {difference} \
              product: {product}, quotient: {quotient} \
              truncated: {truncated}, remainder: {remainder}");
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyes_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyes_cat: {heart_eyes_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let other_tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: {tup:.?}, other_tup: {other_tup:.?}, x: {x}, y:  {y}, z: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("a: {a:.?}, months: {months:.?}, b: {b:.?}, c: {c:.?}, first: {first}, second: {second}");

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");
    
    let element = a[index];
    println!("The value of the element at index {index} is: {element}.");
    
}
