use nanar_client::{connection_handler::heartbeat, *};

#[tokio::main]
async fn main() {

    // Connection vars
    const SERVER_ADDR: &str = "127.0.0.1";
    const SERVER_PORT: &str = "9999";

    // Var to check if hearbeat is on
    // TODO
    let mut heart_beat_on: bool = true;

    loop {

        // First Initilize the connection
        match connection_handler::init_conn_with_server(SERVER_ADDR, SERVER_PORT, messages::INIT_CONNECTION_PASS).await {
            Ok(_) => {

                println!("[+] Connection Initilization succeeded from the main");
                println!("[+] Now the client will run the listening for the commands & Run the heartbeat communications");

                /*
                    The client in the idle mood run both the heartbeat function and the command listing function.
                */
                let heartbeat_task: tokio::task::JoinHandle<Result<(), std::io::Error>> = tokio::spawn(
                    connection_handler::heartbeat(
                        connection_handler::convert_ip_port_to_sockaddr(SERVER_ADDR, SERVER_PORT),
                        messages::HEARTBEAT_NO_ACTION)
                );
                    
                let commands_listener_task: tokio::task::JoinHandle<Result<(), std::io::Error>> = tokio::spawn(
                    connection_handler::commands_communication_handler(SERVER_ADDR, SERVER_PORT,
                         messages::COMMANMD_COMMUNICATION_SECRET, messages::NONCE));
                
                // Start two tasks the first one is the heartbeat and the second one is the one listening for commands
                let _ = tokio::join!(heartbeat_task, commands_listener_task);
            },
            Err(e) => {

                println!("[-] Could not initilize the connection from the main: {}", e);
                println!("[+] Now the server will go into the heartbeat state");

                let heartbeat_status: Result<(), std::io::Error> = heartbeat(
                    connection_handler::convert_ip_port_to_sockaddr(SERVER_ADDR, SERVER_PORT),
                     messages::MISCONNECTION_OR_MISCOMMUNICATION).await;

                if heartbeat_status.is_err() {

                    println!("[!] Something went wrong with the hearbeat message!");
                    break;
                }
            }
        }
    }
}