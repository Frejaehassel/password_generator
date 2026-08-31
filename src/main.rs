// Comment legend (for "colorful comments"): ~ = Imports, * = Global variables, ? = Functions, & = Variables, ! = Combination of lines, ^ = Everything else

//~ Import Cargo and Native Packages
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};

//* Declare our characters for password generation into numbers, lower case, upper case and symbols
const NUMBERS: [char; 10] = [
    '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9'
    ];
const LOWER_CASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 
    'o', 'p', 'q', 'r', 's','t', 'u', 
    'v', 'w', 'x', 'y', 'z'
];
const UPPER_CASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G',
    'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z'
];
const SYMBOLS: [char; 29] = [
    '!', '?', '.', ',', ':', ';', '"', '@', '#', '*',
    '\\', '$', '%', '^', '&', '\'', '+','-',
    '=', '/', '|', '~', '`', '(', ')', '{', '}',
    '[', ']'
];

//* Declare a global random number generator
static RNG: OnceLock<Mutex<StdRng>> = OnceLock::new();

//? Creates a global Mutex when RNG is called
fn global_rng() -> &'static Mutex<StdRng> {
    RNG.get_or_init(|| Mutex::new(StdRng::from_entropy()))
}

//? Runs the app
fn main() {
    println!("What length of password would you like to generate? ");

    //! Takes user input and checks it's a number 8 <= x >= 32
    let length: usize = loop {
        print!("Enter a length from 8 to 32: ");
        io::stdout().flush().unwrap();

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        match input_string.trim().parse::<usize>() {
            Ok(num) if (8..=32).contains(&num) => break num,
            _ => {
                println!("Invalid input. Enter a password length from 8 to 32 symbols.");
            }
        }
    };

    //! Calls for the generation of password
    let password = generate_password(length);
    println!("Password: {}", password);

    //! Copies password to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.clone()).unwrap();
    let clipboard_contents = ctx.get_contents().unwrap();
    assert_eq!(clipboard_contents, password);
    println!("Copied password to clipboard!");
}

//? Generates a random password with at least one character from each group
fn generate_password(length: usize) -> String {
    //& Access global random number generator
    let mut rng = global_rng().lock().unwrap();

    //& Creates an empty vector to store the password characters
    let mut password = Vec::with_capacity(length);

    //^ Add one guaranteed character from each char group
    password.push(NUMBERS[rng.gen_range(0..NUMBERS.len())]);
    password.push(LOWER_CASE_LETTERS[rng.gen_range(0..LOWER_CASE_LETTERS.len())]);
    password.push(UPPER_CASE_LETTERS[rng.gen_range(0..UPPER_CASE_LETTERS.len())]);
    password.push(SYMBOLS[rng.gen_range(0..SYMBOLS.len())]);

    //! Add the remaining characters.
    //^ Each character has an equal chance of coming from any group.

    for _ in 0..length - 4 {
        match rng.gen_range(0..4) {
            //^ Add a random number
            0 => password.push(NUMBERS[rng.gen_range(0..NUMBERS.len())]),

            //^ Add a random lowercase letter
            1 => password.push(LOWER_CASE_LETTERS[rng.gen_range(0..LOWER_CASE_LETTERS.len())]),

            //^ Add a random uppercase letter
            2 => password.push(UPPER_CASE_LETTERS[rng.gen_range(0..UPPER_CASE_LETTERS.len())]),

            //^ Add a random symbol
            _ => password.push(SYMBOLS[rng.gen_range(0..SYMBOLS.len())]),
        }
    }

    //^ Randomizes the order of all characters so the guaranteed characters aren't always at the beginning
    password.shuffle(&mut *rng);

    //^ Converts the vector of characters into a String
    password.into_iter().collect()
}