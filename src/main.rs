/*
Goal for this program: To store the names of all the artists i follow and also link their twitter account and their pixiv accounts (if they have one)
I also want to be able to sort them by keywords (Chinese artist, Arknights, Kami Eshi, etc)
Probably not possible but id like to eventually add a preview artwork for each artist (makes it easier to remember who they are)
Maybe i could also use this to associate folders to the names and be able to sort through artworks in my computer?

Notes for Rust:
how it works in python: make menus, ask for input number, then depending on that number, do stuff. need to check this number and decide what to do after

notes: use structs for replacing the Artist class (done)
       use match for making a menu? (done)
*/
use std::{io, process::exit};
struct Artist {
    name: String,
    twitter: String,
    pixiv: String
}

fn open_menu() -> u32 {
    loop {
        println!("Artist list options:");
        println!("1) Show list of artists");
        println!("2) Remove an artist/classification");
        println!("3) Add a new artist");
        println!("4) Edit an artist");
        println!("5) Quit the program");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        
        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your option is invalid. Please choose one of the 5 options");
                continue;
            }
         }; return answer;
    }
}

// now, match the answer given and then make decision based on that value, maybe return that value 
fn main() {
    let answer = 
    open_menu();
    match answer {
        1 => show_list(),
        2 => remove_artist(), // note: also needs to ask if you want to remove artist or a classif here
        3 => add_artist(),
        4 => edit_artist(),
        5 => exit(1),
        _epic => ()
    };
  /* 
    let mut artist_list = Vec::new();
    artist_list.push("epic");
    println!("{:#?}", artist_list);
  */
}

fn show_list() {e
    let artist_vec = 1;
    println!("{}", artist_vec);
}

fn remove_artist() {
    
}

fn add_artist() {
    let mut name = String::new();
    let mut twitter = String::new();
    let mut pixiv = String::new();

    println!("Name of the artist:");
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

    println!("The artists twitter:");
    io::stdin()
    .read_line(&mut twitter)
    .expect("Failed to read line");

    println!("The artists pixiv:");
    io::stdin()
    .read_line(&mut pixiv)
    .expect("Failed to read line");
    
    let artist = Artist {
            name, twitter, pixiv
        };
}

fn edit_artist() {
    
}