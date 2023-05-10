fn main() {
    let s = String::from("Hello World");

    let len = s.len();
    println!("{}, {}, {}, {}", &s[..], &s[0..len], &s[6..], &s[..5]);
    let word = first_word(&s);
    println!("{word}");
    
    let word = first_word(&s[0..6]);
    println!("{word}");

    let word = first_word(&s[..]);
    println!("{word}");
    
    let s_literal = "Hello World";
    let word = first_word(&s_literal);
    println!("{word}");

    let word = first_word(&s_literal[0..6]);
    println!("{word}");

    let word = first_word(&s_literal[..]);
    println!("{word}");


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

