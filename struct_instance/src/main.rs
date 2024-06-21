//?構造体
// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let user1: User = build_user("email@example.com".to_string(), "kasshii".to_string());
//     println!("{:?}", &user1);
//     let user2 : User = User{
//         email: String::from("kasshii@example.com"),
//         username: String::from("nextkasshii"),
//         ..user1
//     };
//     println!("{:?}", &user2);
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


//?タプル構造体
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(width: u32) -> Self {
        Self { 
            width,
            height: width,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn set_width(&mut self, width: u32){
        self.width = width;
    }
}

fn main() {

    let mut rect = Rectangle{
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(60);
    
    // println!(
    //     "The area of the rectangle is {} square pixels.", 
    //     rect.area()
    // );

    println!(
        "The square area is {} square pixels.",
        square.area(),
    );

    println!("rect: {:?}", rect);
    println!("square: {:?}", square);

    rect.set_width(90);

    println!("rect: {:?}", rect);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }