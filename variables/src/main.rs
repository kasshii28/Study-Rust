fn main() {
    // let mut x: i32 = 5;
    // const CONSTANT: usize = 100;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // println!("The value of CONSTANT is: {}", CONSTANT);


    //シャドーイング
    let y: i32 = 5;
    let y: i32 = y+1;

    {
        let y: i32 = y*2;
        println!("The value of y in the inner scope is : {}", y);
    }

    println!("The value of y is : {}", y);

    let strings = "aaa";
    println!("The value of strings is : {}", strings);

    let strings = strings.len();
    println!("The value of strings is : {}", strings)
}