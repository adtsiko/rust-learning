/*
 Guess a secret number between 1 and x, where x > 2
 1 - Print welcome info about the game
 2 - Enter outer loop (play again):
    1 - Prompt for max value or use default of 100 if empty
    2 - Generate secret_number between 1 and max value
    3 - Play the round (inner loop):
       1 - Prompt user for guess and collect input
       2 - Validate guess is an integer within 1 to max value
       3 - If guess == secret_number:
          1 - Print celebration with number of attempts
          2 - Break out of inner loop
       4 - If guess is incorrect:
          1 - If guess is lower than secret_number, print higher hint
          2 - If guess is higher than secret_number, print lower hint
       5 - Continue inner loop
    4 - Ask play again (Y/N)
       1 - If Y, start a new round from step 2
       2 - If anything else, print goodbye and exit outer loop
 */

use std::io;

use rand::RngExt;

fn main() {
    println!("Hello welcome to Number Guesser");
    println!("--------------------------------");

    let mut user_input: String = String::new();
    let mut max_value_option: Option<u32> = None;

    loop {
        println!("Enter max value or leave empty for 100 max (must be greater than 2)");
        let max_value = read_max_value(&mut max_value_option, &mut user_input);
        let secret_number: u32 = rand::rng().random_range(1..=max_value);

        play(max_value, &mut user_input, secret_number);

        if !ask_play_again(&mut user_input) {
            println!("Thanks for playing!");
            break
        }
    }

}
fn ask_play_again(input: &mut String) -> bool {

    println!("Do you want to play again! Y/N");
    input.clear();

    io::stdin().read_line(input).expect("Failed to read line");

    match input.trim() {
        "Y" |  "y" => true,
        _ => false
    }

}

fn read_max_value(max_value_option: &mut Option<u32>, mut user_input: &mut String) -> u32 {
    while max_value_option.is_none() {
        user_input.clear();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        *max_value_option = parse_max_value(&user_input);
    }

    max_value_option.take().unwrap()
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


fn play(max_value: u32, mut user_input: &mut String, secret_number: u32 ) {
    let mut attempts: u32 = 0;

    loop {
        attempts += 1;
        println!("Guess number from 1 to {}", max_value);
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let parsed_guess = match user_input.trim().parse() {
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

        let guess = match parsed_guess {
            Some(n) => n,
            None => continue
        };

        if guess == secret_number {
            print_celebration(guess, attempts);
            break;
        };
        println!("{} is an incorrect guess!", guess);
        println!("");

        let hint = guess_hint(guess, secret_number);

        println!("-------------------------{}-------------------------------", hint);
        println!("------------------------Try again!--------------------------");
    
    }
}

fn guess_hint(guess: u32, secret_number: u32)-> &'static str{
    if guess < secret_number {
        "Guess higher"
    }
    else {
        "Guess lower"
    }
}


fn print_celebration(correct_guess: u32, attempts: u32) -> () {
    println!("******************************************************************");
    println!("******************************************************************");
    println!("**********************Congratulations******************************");
    println!("**************You guessed the correct number**********************");
    println!("***************************{}! It took {} attempt(s)*************************************", correct_guess, attempts);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_max_value_accepted(){
        assert_eq!(parse_max_value("50"), Some(50));
    }

    #[test]
    fn max_value_must_be_greater_than_two(){
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