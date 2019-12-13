// program to convert english text to to_morse code & vice versa

use std::collections::HashMap;
use std::io;

fn main() {
    println!("Press e to convert English to Morse or press m to convert Morse to English {}", " ");
   
    let mut user_choice = String::new();

    match io::stdin().read_line(&mut user_choice) { // calls read_line() method and saves input
        
        Ok(_) => {
            if user_choice.trim() == "e" || user_choice.trim() == "E" {
                eng_to_morse();
            } else if user_choice.trim() == "m" || user_choice.trim() ==  "M" {
                morse_to_eng();
            }
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}


fn eng_to_morse() {
    let mut to_morse: HashMap<char, &&str> = HashMap::new(); // build hashmap with key-value pairs

    to_morse.insert(' ', &"  ");
    to_morse.insert('A', &".-");
    to_morse.insert('B', &"-...");
    to_morse.insert('C', &"-.-");
    to_morse.insert('D', &"-..");
    to_morse.insert('E', &".");
    to_morse.insert('F', &"..-.");
    to_morse.insert('G', &"--.");
    to_morse.insert('H', &"....");
    to_morse.insert('I', &"..");
    to_morse.insert('J', &".---");
    to_morse.insert('K', &"-.");
    to_morse.insert('L', &".-..");
    to_morse.insert('M', &"--");
    to_morse.insert('N', &"-.");
    to_morse.insert('O', &"---");
    to_morse.insert('P', &".--.");
    to_morse.insert('Q', &"--.-");
    to_morse.insert('R', &".-.");
    to_morse.insert('S', &"...");
    to_morse.insert('T', &"-");
    to_morse.insert('U', &"..-");
    to_morse.insert('V', &"...-");
    to_morse.insert('W', &".--");
    to_morse.insert('X', &"-..-");
    to_morse.insert('Y', &"-.--");
    to_morse.insert('Z', &"--..");

    println!("Enter something you'd like to be converted to morse code:");

    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        
        Ok(_) => {

            println!("\nGot it! You said: {}", user_input);

            let new_input = user_input.to_uppercase(); 
            let input_chars: Vec<char> = new_input.chars().collect(); // breaks input down to chars           

            println!("Here's your statement in morse: {}", " ");
            for i in input_chars { // compares user input letter by letter to find matching value
                for (letter, code) in &to_morse {
                    if i == *letter {
                        // println!("Letter {} is {} in morse", letter, code);
                        print!("{} {}", code, " ");
                    }
                }
            }
            println!("\n{}", " ");
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}


fn morse_to_eng() {
    let mut to_english: HashMap<&str, char> = HashMap::new(); // build hashmap with key-value pairs

    to_english.insert(&"/", ' ');
    to_english.insert(&"   ", ' ');
    to_english.insert(&".-", 'A');
    to_english.insert(&"-...", 'B');
    to_english.insert(&"-.-", 'C');
    to_english.insert(&"-..", 'D');
    to_english.insert(&".", 'E');
    to_english.insert(&"..-.", 'F');
    to_english.insert(&"--.", 'G');
    to_english.insert(&"....", 'H');
    to_english.insert(&"..", 'I');
    to_english.insert(&".---", 'J');
    to_english.insert(&"-.", 'K');
    to_english.insert(&".-..", 'L');
    to_english.insert(&"--", 'M');
    to_english.insert(&"-.", 'N');
    to_english.insert(&"---", 'O');
    to_english.insert(&".--.", 'P');
    to_english.insert(&"--.-", 'Q');
    to_english.insert(&".-.", 'R');
    to_english.insert(&"...", 'S');
    to_english.insert(&"-", 'T');
    to_english.insert(&"..-", 'U');
    to_english.insert(&"...-", 'V');
    to_english.insert(&".--", 'W');
    to_english.insert(&"-..-", 'X');
    to_english.insert(&"-.--", 'Y');
    to_english.insert(&"--..", 'Z');

    println!("Enter something you'd like to be converted to English, separating words with a /:");

    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        
        Ok(_) => {

            println!("\nAlright. You said: {}", user_input);

            let new_input =  user_input.split_whitespace();
            let vec: Vec<&str> = new_input.collect();
        
            println!("Here's your statement in English: {}", " ");
            for i in vec {
                for (code, letter) in &to_english {
                    if i == *code {
                        print!("{}", letter);
                    }
                }
            }  
            println!("\n{}", " ");
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}