use std::io;

fn main() {
    println!("Choose an option:");
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Convert Celsius to Fahrenheit");

    // Read the user's choice from the input
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    // Convert the input to an unsigned 32-bit integer
    let option: u32 = match option.trim().parse() {
        Ok(num) => num, // If parsing is successful, assign the parsed value to `option`
        Err(_) => {
            println!("Invalid option");
            return; // Exit the program if parsing fails
        }
    };

    // Check if the user chose to convert Fahrenheit to Celsius
    if option == 1 {
        // Prompt the user to input the temperature in Fahrenheit
        println!("Input temperature in Fahrenheit");
        let mut temp_fahrenheit = String::new();
        io::stdin()
            .read_line(&mut temp_fahrenheit)
            .expect("Failed to read line");

        // Convert the input to a 32-bit floating-point number
        let temp_fahrenheit: f32 = match temp_fahrenheit.trim().parse() {
            Ok(num) => num, // If parsing is successful, assign the parsed value to `temp_fahrenheit`
            Err(_) => {
                println!("Invalid temperature");
                return; // Exit the program if parsing fails
            }
        };

        // Perform the Fahrenheit to Celsius conversion
        let temp_celsius = (temp_fahrenheit - 32.0) * 5.0 / 9.0;

        // Print the result of the conversion. Use `:.2` to round to two decimal places.
        println!("{:.2}°F = {:.2}°C", temp_fahrenheit, temp_celsius);
    }
    // Check if the user chose to convert Celsius to Fahrenheit
    else if option == 2 {
        // Prompt the user to input the temperature in Celsius
        println!("Input temperature in Celsius");
        let mut temp_celsius = String::new();
        io::stdin()
            .read_line(&mut temp_celsius)
            .expect("Failed to read line");

        // Convert the input to a 32-bit floating-point number
        let temp_celsius: f32 = match temp_celsius.trim().parse() {
            Ok(num) => num, // If parsing is successful, assign the parsed value to `temp_celsius`
            Err(_) => {
                println!("Invalid temperature");
                return; // Exit the program if parsing fails
            }
        };

        // Perform the Celsius to Fahrenheit conversion
        let temp_fahrenheit = (temp_celsius * 9.0 / 5.0) + 32.0;

        // Print the result of the conversion
        println!("{:.2}°C = {:.2}°F", temp_celsius, temp_fahrenheit);
    }
    // Handle invalid option
    else {
        println!("Invalid option");
    }
}
