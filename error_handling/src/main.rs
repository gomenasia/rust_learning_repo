use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn"); cette macro fait apniquer le programme
    let greeting_file_result = File::open("hello.txt"); //cette fonction renvoie un Result<T, E>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    //mm chose qu'au dessu en plus propre
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });
}

use std::io::{self, Read};

// propagation d'erreur, si on veut laisser le choix a celui qui appele read_username_from_file
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//fait la meme chose qu'au dessus avec le ?
fn read_username_from_file_interogation() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// toujour la mm chose

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// ? est utilisable uniquement si la fonction renvoie Result ou Option

// on peut écrire RUST_BACKTRACE=1 cargo run dans le terminal pour avoir une backtrace des appele qui mene au crash


//créer un type personaliser pour s'assurer des parametre d'une fonction ex: fn guess_number(Guess) -> bool
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}