use std::io::Write;
use colored::Colorize;

pub fn server_shell() {

    // TODO Check the config file.
    // TODO Check the clients.json file.

    loop {

        print!("{}> ", "NanarC2".yellow());
        std::io::stdout().flush().expect("[-] Error Failed to flush stdout");
        
        // Get the user input
        let mut user_shell_input: String = String::new();

        std::io::stdin().read_line(&mut user_shell_input).expect("[!] Error: Failed to read the user input!");

        // Remove the enter and the spaces from the user input
        user_shell_input = user_shell_input.trim().to_string();

        // Split user input into Vector
        let shell_input_vec_string: Vec<String> = user_shell_input.split_whitespace().map(|s: &str| s.to_string()).collect();

        // Check if the user press nothing
        if shell_input_vec_string.is_empty() {
            continue;
        }

        // Check the user input command
        if shell_input_vec_string[0] == "exit" && shell_input_vec_string.len() == 1 {
            
            println!("\n{}\n", "[i] Exiting NanarC2...".blue());
            std::process::exit(0);

        } else if shell_input_vec_string[0] == "help" && shell_input_vec_string.len() == 1 {

            crate::help::print_local_commands();
        
        } else if shell_input_vec_string[0] == "l" || shell_input_vec_string[0] == "listener" {

            println!("[i] Setting up the listener...")
            // TODO listeners function
        } else if shell_input_vec_string[0] == "cc" || shell_input_vec_string[0] == "create-client" {
            
            println!("[i] Creating client...");
            // TODO Create client function
        } else if (shell_input_vec_string[0] == "ls" || shell_input_vec_string[0] == "list-sessions") && shell_input_vec_string.len() == 1 {

            println!("[i] Listing Sesssions...");
            // TODO ist sessions function
        } else if (shell_input_vec_string[0] == "ll" || shell_input_vec_string[0] == "list-listeners") && shell_input_vec_string.len() == 1 {
            
            println!("[i] Listing listeners...");
            // TODO list listeners function
        } else if shell_input_vec_string[0] == "s" || shell_input_vec_string[0] == "session" {
            
            println!("[i] Session minipulation...");
            // TODO session interaction and minipulation function
        } else if shell_input_vec_string[0] == "config" {

            println!("[!] Server configurations...");
            // TODO server config function
        } else if (shell_input_vec_string[0] == "prc" || shell_input_vec_string[0] == "print-remote-commands") && shell_input_vec_string.len() == 1 {

            crate::help::print_remote_commands();
        
        } else if (shell_input_vec_string[0] == "pc" || shell_input_vec_string[0] == "print-clients") && shell_input_vec_string.len() == 1 {

            println!("[i] Printing the content of the clients.jon file");
            // TODO printing clients.json file function
        } else if shell_input_vec_string[0] == "pcf" || shell_input_vec_string[0] == "point-client-file" {

            println!("[i] Pointing to another clients file...");
            // TODO point to client file function
        } else {

            println!("{}", "[!] Error: Unknown command!".red());
            println!("{}", "[i] You can check the list of avaliable commands by running {{ help }} commmand.".blue());
        }
    }
 }