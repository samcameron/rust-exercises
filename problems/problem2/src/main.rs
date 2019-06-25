use std::io;
use problem2::sum_one_to_n;

fn main() {

    loop {
        println!("Give me a number, any number!");

        // this variable can be used to store the string input from the user
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read number");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = sum_one_to_n(number);

        println!("The sum of 1 to {} is: {}", number, result);

        // this is here to prevent an infinite loop with the sample code above,
        // but can be removed once your program is working
        break;
    }
}
