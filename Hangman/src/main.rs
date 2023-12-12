use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io;
use std::io::Write;

struct Hangman {
    secret_word: String,
    guessed_letters: HashSet<char>,
    max_attempts: usize,
    attempts: usize,
}

impl Hangman {
    // Constructor to initialize the Hangman struct
    fn new(word: &str, max_attempts: usize) -> Hangman {
        Hangman {
            secret_word: word.to_lowercase(),
            guessed_letters: HashSet::new(),
            max_attempts,
            attempts: 0,
        }
    }

    // Method to display the current state of the word being guessed
    fn display_word(&self) -> String {
        let mut display = String::new();
        for c in self.secret_word.chars() {
            if self.guessed_letters.contains(&c) {
                display.push(c);
            } else {
                display.push('_');
            }
            display.push(' ');
        }
        display
    }

    // Method to check if the game is won
    fn is_game_won(&self) -> bool {
        self.secret_word.chars().all(|c| self.guessed_letters.contains(&c))
    }

    // Method to process a player's guess
    fn process_guess(&mut self, guess: char) {
        if !self.guessed_letters.contains(&guess) {
            self.guessed_letters.insert(guess);
            if !self.secret_word.contains(guess) {
                self.attempts += 1;
            }
        }
    }

    // Method to check if the game is over
    fn is_game_over(&self) -> bool {
        self.attempts >= self.max_attempts || self.is_game_won()
    }

    // Method to draw the hangman (full hangman when the player loses)
    fn draw_hangman(&self) {
        let hangman_full = "   _______\n   |/      |\n   |      (_)\n   |      \\|/\n   |       |\n   |      / \\\n___|___\n";
        println!("{}", hangman_full);
    }
}

fn main() {
    println!("Welcome to Hangman!");

    // Define word options for the game
    let word_options = vec!["RUST", "CLASSES", "STUDENT", "GAME", "PROGRAMMING"];
    let mut rng = rand::thread_rng();
    let secret_word = word_options.choose(&mut rng).unwrap(); // Randomly choose a word

    let max_attempts = 6;
    let mut game = Hangman::new(secret_word, max_attempts); // Initialize the game with the chosen word

    // Game loop until game over
    while !game.is_game_over() {
        println!("Current word: {}", game.display_word());
        println!("Attempts remaining: {}", game.max_attempts - game.attempts);
        println!("Enter a letter guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: char = match guess.trim().chars().next() {
            Some(c) => c,
            None => continue,
        };

        game.process_guess(guess); // Process the player's guess
    }

    // Display the result of the game and draw the hangman if the player loses
    if game.is_game_won() {
        println!("Congratulations! You've guessed the word: {}", game.secret_word);
    } else {
        println!("Sorry, you ran out of attempts. The word was: {}", game.secret_word);
        game.draw_hangman(); // Draw the full hangman when the player loses
    }
}
