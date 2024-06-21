fn main() {
    //?所有権
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
    //let s= String::from("hello");
    // {
    //     let r1 = &mut s;
    //     println!("r1 : {}", r1);
    // }

    //?参照と借用
    // let r1 = &s;
    // let r2: &String = &s;
    // println!("r1: {}, r2: {}", r1, r2);

    //借用不可
    // fn main() {
    //     let s = String::from("hello");
    //     change(&s);
    // }
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }

    //借用可
    // let mut s = String::from("hello");
    // change(&mut s);
    // fn change(some_string: &mut String) {
    //     some_string.push_str(", world");
    // }

    //?スライス型
    
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        s
    }

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    // first_wordは`String`のスライスに対して機能する
    let _word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    // first_wordは文字列リテラルのスライスに対して機能する
    let _word = first_word(my_string_literal);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // 文字列リテラルは「それ自体すでに文字列スライスなので」、
    // スライス記法なしでも機能するのだ！
    let _word = first_word(my_string_literal);
}
