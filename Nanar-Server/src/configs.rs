use colored::Colorize;

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

    if clients_file_path.exists() {

        println!("\n{0} {1}\n", "[i] Clients file path: ".blue(), clients_file_str.blue());

        let clients_file_content: Result<String, std::io::Error> = std::fs::read_to_string(clients_file_path);

        if clients_file_content.is_ok() {

            println!("{}", pretty_json(clients_file_content.unwrap().as_str()));
        
        } else if clients_file_content.is_err() {

            println!("{}", "[-] Error: Could not read the clients json file!".red());
        }
    } else {

        println!("\n{}\n", "[!] Error: Clients json file does not exist in this path please recheck the server configurations!".red());
    }
}