use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn main() {
    let numlist = vec![1, 2, 3, 4, 5];
    println!("{}", largest(&numlist));
    let numlist = vec![100, 43, 50, 79, 5000, 1, 0];
    println!("{}", largest(&numlist));
    let numlist = vec![100.11, 43.25, 50.098, 79.56, 5000.51, 11.1, 0.456];
    println!("{}", largest(&numlist));

}

//? トレイト

// fn main() {
//     // summaryトレイトを定義
//     pub trait Summary {
//         // sum_authorとsummarizeを定義
//         fn summarize_author(&self) -> String;

//         fn summarize(&self) -> String {
//             // "（{}さんの文章をもっと読む）"
//             format!("(Read more from {}...)", self.summarize_author())
//         }
//     }

//     // tweet構造体
//     pub struct Tweet {
//         pub username: String,
//         pub content: String,
//         pub reply: bool,
//         pub retweet: bool,
//     }

//     // A for Bでトレイト名と適用する型を記述
//     impl Summary for Tweet {
//         fn summarize_author(&self) -> String {
//             format!("@{}", self.username)
//         }
//     }

//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }