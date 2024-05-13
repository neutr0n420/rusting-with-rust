use std::io;
fn main() {
    loop{
    let mut convert_trigger = String::new(); 
    println!("Press 'f' to convert temperature to fahrenhheit and 'c' to convert temperature to celsius and 'q' to stop ");
    io::stdin()
        .read_line(&mut convert_trigger)
        .expect("Failed to read line");
    if convert_trigger.trim() == "q"{
        break;
    } 
    println!("Enter the tempreture: ");
    let mut value_to_convert = String::new();
    io::stdin()
        .read_line(&mut value_to_convert)
        .expect("Failed to read line");
    
    let value_to_convert:i32 = match value_to_convert.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid input!, Please enter valid input");
            continue
        }
    };
    let convert_trigger = convert_trigger.trim();

    if convert_trigger == "f"{
    fahrenheit_to_celsius(value_to_convert);
    } else if convert_trigger == "c" {
    celsius_to_fahrenheit(value_to_convert);
    } else{
        println!("Enter valid input!");
        continue;
    }
}
}

fn fahrenheit_to_celsius (value:i32){
    let converted_value = (value - 32) * 5/9;
    println!("Fahrenheit to celsius, {converted_value}");
}
fn celsius_to_fahrenheit (value:i32){
    let converted_value = (value * 9/5) +32;
    println!("Celsius to fahrenheit {converted_value}");
}