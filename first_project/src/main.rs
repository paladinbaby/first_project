mod math;
use math::{add};

fn main() {
    let input1 = "my_first";
    println!("input1 is {}", input1);
    let input2 = String::from("hello");
    println!("input2 is {}", input2);
    let rand_number = rand::random();
    println!("You {}!", if rand_number { "win" } else { "lose" });
    let result = add(1, 2);
    println!("1 + 2 = {}", result);
}
