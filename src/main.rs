use std::io;

fn main() {
    println!("Choose an option:");
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Convert Celsius to Fahrenheit");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid option");
            return;
        }
    };

    if option == 1 {
        println!("Input temperature in Fahrenheit");
        let mut temp_fahrenheit = String::new();
        io::stdin()
            .read_line(&mut temp_fahrenheit)
            .expect("Failed to read line");

        let temp_fahrenheit: f32 = match temp_fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature");
                return;
            }
        };

        let temp_celsius = (temp_fahrenheit - 32.0) * 5.0 / 9.0;

        println!("{:.2}°F = {:.2}°C", temp_fahrenheit, temp_celsius);
    } else if option == 2 {
        println!("Input temperature in Celsius");
        let mut temp_celsius = String::new();
        io::stdin()
            .read_line(&mut temp_celsius)
            .expect("Failed to read line");

        let temp_celsius: f32 = match temp_celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature");
                return;
            }
        };

        let temp_fahrenheit = (temp_celsius * 9.0 / 5.0) + 32.0;

        println!("{:.2}°C = {:.2}°F", temp_celsius, temp_fahrenheit);
    } else {
        println!("Invalid option");
    }
}
