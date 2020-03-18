mod math;

fn main() {
    let input1 = "my_first";
    println!("input1 is {}", input1);

    let input2 = "hello";
    println!("input2 is {}", input2);

    let rand_number = rand::random();
    println!("You {}!", if rand_number { "win" } else { "lose" });

    let res = math::add(3, 4);
    println!("3 + 4 = {}", res);

}
