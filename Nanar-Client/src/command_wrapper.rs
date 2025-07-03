/*
    This file is a wrapper for all the commands that can be sent from the server to the client
*/

const AVAILABLE_COMMANDS: [&str; 2] = ["pwd", "ls"]; 

// TODO
pub fn excute_server_command(server_command: &str) -> String {

    if AVAILABLE_COMMANDS.contains(&server_command) {

        if server_command == "pwd" {

            return super::fs_functions::get_current_dir();
        
        } else if server_command == "cd" {
            // TODO
        } else if server_command == "terminate" {
            // terminate the client process completely
            std::process::exit(0);

        }
    } else {

        return "This command is not avaliable in the client".to_string();
    }

    return "".to_string();
}