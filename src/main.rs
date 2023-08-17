use std::io;
//To accept user input

fn main() {
    //To convert from Fahrenheit to Celsius its
    //Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
    //To Convert to Celsisus from Fahrenheit its
    //Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9

    loop {
        println!("Please input the temperature in Fahrenheit you would like to convert to Celsius.");
        
        //mutable variable to collect the user's input
        let mut temperature_fahrenheit = String::new();

        io::stdin()
            .read_line(&mut temperature_fahrenheit)
            .expect("Failed to read line");

        let temperature_fahrenheit:u32 = match temperature_fahrenheit.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered {temperature_fahrenheit}°F.");

        let temperature_fahrenheit = (temperature_fahrenheit-32) * 5/9;

        println!("That is about {temperature_fahrenheit}°C");
        println!("Thank you and have a good one.");
        break;
    }



}
