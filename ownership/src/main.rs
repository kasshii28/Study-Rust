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
    println!("r1: {}, r2: {}", r1, r2)

}
