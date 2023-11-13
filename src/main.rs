mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
░░░░░░░░░░░░░░░░░░░░
░▄▀▄▀▀▀▀▄▀▄░░░░░░░░░
░█░░░░░░░░▀▄░░░░░░▄░
█░░▀░░▀░░░░░▀▄▄░░█░█
█░▄░█▀░▄░░░░░░░▀▀░░█
█░░▀▀▀▀░░░░░░░░░░░░█
█░░░░░░░░░░░░░░░░░░█
█░░░░░░░░░░░░░░░░░░█
░█░░▄▄░░▄▄▄▄░░▄▄░░█░
░█░▄▀█░▄▀░░█░▄▀█░▄▀░
░░▀░░░▀░░░░░▀░░░▀░░░
    "#;
    println!("{ascii}");
    loop {
        println!("Password manager menu: ");
        println!("1. Add entry");
        println!("2. List entries");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {

            },
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
                        - Username: {}
                        - Password: {}",
                        item.service, item.username, item.password
                    );
                } 
            },
            "3" => {

            },
            "4" => {

            },
        }

    }
}
