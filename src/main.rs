use std::io;
fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let weigth_on_mars = check_mars_weight(100.0);
    println!("Weight on Mars: {}kg", weigth_on_mars + 1.0);

    Ok(())
}

fn check_mars_weight(weight: f32) -> f32 {
    weight / 9.81 * 3.71
}