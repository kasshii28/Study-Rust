pub mod second;
pub mod third;

use second::hello;
use third::return_three::return_three;

fn main() {
    hello();
    println!("{}", return_three());
}
