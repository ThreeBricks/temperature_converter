use std::io;
//To accept user input

fn main() {
    //To convert from Fahrenheit to Celsius its
    //Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
    //To Convert to Celsisus from Fahrenheit its
    //Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9

    loop {

        let mut unit = String::new();

        println!("Do you want to convert from Fahrenheit (F) or Celsius(C)?");

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit:char = match unit.trim().parse(){
            Ok(letter) => letter,
            Err(_) => continue,
        };

        let mut temp = String::new();

        println!("Enter the temperature you want to convert to {unit}");

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to real line");

        let temp:u32 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You entered {temp}°{unit}.");

        if unit=='F' {
            let temp = convert_to_celsius(temp);
            println!("That is about {temp}°C");
        } else {
            let temp = convert_to_fahrenheit(temp);
            println!("That is about {temp}°F");
        }

        println!("Thank you and have a good one.");
        break;
    }
}


fn convert_to_celsius(temp: u32) -> u32{
    (temp-32) * 5/9
}

fn convert_to_fahrenheit(temp: u32) -> u32{
    (temp*9/5) + 32
}