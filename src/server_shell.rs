use std::io::Write;
use colored::Colorize;

pub fn server_shell() {

    println!("{}\n", crate::help::LOGO_STR.yellow());
    println!("{}", "[i] Starting NanarC2 server...".blue());
    
    // Check the server configuration directory and files
    crate::configs::check_config_files();

    println!();

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

        } else if shell_input_vec_string[0] == "clear" && shell_input_vec_string.len() == 1 {

            crate::help::clear_screen()

        } else if shell_input_vec_string[0] == "help" && shell_input_vec_string.len() == 1 {

            crate::help::print_local_commands();
        
        } else if shell_input_vec_string[0] == "l" || shell_input_vec_string[0] == "listener" {

            if shell_input_vec_string.len() < 4 {

                println!("{}", format!("[-] Error: command arguments are missing or incorrect!").red());
                println!("{}", format!("[i] You can check the help message by using the command {{ help }}.").blue());
            
            } else if shell_input_vec_string.len() == 4 && (shell_input_vec_string[1] == "socket" || shell_input_vec_string[1] == "http") {

                let local_host: &str = shell_input_vec_string[2].as_str();
                let local_port: &str = shell_input_vec_string[3].as_str();

                if shell_input_vec_string[1] == "socket" {
                    // TODO
                    crate::listeners::setup_socket_listener(&local_host, &local_port);
                
                } else if shell_input_vec_string[1] == "http" {
                    // TODO
                    crate::listeners::setup_http_listener(&local_host, &local_port);
                }
            
            } else if shell_input_vec_string.len() == 5 && shell_input_vec_string[1] == "https" {

                let local_host: &str = shell_input_vec_string[2].as_str();
                let local_port: &str = shell_input_vec_string[3].as_str();
                let certificate_path: &str = shell_input_vec_string[4].as_str();
                // TODO
                crate::listeners::setup_https_listener(&local_host, &local_port, &certificate_path);

            } else if shell_input_vec_string[1] == "nanar-rev-shell" && shell_input_vec_string.len() > 4 {

                if shell_input_vec_string[2] == "socket" && shell_input_vec_string.len() == 5 {

                    let local_host: &str = shell_input_vec_string[3].as_str();
                    let local_port: &str = shell_input_vec_string[4].as_str();
                    // TDOO setup the nanar reverse shell socket type
                    crate::listeners::setup_nanar_revshell_socket(&local_host, &local_port);
                
                } else if shell_input_vec_string[2] == "http" && shell_input_vec_string.len() == 5 {

                    let local_host: &str = shell_input_vec_string[3].as_str();
                    let local_port: &str = shell_input_vec_string[4].as_str();
                    // TDOO setup the nanar reverse shell HTTP type
                    crate::listeners::setup_nanar_revshell_http(&local_host, &local_port);
                
                } else if shell_input_vec_string[2] == "https" && shell_input_vec_string.len() == 6 {

                    let local_host: &str = shell_input_vec_string[3].as_str();
                    let local_port: &str = shell_input_vec_string[4].as_str();
                    let certificate_path: &str = shell_input_vec_string[5].as_str();
                    // TDOO setup the nanar reverse shell HTTP type
                    crate::listeners::setup_nanar_revshell_https(&local_host, &local_port, &certificate_path);
                
                } else {
                    
                    println!("{}", format!("[-] Error: command arguments are missing or incorrect!").red());
                    println!("{}", format!("[i] You can check the help message by using the command {{ help }}.").blue());
                }

            } else {
      
                println!("{}", format!("[-] Error: command arguments are missing or incorrect!").red());
                println!("{}", format!("[i] You can check the help message by using the command {{ help }}.").blue());
            }

        } else if shell_input_vec_string[0] == "cc" || shell_input_vec_string[0] == "create-client" {
            
            println!("[i] Creating client...");
            // TODO Create client function
        } else if (shell_input_vec_string[0] == "ls" || shell_input_vec_string[0] == "list-sessions") && shell_input_vec_string.len() == 1 {

            println!("[i] Listing Sessions...");
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

            crate::configs::print_clients();

        } else {

            println!("{}", "[!] Error: Unknown command!".red());
            println!("{}", "[i] You can check the list of avaliable commands by running {{ help }} commmand.".blue());
        }
    }
 }