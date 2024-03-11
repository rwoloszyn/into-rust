

fn main() {
    let guess1:i32 = "42".parse().expect("Not a number!");
    println!("{:?}", guess1);

    let guess2:f32 = "42.22".parse().expect("Not a number!");
    println!("{:?}", guess2);

    let guess3:u32 = "Test".parse().expect("Not a number!");
    println!("{:?}", guess3);
}