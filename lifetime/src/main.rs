// pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     // 最長の文字列は、{}です
//     println!("The longest string is {}", result);
// }

// #[derive(Debug)]
// struct SomeStruct <'a> {
//     part: &'a str,
// }

// fn main() {
//     let s: SomeStruct;
//     {
//         let novel = String::from("Call me Ishmael. Some years ago...");
//         s = SomeStruct { part: &novel };
//         println!("{:?}",&s);
//     }
//     //println!("{:?}",&s);
// }

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where // トレイトの集約
    T: Display,
{
    //       "アナウンス！ {}"
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
