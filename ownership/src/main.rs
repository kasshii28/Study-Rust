fn main() {
    // {
    //     let s = "hello";
    //     println!("{}", s);
    // }

    //println!("{}", s);

    // {
    //     let mut s = String::from("hello");
    //     s.push_str(", world!"); //helloにworldを結合
    //     println!("{}", s);
    // }

    //osにメモリ返す

    let s= String::from("hello");
    // {
    //     let r1 = &mut s;
    //     println!("r1 : {}", r1);
    // }

    let r1 = &s;
    let r2: &String = &s;
    println!("r1: {}, r2: {}", r1, r2);

    //借用不可
    // fn main() {
    //     let s = String::from("hello");
    
    //     change(&s);
    // }
    
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }

    //借用可
    let mut s = String::from("hello");

    change(&mut s);
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    
}
