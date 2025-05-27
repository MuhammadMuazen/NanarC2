use std::io::{Read, Write};

// Messages
pub const CHECK_SERVER_MSG: &[u8] = "CHECK_SERVER_MSG".as_bytes();
pub const SERVER_IS_UP_MSG: &[u8] = "SERVER_IS_UP_MSG".as_bytes();
pub const KEY_EXCHANGE_SUCCEEDED_MSG: &[u8] = "KEY_EXCHANGE_SUCCEEDED_MSG".as_bytes();
pub const KEY_EXCHANGE_FAILED_MSG: &[u8] = "KEY_EXCHANGE_FAILED_MSG".as_bytes();
pub const CLIENT_INIT_CONN_KEY_MSG: &[u8] = "CLIENT_INIT_CONN_KEY_MSG".as_bytes();
pub const HEARTBEAT_RETRY_CONNECTION_MSG: &[u8] = "HEARTBEAT_RETRY_INIT_CONNECTION_MSG".as_bytes();
pub const HEARTBEAT_SUCCESS_RESPONSE_MSG: &[u8] = "HEARTBEAT_SUCCESS_RESPONSE_MSG".as_bytes();
pub const HEARTBEAT_NO_ACTION_MSG: &[u8] = "HEARTBEAT_NO_ACTION_MSG".as_bytes();
pub const HEARTBEAT_NO_ACTION_RESPONSE_MSG: &[u8] = "HEARTBEAT_NO_ACTION_RESPONSE_MSG".as_bytes();
pub const COMMAND_MSG: &[u8] = "COMMAND_MSG:".as_bytes();

// Reasons to call the heartbeat function
pub const MISCONNECTION_OR_COMMUNICATION: &str = "MICSONNECTION_OR_COMMUNICATION";
pub const HEARTBEAT_NO_ACTION: &str = "HEARTBEAT_NO_ACTION";

pub fn ip_to_u8_array(ip_str: &str) -> Option<[u8; 4]> {
    
    let (a, rest) = ip_str.split_once('.')?;
    let (b, rest) = rest.split_once('.')?;
    let (c, d) = rest.split_once('.')?;
    
    Some([
        a.parse().ok()?,
        b.parse().ok()?,
        c.parse().ok()?,
        d.parse().ok()?,
    ])
}


/// I AM NOT CONVINCED AT ALL WITH THE RETURN VALUE BUT LEAVE IT FOR NOW!!!
pub fn hearbeat_connection(sock_addr: std::net::SocketAddr, heartbeat_message_duration: std::time::Duration, call_reason: &str) -> bool {

    // Buffer to store the server messages in
    let mut buffer: [u8; 1024]   = [0; 1024];

    loop {
        match std::net::TcpStream::connect(&sock_addr) {
            Err(e) => {
                println!("[-] Error: Could not initilize the heartbeat connection: {}", e);
                println!("[!] Retrying Hearbeat connection");
                std::thread::sleep(heartbeat_message_duration);
            },
            Ok(mut stream) => {
                println!("[!] Entering heartbeat message communication status!");

                // If called beacuse of failing in the connection or the communication
                if call_reason == MISCONNECTION_OR_COMMUNICATION {

                    // Now init the heartbeat connection
                    match stream.write(HEARTBEAT_RETRY_CONNECTION_MSG) {
                        Ok(_) => println!("[+] Sent the message: {:?}", HEARTBEAT_RETRY_CONNECTION_MSG),
                        Err(e) => println!("[-] Error could not send the HEARTBEAT_RETRY_CONNECTION_MSG: {}", e)
                    };

                    match stream.read(&mut buffer) {
                        
                        Ok(data) => if data > 0 {
                            let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                            if server_response.to_string().as_bytes() == HEARTBEAT_SUCCESS_RESPONSE_MSG {

                                println!("[+] Heartbeat connection succeeded...Retrying to initiate the connection again");
                                true;
                            } else {
                                
                                println!("[-] Error: got message {} which is not the corrent message...Retrying!!!", 
                                server_response.to_string());
                            }
                        },
                        Err(e) => {
                            println!("[-] Error: Could not read the response to the message HEARTBEAT_RETRY_CONNECTION_MSG: {}", e);
                        }
                    };
                    
                } else if call_reason == HEARTBEAT_NO_ACTION {

                    match stream.write(HEARTBEAT_NO_ACTION_MSG) {
                        Ok(_) => println!("[+] Sent the message  HEARTBEAT_NO_ACTION_MSG"),
                        Err(e) => {
                            println!("[-] Error: Could not send the HEARTBEAT_NO_ACTION_MSG: {}", e);
                        }
                    };

                    match stream.read(&mut buffer) {

                        Ok(data) if data > 0 => {
                            let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                            if server_response.to_string().as_bytes() == HEARTBEAT_NO_ACTION_RESPONSE_MSG {

                                println!("[+] Hearbeart message exchange succeeded!!")
                            
                            } else if server_response.to_string().as_bytes().starts_with(COMMAND_MSG) {
                                // WHAT IN THE ACTUAL FUCK IS THIS FUCKING HACK?????
                                // Concurrent connection one for the heartbeat and the other for the commands
                                // MAYBE!!!
                                println!("[!] Got command from the server: {}", server_response.to_string());
                                // TODO
                            }

                            true;
                        },
                        Ok(_) => {
                            println!("[-] Could not get the data correctly from the server!!!");
                        },
                        Err(e) => {
                            println!("[-] Error: Did not get the response to the HEARTBEAT_NO_ACTION_MSG: {}", e)
                        }

                    }
                }
            }
        }
    }
}