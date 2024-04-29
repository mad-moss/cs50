use std::io;

//equivalent of cs50.h's get_int(), etc. but generic over any type which implements std::str::FromStr
pub fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {

        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: T = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        return input;
    }
}