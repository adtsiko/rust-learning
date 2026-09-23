/*
 To guess a number between positive numbers between 0 to x where x > 2
 1 - Print info about game
 2 - Collect the max value from user or if empty use default value of 100
 3 - Use the random number generator with max value as boundary to generate a random number
 4 - Enter into while loop: condition while true
    1 - Prompt user for guess and the collect guess
    2 - check if guess == randomInt
        1 - If match print celebrations info with number of attempts
        2 - break out of the while
    3 - Due to guess being incorrect print below condition
        1 - If guess is higher than randomInt print lower
        2 - If guess is lower than randomInt print higher
    4 - continue while loop
 */

use std::io;

use rand::RngExt;

fn main() {
    println!("Hello welcome to Number Guesser");
    println!("--------------------------------");
    println!("Enter max value or leave empty for 100 max (must be greater than 2)");

    let mut user_input: String = String::new();
    let mut max_value: Option<u32> = None;


    let max_value = get_max_value(&mut max_value, &mut user_input);
    
    let random_int: u32 = rand::rng().random_range(1..=max_value);

    take_a_guess(max_value, &mut user_input, random_int);
}

fn get_max_value(max_value: &mut Option<u32>, mut user_input: &mut String) -> u32 {
    while max_value.is_none() {
        user_input.clear();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        *max_value = parse_max_value(&user_input);
    }

    max_value.take().unwrap()
}
fn parse_max_value(input: &str) -> Option<u32> {
    match input.trim().parse() {
        Ok(n) if n > 2 => Some(n),
        Ok(_) => {
            println!("Must be greater than 2");
            None
        },
        Err(_) if input.trim().is_empty() => Some(100),
        Err(_) => {
            println!("Input an integer");
            None
        }
    } 
}

fn take_a_guess(max_value: u32, mut user_input: &mut String, random_int: u32 ) {

    loop {
        println!("Guess number from 1 to {}", max_value);
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let guess_value = match user_input.trim().parse() {
            Ok(n) if n <= max_value && n > 0 => Some(n),
            Ok(_) => {
                println!("Number not within 1 to {} range", &max_value);
                None
            }
            Err(_) => {
                println!("Input an integer");
                None
            }
        };

        let guess = match guess_value {
            Some(n) => n,
            None => continue
        };

        if guess == random_int {
            celebrate_msg(guess);
            break;
        };
        println!("{} is an incorrect guess!", guess);
        println!("");

        let hint = guess_hint(guess, random_int);

        println!("-------------------------{}-------------------------------", hint);
        println!("------------------------Try again!--------------------------");
    
    }
}

fn guess_hint(guess: u32, random_int: u32)-> &'static str{
    if guess < random_int {
        "Guess higher"
    }
    else {
        "Guess lower"
    }
}


fn celebrate_msg(actual_value: u32) -> () {
    println!("******************************************************************");
    println!("******************************************************************");
    println!("**********************Congratulations******************************");
    println!("**************You guessed the correct number**********************");
    println!("***************************{}!*************************************", actual_value);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_max_value_accepted(){
        assert_eq!(parse_max_value("50"), Some(50));
    }

    #[test]
    fn max_value_must_be_greater_than_w(){
        assert_eq!(parse_max_value("2"), None);
    }

    #[test]
    fn invalid_input_returns_none(){
        assert_eq!(parse_max_value("abs"), None);
    }

    #[test]
    fn hint_when_too_low(){
        assert_eq!(guess_hint(10, 50), "Guess higher")
    }

    #[test]
    fn hint_when_too_high(){
        assert_eq!(guess_hint(300, 101), "Guess lower")
    }

}