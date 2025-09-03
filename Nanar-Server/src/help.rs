const LOGO_STR: &str = r#"
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
"#;

fn get_exe_file_name() -> String {

    let exe_path: std::path::PathBuf = std::env::current_exe().expect("Failed to get current executable path");
    let exe_name: &std::ffi::OsStr = exe_path.file_name().expect("Failed to get executable name");
    let str_exe_name: &str = exe_name.to_str().expect("Failed to convert executable name to string");

    return str_exe_name.to_owned();
}

pub fn help_message() {

    let executable_name_string: String = get_exe_file_name();

    println!(r#"{0}

    [i] Usage:

        {1} [options]
    
    [i] Options:

        -h,   --help                      Prints this help message.
        -pc,  --print-clients             Prints the contents of the json file that contains all the clients IDs.
        -rc,  --remove-client [ID]        Remove a client ID and information from the json file that contains all the clients IDs.
        -rac, --remove-all-clients        Delets the clients information json file.
        -gcf, --get-clients-file          Prints the path of the clients information json file.
        -pcf, --point-client-file [Path]  Make the server point to another clients json file.
        -plc, --print-local-commands      Prints all the avaliable commands that we can run once we start the server.
        -prc, --print-remote-commands     Prints all the avaliable commands that can be sent to the client.

    [i] To start the server simply run the command:

        {1}
"#, LOGO_STR, executable_name_string);
}

pub fn print_avaliable_local_commands() {

    println!(r#"    
[i] These commands can be run inside the server command line interface:

    [1] l, listen [options] --> Make the server listen in a way specifed by the options:

        1) socket [ip] [port]
        2) http [port]
        3) https [port] [certificate-path]

    [2] cc, create-client [options] --> Compiles a new client with new ID and communication password and add 
        them to the clients.json file (Requires cargo to be installed on your machine):
 
        1) -s, --socket [ip/domain] [port]  Specify the socket the client will connect to.
        2) -http [ip/domain] [port]         Specify the HTTP server the client will connect to.
        3) -https [ip/domain] [port]        Specify the HTTPS server the client will connect to.     
        4) -cd, --client-directory [path]   Specify the client directory path which will be compiled.

    [2] gcid, generate-client-id --> Generate a client id and add it to the clients.json file

    [3] 
      
"#);
}