use colored::Colorize;

pub fn arguments_handler(args: Vec<String>) {

    if args.len() == 1 {
        
        println!("{}\n", crate::help::LOGO_STR.yellow());
        println!("{}\n", "[i] Starting the server...".blue());
        crate::server_shell::server_shell();

    } else if args.len() == 2 {

        // Check for {{ -h, --help }} option
        if args[1] == "-h" || args[1] == "--help" {
            crate::help::help_message();
        }
        // Check for the {{ -pc,  --print-clients }} option
        else if args[1] == "-pc" || args[1] == "--print-clients" {
            crate::configs::print_clients(r"C:\Users\muhammad\Desktop\fff.txt") // Change later
        }
        // Check for the {{ -rac, --remove-all-clients }} option
        else if args[1] == "-rac" || args[1] == "--remove-all-clients" {
            println!("[i] Removiong all clients...");
            // TODO Run removing all clients function
        }
        // Check for the {{ -gcf, --get-clients-file }} option
        else if args[1] == "-gcf" || args[1] == "--get-clients-file" {
            println!("[i] Printing the clients.json file path...");
            // TODO Run printing the clients.jon path function
        }
        // Check for the {{ -plc, --print-local-commands }} option
        else if args[1] == "-plc" || args[1] == "--print-local-commands" {
            crate::help::print_local_commands();
        }
        // Check for the {{ -prc, --print-remote-commands }} option
        else if args[1] == "-prc" || args[1] == "--print-remote-commands" {
            crate::help::print_remote_commands();
        } else {
            println!("{}", "[!] Error: Invalid option or option value!".red());
            println!("{}", "[i] you can check the help menu using the option {{-h, --help }}.".blue());
            std::process::exit(-1);
        }

    } else if args.len() == 3 {
        // Check for the {{ -rc,  --remove-client [ID] }} option
        if args[1] == "-rc" || args[1] == "--remove-client" {
            println!("[i] Remove Client...");
            // TODO Run the remove client function
        }
        // Check for the command {{ -pcf, --point-client-file [Path] }}
        else if args[1] == "-pcf" || args[1] == "--point-client-file" {
            println!("[i] Pointing to another clients file...");
            // TODO Run the poiting to another clients file function
        } else {
            println!("{}", "[!] Error: Invalid option or option value!".red());
            println!("{}", "[i] you can check the help menu using the option {{-h, --help }}.".blue());
            std::process::exit(-1);
        }
    }

}