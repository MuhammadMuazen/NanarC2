use std::io::Write;
use colored::Colorize;

fn file_exist(file_path: &std::path::Path) -> bool {

    match file_path.exists() {

        true => return true,
        false => {

            println!("\n{0}{1}\n", "[!] Error: There is no file with name: ".red(), file_path.to_str().unwrap_or("<invalid UTF-8>").red());
            return false;
        }
    }
}

fn is_valid_json_data(json_data_str: &str) -> bool {

    match serde_json::from_str::<serde_json::Value>(json_data_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn pretty_json(json_data_str: &str) -> String {

    if is_valid_json_data(json_data_str) {

        let json_value: serde_json::Value = serde_json::from_str(json_data_str).unwrap();
        colored_json::to_colored_json_auto(&json_value).unwrap()
    
    } 
    // If there is something wrong with the file formation
    else {

        return "[!] Error: JSON file formation is not correct please recheck it!\n".red().to_string();
    }
}

pub fn print_clients(clients_file_str: &str) {

    let clients_file_path: &std::path::Path = std::path::Path::new(clients_file_str);

    if file_exist(clients_file_path) {

        println!("\n{0} {1}\n", "[i] Clients file path: ".blue(), clients_file_str.blue());

        let clients_file_content: Result<String, std::io::Error> = std::fs::read_to_string(clients_file_path);

        if clients_file_content.is_ok() {

            println!("{}", pretty_json(clients_file_content.unwrap().as_str()));
        
        } else if clients_file_content.is_err() {

            println!("{}", "[-] Error: Could not read the clients json file!".red());
        }

    }
}

pub fn remove_all_clients(clients_file_str: &str) {

    // Ask the user to confirm his choice to remove all the clients
    let mut user_input_choice: String = String::new();

    print!("{}", "[i] Are you sure you want to remove all the clients from the clients file (y/n): ".blue());
    
    std::io::stdout().flush().expect("[-] Error Failed to flush stdout");
    std::io::stdin().read_line(&mut user_input_choice).expect("[!] Error: Could not get the user choice");
    
    let user_input_choice: &str = user_input_choice.trim();

    match user_input_choice {

        "y" | "Y" => {

            let clients_file_path: &std::path::Path = std::path::Path::new(clients_file_str);

            if file_exist(clients_file_path) {

                println!("{}", "[i] Removing all the contents of the clients file...".blue());
                
                match std::fs::write(clients_file_path, "") {
                    
                    Ok(()) => println!("{}", "[+] Clients file contents have been removed!".green()),
                    
                    Err(error) => 
                        println!("{0}{1}", "[!] Error Could not remove the client file contents due to ".red(), error.to_string().red()) 
                }
            }
        }
        _ => {

            println!("\n{}\n", "[!] Clients file contents have not been removed!".red());
        } 

    }

}