use std::{println, dbg};

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("mymail@service.com"),
                           String::from("myname"));

    println!("{}, {}, {}", user1.username, user1.email, user1.active);
    println!("{}, {}, {}", user2.username, user2.email, user2.active);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}, {}, {}", user3.username, user3.email, user3.active);
    //println!("{}, {}, {}", user1.username, user1.email, user1.active);
    
    let black = Colour(0, 0, 0);
    let red = Colour(255, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("\nExample program...\n");

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.",
             area(&rect1));
    //dbg!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", 
             rect1.area());
    if rect1.width(){
        println!("The rectangle has non-zero width; it is {}.", rect1.width);
    }

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
    
    let sq = Rectangle::square(3);
    println!("The square is {:#?}", sq);

}

fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1,}
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
   fn width(&self) -> bool {
        self.width > 0
    }
   fn can_hold(&self, other: &Rectangle) -> bool {
       self.width > other.width && self.height > other.height
   }
   fn square(size: u32) -> Self {
       Self {
           width: size,
           height: size,
       }
   }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

