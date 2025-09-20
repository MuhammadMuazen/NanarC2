/*
    config.rs: Made by Muhammad Muzen

    This file holds all the functions and values needed to minipulate the server files and configurations.
    
*/

use std::io::Write;
use colored::Colorize;

pub const NANARC2_DIRECTORY_NAME: &str = "nanarc2";

pub fn get_nanarc2_dir_path() -> String {
    
    if cfg!(target_os = "windows") {

        format!(r"C:\Users\{}\Documents\{}", get_username(), NANARC2_DIRECTORY_NAME)

    } else if cfg!(target_os = "linux") {
        
        format!("/usr/share/{}", NANARC2_DIRECTORY_NAME)
    
    } else {

        panic!("{}", "Error: Unsupported OS".red())
    }
}

pub fn get_username() -> String {
    
    if cfg!(target_os = "windows") {
        
        std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
    
    } else if cfg!(target_os = "linux") {
        
        std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    
    } else {

        panic!("{}", "Error: Unsupported OS".red())
    }
}

pub fn get_default_config_path() -> String {
    
    if cfg!(target_os = "windows") {
        
        format!(r"{}\nanarc2_config.json", get_nanarc2_dir_path())
    
    } else if cfg!(target_os = "linux") {
        
        format!(r"{}/nanarc2_config.json", get_nanarc2_dir_path())
    
    } else {
        
        panic!("{}", "Error: Unsupported OS".red())
    }
}

pub fn get_default_clients_path() -> String {
    
    if cfg!(target_os = "windows") {
        
        format!(r"{}\clients.json", get_nanarc2_dir_path())
    
    } else if cfg!(target_os = "linux") {
        
        format!(r"{}/clients.json", get_nanarc2_dir_path())
    
    } else {
        
        panic!("{}", "Error: Unsupported OS".red())
    }
}

pub fn default_config_file_content() -> serde_json::Map<String, serde_json::Value> {

    let mut def_conf_json_obj: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

    // Add keys and values to the json configuration file
    def_conf_json_obj.insert("clients_file_path".to_string(), serde_json::Value::String(get_default_clients_path()));
    def_conf_json_obj.insert("listeners".to_string(), serde_json::Value::Array(vec![]));

    def_conf_json_obj
}

fn file_exists(file_path: &std::path::Path) -> bool {

    match file_path.exists() {

        true => return true,
        false => {

            println!("{0}{1}", "[-] Error: There is no file with name: ".red(), file_path.to_str().unwrap_or("<invalid UTF-8>").red());
            return false;
        }
    }
}

fn directory_exists(dir_path: &std::path::Path) -> bool {

    match dir_path.exists() && dir_path.is_dir() {

        true => return true,
        false => {

            println!("\n{0}", format!("[-] Error: Directory {} does not exists!", dir_path.to_str().unwrap_or("<invalid UTF-8>")).red());
            return false;
        }
    }
}

fn write_to_file(mut file_path: std::fs::File, content_to_write: &str) -> bool {

    match file_path.write_all(content_to_write.as_bytes()) {

        Ok(_) => {
            
            match file_path.flush() {
                
                Ok(_) => true,
                Err(_) => {

                    println!("{}", format!("[-] Error: Could not flush the file after writing to it!").red());
                    false
                }
            }
        },
        Err(_) => {

            println!("{}", format!("[-] Error: Could not write to file {:?}", file_path).red());
            false
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

fn get_json_key_value<'a>(json_data: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    
    if let serde_json::Value::Object(obj) = json_data {

        obj.get(key)
    
    } else {
        
        None
    }
}

pub fn print_clients() {

    match file_exists(std::path::Path::new(&get_default_config_path())) {

        true => {

            let clients_file_str: String = get_json_key_value(
            &serde_json::from_str(&std::fs::read_to_string(&get_default_config_path()).unwrap()).unwrap(),
            "clients_file_path").unwrap().as_str().unwrap().to_string();
            
            if file_exists(&std::path::Path::new(&clients_file_str)) {
            
                println!("\n{0} {1}\n", "[i] Clients file path: ".blue(), clients_file_str.blue());
            
                let clients_file_content: Result<String, std::io::Error> = std::fs::read_to_string(&std::path::Path::new(&clients_file_str));
            
                if clients_file_content.is_ok() {
                
                    println!("{}", pretty_json(clients_file_content.unwrap().as_str()));
                
                } else if clients_file_content.is_err() {
                
                    println!("{}", "[-] Error: Could not read the clients json file!".red());
                }
            } else {

                println!("{}", format!("[-] Error: Clients JSON file does not exists!").red());
                println!("{}", format!("[i] Run the server at least once to generate it!").blue());
                std::process::exit(-1);
            } 
        
        }, false => {

            println!("{}", format!("[-] Error: Could not find the configuration file that have the key:value of the clients file path!").red());
            println!("{}", format!("[i] You can run the server once to generate the default configuration file and the default clients file").blue());
            std::process::exit(-1);
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

            if file_exists(clients_file_path) {

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

pub fn check_config_file() {

    let nanarc2_dir_path: String = get_nanarc2_dir_path();
    let default_config_path: String = get_default_config_path();
    let default_clients_path: String = get_default_clients_path();

    // 1. Check for default configuration directory existance
    match directory_exists(std::path::Path::new(&nanarc2_dir_path)) {

        true => {

            println!("{}", format!("[i] Found server configuration directory at: {}", nanarc2_dir_path).blue());
        }
        false => {

            println!("{}", format!("[i] Creating default server configuration directory at: {}", nanarc2_dir_path).blue());

            // Creating the nanarc2 directory.
            match std::fs::create_dir(std::path::Path::new(&get_nanarc2_dir_path())) {
                
                Ok(_) => println!("{}", format!("[+] Created the default configuration directory at: {}", nanarc2_dir_path).green()),
                Err(_) => {

                    println!("{}", format!(
                        "[-] Error Could not create the default configuration directory at: {}", nanarc2_dir_path).red());
                    println!("{}", "[i] Creating it manually might help!".blue());

                    std::process::exit(-1);
                }
            }
        } 
    }

    // 2. Check for the default server json configuration file existance 
    match file_exists(std::path::Path::new(&default_config_path)) {

        true => {

            // 2.1. Check if the json file format is ok
            if is_valid_json_data(std::fs::read_to_string(&default_config_path).unwrap().as_str()) {

                println!("{}", format!("[i] Found configuration file in: {}", default_config_path).blue());
                // 2.2. TODO check for the keys in the configuration file
            
            } else {

                println!("{}", format!("[-] Error: Unvalid json format in the configuration file!").red());
                std::process::exit(-1);
            } 

        },
        // 2.3. If the default configuration file does not exist
        false => {

            println!("{}", format!("[-] Error: Configuration file could not be found in: {}", default_config_path).red());
            println!("{}", format!("[i] Creating new configuration file in path: {}", default_config_path).blue());

            let new_config_file: Result<std::fs::File, std::io::Error> = std::fs::File::create(&default_config_path);

            match new_config_file {
                // 2.3.1. Create the default configuration file
                Ok(file) => {
                    
                    //2.3.2. Write the default configurations to the file
                    match write_to_file(file, serde_json::to_string_pretty(&default_config_file_content()).unwrap().as_str()) {
                        
                        true => {
                            
                            println!("{}", format!("[+] Finished writing the default configuration file content successfully!").green());
                        },
                        false => {
                                
                            println!("{}", format!("[-] Error: Could not write the default configuration to the config file").red());
                            std::process::exit(-1);
                        }
                    }

                }, 
                Err(_) => {
                    
                    println!("{}", format!("[-] Error: Could not create server configuration file in: {}", default_config_path));
                    std::process::exit(-1);
                }
            }
        }
    };

    // 2.4. Checking the server clients file path first in the server configs.json file
    let clients_file_path: String = get_json_key_value(
    &serde_json::from_str(&std::fs::read_to_string(&default_config_path).unwrap()).unwrap(),
    "clients_file_path").unwrap().as_str().unwrap().to_string();

    if file_exists(&std::path::Path::new(&clients_file_path)) && clients_file_path != default_clients_path {

        if is_valid_json_data(std::fs::read_to_string(&clients_file_path).unwrap().as_str()) {

            println!("{}", format!("[i] Found the clients file specified in the config file: in {}", clients_file_path).blue());
        
        } else {

            println!("{}", format!("[-] Error: Invalid JSON format in the specified clients file: {}", clients_file_path).red());
            std::process::exit(-1);
        } 
    
    } else {
        // 2.4.1. Check for the default clients json file
        match file_exists(std::path::Path::new(&default_clients_path)) {

            true => {
                // 2.4.2. Check if the json format in the clients json file is ok
                if is_valid_json_data(std::fs::read_to_string(&default_clients_path).unwrap().as_str()) {

                    println!("{}", format!("[i] Found clients json file in: {}", default_clients_path).blue());
                
                } else {

                    println!("{}", format!("[-] Error: Unvalid json format in the clients json file!").red());
                    std::process::exit(-1);
                } 
            // 2.4.3. Create the default clinets json file if the it does not exist
            } false => {

                println!("{}", format!("[-] Error: clients json file could not be found in: {}", default_clients_path).red());
                println!("{}", format!("[i] Creating new clients.json file in path: {}", default_clients_path).blue());

                let new_clients_file: Result<std::fs::File, std::io::Error> = std::fs::File::create(&default_clients_path);

                match new_clients_file {
                    // 2.4.4. Create the default clients.json file
                    Ok(file) => {
                        //2.4.5. Write the default clients.json content to the default clients.json file
                        match write_to_file(file, serde_json::to_string_pretty("[]").unwrap().as_str()) {

                            true => {

                                println!("{}", format!("[+] Finished writing the default clients.json file content successfully!").green());
                            },
                            false => {

                                println!("{}", format!("[-] Error: Could not write the default clients.json content!").red());
                                std::process::exit(-1);
                            }
                        }
                    },
                    Err(_) => {

                        println!("{}", format!("[-] Error: Could not create default clients.json file in: {}", default_clients_path));
                        std::process::exit(-1);
                    }
                }
            }
        }
}
}