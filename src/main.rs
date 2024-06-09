// use std::collections::HashMap;
// use std::io::{self, Write};
// use rpassword::read_password;

// struct PasswordManager {
//     passwords: HashMap<String, String>,
// }

// impl PasswordManager {
//     fn new() -> PasswordManager {
//         PasswordManager {
//             passwords: HashMap::new(),
//         }
//     }

//     fn add_password(&mut self, name: &str, password: &str) {
//         self.passwords.insert(name.to_string(), password.to_string());
//     }

//     fn get_password(&self, name: &str) -> Option<&String> {
//         self.passwords.get(name)
//     }

//     fn delete_password(&mut self, name: &str) -> Option<String> {
//         self.passwords.remove(name)
//     }

//     fn list_passwords(&self) {
//         println!("Stored Passwords:");
//         for name in self.passwords.keys() {
//             println!("{}", name);
//         }
//     }

//     fn update_password(&mut self, name: &str, new_password: &str) -> bool {
//         if let Some(existing_password) = self.passwords.get_mut(name) {
//             *existing_password = new_password.to_string();
//             true
//         } else {
//             false
//         }
//     }
// }

// fn main() {
//     let mut manager = PasswordManager::new();

//     loop {
//         println!("Password Manager");
//         println!("1. Add Password");
//         println!("2. Retrieve Password");
//         println!("3. Delete Password");
//         println!("4. Update Password");
//         println!("5. List Passwords");
//         println!("6. Exit");
//         print!("Enter your choice: ");
//         io::stdout().flush().unwrap();

//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).expect("Failed to read line");

//         let choice = choice.trim();

//         match choice {
//             "1" => {
//                 print!("Enter name: ");
//                 io::stdout().flush().unwrap();
//                 let mut name = String::new();
//                 io::stdin().read_line(&mut name).expect("Failed to read line");

//                 print!("Enter password: ");
//                 io::stdout().flush().unwrap();
//                 let password = read_password().unwrap();

//                 manager.add_password(name.trim(), &password);
//                 println!("Password added successfully!");
//             }
//             "2" => {
//                 print!("Enter name: ");
//                 io::stdout().flush().unwrap();
//                 let mut name = String::new();
//                 io::stdin().read_line(&mut name).expect("Failed to read line");

//                 match manager.get_password(name.trim()) {
//                     Some(password) => println!("Password: {}", password),
//                     None => println!("No password found for {}", name.trim()),
//                 }
//             }
//             "3" => {
//                 print!("Enter name to delete: ");
//                 io::stdout().flush().unwrap();
//                 let mut name = String::new();
//                 io::stdin().read_line(&mut name).expect("Failed to read line");

//                 match manager.delete_password(name.trim()) {
//                     Some(_) => println!("Password deleted successfully!"),
//                     None => println!("No password found for {}", name.trim()),
//                 }
//             }
//             "4" => {
//                 print!("Enter name to update: ");
//                 io::stdout().flush().unwrap();
//                 let mut name = String::new();
//                 io::stdin().read_line(&mut name).expect("Failed to read line");

//                 print!("Enter new password: ");
//                 io::stdout().flush().unwrap();
//                 let new_password = read_password().unwrap();

//                 if manager.update_password(name.trim(), &new_password) {
//                     println!("Password updated successfully!");
//                 } else {
//                     println!("No password found for {}", name.trim());
//                 }
//             }
//             "5" => manager.list_passwords(),
//             "6" => break,
//             _ => println!("Invalid choice!"),
//         }
//     }
// }


use std::collections::HashMap;
use std::io::{self, Write};
use rpassword::read_password;

struct PasswordManager {
    passwords: HashMap<String, String>,
}

impl PasswordManager {
    fn new() -> PasswordManager {
        PasswordManager {
            passwords: HashMap::new(),
        }
    }

    fn add_password(&mut self, name: &str, password: &str) {
        self.passwords.insert(name.to_string(), password.to_string());
    }

    fn get_password(&self, name: &str) -> Option<&String> {
        self.passwords.get(name)
    }

    fn delete_password(&mut self, name: &str) -> Option<String> {
        self.passwords.remove(name)
    }

    fn list_passwords(&self) {
        println!("Stored Passwords:");
        for (name, _) in &self.passwords {
            println!("{}", name);
        }
    }

    fn update_password(&mut self, name: &str, new_password: &str) -> bool {
        if let Some(password) = self.passwords.get_mut(name) {
            *password = new_password.to_string();
            true
        } else {
            false
        }
    }

    fn show_partial_password(&self, name: &str) -> Option<&str> {
        self.passwords.get(name).map(|password| &password[..3])
    }
}

fn main() {
    let mut manager = PasswordManager::new();

    loop {
        println!("Password Manager");
        println!("1. Add Password");
        println!("2. Retrieve Password");
        println!("3. Delete Password");
        println!("4. List Passwords");
        println!("5. Update Password");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                print!("Enter password: ");
                io::stdout().flush().unwrap();
                let password = read_password().unwrap();

                manager.add_password(name.trim(), &password);
                println!("Password added successfully!");
            }
            "2" => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                match manager.get_password(name.trim()) {
                    Some(password) => println!("Password: {}", password),
                    None => println!("No password found for {}", name.trim()),
                }
            }
            "3" => {
                print!("Enter name to delete: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                match manager.delete_password(name.trim()) {
                    Some(_) => println!("Password deleted successfully!"),
                    None => println!("No password found for {}", name.trim()),
                }
            }
            "4" => manager.list_passwords(),
            "5" => {
                print!("Enter name to update: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                if let Some(partial_password) = manager.show_partial_password(name.trim()) {
                    println!("Current password starts with: {}", partial_password);
                    print!("Enter new password: ");
                    io::stdout().flush().unwrap();
                    let new_password = read_password().unwrap();

                    if manager.update_password(name.trim(), &new_password) {
                        println!("Password updated successfully!");
                    } else {
                        println!("No password found for {}", name.trim());
                    }
                } else {
                    println!("No password found for {}", name.trim());
                }
            }
            "6" => break,
            _ => println!("Invalid choice!"),
        }
    }
}

