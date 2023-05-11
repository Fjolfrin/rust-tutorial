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
}

fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1,}
}
