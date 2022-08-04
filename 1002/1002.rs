use std::io;

fn main() {
    let mut inputRain = String::new();

    io::stdin().read_line(&mut inputRain);
    
    let rain: f64 = inputRain.trim().parse().unwrap();
    let pi: f64 = 3.14159;
    let area: f64 = rain * rain * pi;

    println!("A={:.4}", area);
}