struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count : u64,
}

fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusernam123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@exmaple.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        username: String::from("whattheheck"),
        ..user2
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



struct AlwaysEqual;
