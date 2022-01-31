use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    print!("Please input your weight: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight);
    println!("Weight on mars: {}lbs\n", mars_weight);
    Ok(())
}

fn calculate_weight_on_mars(_weight: f32) -> f32 {
    return (_weight / 9.81) * 3.711;
}