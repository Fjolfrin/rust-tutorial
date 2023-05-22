use std::{vec, sync::RwLockWriteGuard, collections::HashMap};

fn main() {

    // Vectors!!!
    //let v1: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);
    
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}.");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    //let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist0 = &v[100];
    //let does_not_exist1 = v.get(100);

    //println!("{:?}, {:?}", does_not_exist0, does_not_exist1);
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6);
    println!("The first element is {first}.");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);


    // Strings!!!


    //let mut s = String::new()
    let data = "initial contents";
    //let s = data.to_string();
    //let s = "initial contents".to_string();
    let s = String::from("initial contents");
    println!("{data}");
    

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:?}", s);
    let s2 = "200";
    s.push_str(s2);
    println!("{:?}", s);
    s.push('0');
    println!("{:?}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{:?}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{:?}", s);

    
    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }


    // HashMaps!!!
    
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for k in scores.keys() {
        println!("{:?} -> {:?}", k, scores[k]);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{:?} -> {:?}", team_name, score);

    let text  = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
