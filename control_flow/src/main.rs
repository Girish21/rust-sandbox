use std::io;

fn main() {
    loop {
        let mut convert_option = String::new();

        println!("Enter the conversion type (number)");
        println!("1) Celsius to Fahrenheit\n2) Fahrenheit to Celsius\n3) Exit");
        io::stdin()
            .read_line(&mut convert_option)
            .expect("Enter a valid option");

        let selected_option: u32 = convert_option.trim().parse().expect("Enter a number");

        if selected_option == 1 {
            let mut celsius = String::new();

            println!("Enter temperature in Celsius: ");
            io::stdin()
                .read_line(&mut celsius)
                .expect("Enter a Celcius value");

            let celsius: f32 = celsius
                .trim()
                .parse()
                .expect("Enter a valid Celcius degree value");
            let fahrenheit = (celsius * (9.0 / 5.0)) + 32.0;

            println!(
                "{} degree Celsius is {} degree Fahrenheit",
                celsius, fahrenheit
            );
        } else if selected_option == 2 {
            let mut fahrenheit = String::new();

            println!("Enter temperature in Fahrenheit: ");
            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Enter a Fahrenheit value");

            let fahrenheit: f32 = fahrenheit
                .trim()
                .parse()
                .expect("Enter a valid Fahrenheit degree value");
            let celsius = (fahrenheit - 32.0) * (5.0 / 9.0);

            println!(
                "{} degree Fahrenheit is {} degree Celsius",
                fahrenheit, celsius
            );
        } else {
            break;
        }
    }
}
