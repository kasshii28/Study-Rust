//?match式
#[derive(Debug)]

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    let _maybe_num: Option<u32> = Some(5);
    let green: Color = Color::Green;
    println!("Green Color Code: {}", color_to_str(&green));
    let red: Color = Color::Red;
    println!("Red Color Code: {}", color_to_str(&red));
    let blue: Color = Color::Blue;
    println!("Blue Color Code: {}", color_to_str(&blue));
    let yellow: Color = Color::Yellow;
    println!("Yellow Color Code: {}", color_to_str(&yellow));
    find_maybe_num(Some(5));
    find_maybe_num(None);

    let maybe_num: Option<u32> = None;

    if let Some(number) = maybe_num {
        println!("number: {}", number);
    } else {
        println!("Nothing found");
    }
}

fn find_maybe_num(maybe_num: Option<u32>){
    match maybe_num {
        Some(number) => println!("found {}", number),
        None => println!("Nothing found"),
    }
}

fn color_to_str(color: &Color) -> &str {
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
        Color::Yellow => "#FF00FF",
    }
}