use std::io;

enum PowerOption {
    OFF,
    SLEEP,
    REBOOT,
    SHUTDOWN,
    HIBERNATE,
}

fn main() {
    let mut command = String::new();

    println!("\n\n..............Commands.............");
    println!("1       - Power off the system");
    println!("2       - Enter sleep mode");
    println!("3       - Reboot the system");
    println!("4       - Shutdown the system");
    println!("5 - Hibernate the system");
    println!("..................................\n\n");

    loop {
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let input: u8 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number corresponding to a command.");
                command.clear();
                continue;
            }
        };

        let parsed_input = match input {
            0 => PowerOption::OFF,
            1 => PowerOption::SLEEP,
            2 => PowerOption::REBOOT,
            3 => PowerOption::SHUTDOWN,
            4 => PowerOption::HIBERNATE,
            _ => {
                println!("Invalid option selected. Please try again.");
                continue;
            }
        };

        match parsed_input {
            PowerOption::OFF => {
                println!(".............................");
                println!("Powering off the system...");
                println!(".............................");
            }
            PowerOption::SLEEP => {
                println!(".............................");
                println!("Entering sleep mode...");
                println!(".............................");
            }
            PowerOption::REBOOT => {
                println!(".............................");
                println!("Rebooting the system...");
                println!(".............................");
            }
            PowerOption::SHUTDOWN => {
                println!(".............................");
                println!("Shutting down the system...");
                println!(".............................");
            }
            PowerOption::HIBERNATE => {
                println!(".............................");
                println!("Hibernating the system...");
                println!(".............................");
            }
        }

        command.clear();
    }
}



// CLASS SOLUTION


// use std::io;

// enum PowerOptions {
//     Sleep,
//     Reboot,
//     Shutdown,
//     Hibernate,
// }

// impl PowerOptions {
//     fn power_action(state: &str) -> Option<PowerOptions> {
//         let state = state.trim().to_lowercase();
//         match state.as_str() {
//             "sleep" => Some(PowerOptions::Sleep),
//             "reboot" => Some(PowerOptions::Reboot),
//             "shutdown" => Some(PowerOptions::Shutdown),
//             "hibernate" => Some(PowerOptions::Hibernate),
//             _ => None,
//         }
//     }
// }

// fn print_state(action: PowerOptions) {
//     match action {
//         PowerOptions::Sleep => println!("Sleeping"),
//         PowerOptions::Reboot => println!("Rebooting"),
//         PowerOptions::Shutdown => println!("Shutting Down"),
//         PowerOptions::Hibernate => println!("Hibernating"),
//     }
// }

// fn main() {
//     let mut input = String::new();
//     println!("Enter power option");

//     let user_input = io::stdin().read_line(&mut input);

//     if user_input.is_ok() {
//         match PowerOptions::power_action(&input) {
//             Some(result) => print_state(result),
//             None => println!("Invalid power state"),
//         }
//     }
// }

